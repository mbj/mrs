pub mod capacityreservation {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-capacityreservation-capacityallocation.html
    pub struct CapacityAllocation_ {
        pub allocation_type: Option<crate::value::ExpString>,
        pub count: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_CapacityReservation_CapacityAllocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::CapacityReservation.CapacityAllocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_CapacityReservation_CapacityAllocation as CapacityAllocation;
    impl crate::value::ToValue for CapacityAllocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allocation_type {
                properties.insert(
                    "AllocationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.count {
                properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-capacityreservation-commitmentinfo.html
    pub struct CommitmentInfo_ {
        pub commitment_end_date: Option<crate::value::ExpString>,
        pub committed_instance_count: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_CapacityReservation_CommitmentInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::CapacityReservation.CommitmentInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_CapacityReservation_CommitmentInfo as CommitmentInfo;
    impl crate::value::ToValue for CommitmentInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.commitment_end_date {
                properties.insert(
                    "CommitmentEndDate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.committed_instance_count {
                properties.insert(
                    "CommittedInstanceCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-capacityreservation-tagspecification.html
    pub struct TagSpecification_ {
        pub resource_type: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_CapacityReservation_TagSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::CapacityReservation.TagSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_CapacityReservation_TagSpecification as TagSpecification;
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
pub mod capacityreservationfleet {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-capacityreservationfleet-instancetypespecification.html
    pub struct InstanceTypeSpecification_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub availability_zone_id: Option<crate::value::ExpString>,
        pub ebs_optimized: Option<crate::value::ExpBool>,
        pub instance_platform: Option<crate::value::ExpString>,
        pub instance_type: Option<crate::value::ExpString>,
        pub priority: Option<i32>,
        pub weight: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_CapacityReservationFleet_InstanceTypeSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::CapacityReservationFleet.InstanceTypeSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_CapacityReservationFleet_InstanceTypeSpecification as InstanceTypeSpecification;
    impl crate::value::ToValue for InstanceTypeSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
            if let Some(ref value) = self.ebs_optimized {
                properties.insert(
                    "EbsOptimized".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_platform {
                properties.insert(
                    "InstancePlatform".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_type {
                properties.insert(
                    "InstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.priority {
                properties.insert(
                    "Priority".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weight {
                properties.insert("Weight".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-capacityreservationfleet-tagspecification.html
    pub struct TagSpecification_ {
        pub resource_type: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_CapacityReservationFleet_TagSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::CapacityReservationFleet.TagSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_CapacityReservationFleet_TagSpecification as TagSpecification;
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
pub mod clientvpnendpoint {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-clientvpnendpoint-certificateauthenticationrequest.html
    pub struct CertificateAuthenticationRequest_ {
        pub client_root_certificate_chain_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_ClientVpnEndpoint_CertificateAuthenticationRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::ClientVpnEndpoint.CertificateAuthenticationRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_ClientVpnEndpoint_CertificateAuthenticationRequest as CertificateAuthenticationRequest;
    impl crate::value::ToValue for CertificateAuthenticationRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClientRootCertificateChainArn".to_string(),
                crate::value::ToValue::to_value(&self.client_root_certificate_chain_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-clientvpnendpoint-clientauthenticationrequest.html
    pub struct ClientAuthenticationRequest_ {
        pub active_directory: Option<Box<DirectoryServiceAuthenticationRequest_>>,
        pub federated_authentication: Option<Box<FederatedAuthenticationRequest_>>,
        pub mutual_authentication: Option<Box<CertificateAuthenticationRequest_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_ClientVpnEndpoint_ClientAuthenticationRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::ClientVpnEndpoint.ClientAuthenticationRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_ClientVpnEndpoint_ClientAuthenticationRequest as ClientAuthenticationRequest;
    impl crate::value::ToValue for ClientAuthenticationRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.active_directory {
                properties.insert(
                    "ActiveDirectory".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.federated_authentication {
                properties.insert(
                    "FederatedAuthentication".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mutual_authentication {
                properties.insert(
                    "MutualAuthentication".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-clientvpnendpoint-clientconnectoptions.html
    pub struct ClientConnectOptions_ {
        pub enabled: crate::value::ExpBool,
        pub lambda_function_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_ClientVpnEndpoint_ClientConnectOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::ClientVpnEndpoint.ClientConnectOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_ClientVpnEndpoint_ClientConnectOptions as ClientConnectOptions;
    impl crate::value::ToValue for ClientConnectOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.lambda_function_arn {
                properties.insert(
                    "LambdaFunctionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-clientvpnendpoint-clientloginbanneroptions.html
    pub struct ClientLoginBannerOptions_ {
        pub banner_text: Option<crate::value::ExpString>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_ClientVpnEndpoint_ClientLoginBannerOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::ClientVpnEndpoint.ClientLoginBannerOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_ClientVpnEndpoint_ClientLoginBannerOptions as ClientLoginBannerOptions;
    impl crate::value::ToValue for ClientLoginBannerOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.banner_text {
                properties.insert(
                    "BannerText".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-clientvpnendpoint-clientrouteenforcementoptions.html
    pub struct ClientRouteEnforcementOptions_ {
        pub enforced: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_ClientVpnEndpoint_ClientRouteEnforcementOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::ClientVpnEndpoint.ClientRouteEnforcementOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_ClientVpnEndpoint_ClientRouteEnforcementOptions as ClientRouteEnforcementOptions;
    impl crate::value::ToValue for ClientRouteEnforcementOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enforced {
                properties.insert(
                    "Enforced".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-clientvpnendpoint-connectionlogoptions.html
    pub struct ConnectionLogOptions_ {
        pub cloudwatch_log_group: Option<crate::value::ExpString>,
        pub cloudwatch_log_stream: Option<crate::value::ExpString>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_ClientVpnEndpoint_ConnectionLogOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::ClientVpnEndpoint.ConnectionLogOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_ClientVpnEndpoint_ConnectionLogOptions as ConnectionLogOptions;
    impl crate::value::ToValue for ConnectionLogOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloudwatch_log_group {
                properties.insert(
                    "CloudwatchLogGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloudwatch_log_stream {
                properties.insert(
                    "CloudwatchLogStream".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-clientvpnendpoint-directoryserviceauthenticationrequest.html
    pub struct DirectoryServiceAuthenticationRequest_ {
        pub directory_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_ClientVpnEndpoint_DirectoryServiceAuthenticationRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::ClientVpnEndpoint.DirectoryServiceAuthenticationRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_ClientVpnEndpoint_DirectoryServiceAuthenticationRequest as DirectoryServiceAuthenticationRequest;
    impl crate::value::ToValue for DirectoryServiceAuthenticationRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DirectoryId".to_string(),
                crate::value::ToValue::to_value(&self.directory_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-clientvpnendpoint-federatedauthenticationrequest.html
    pub struct FederatedAuthenticationRequest_ {
        pub saml_provider_arn: crate::value::ExpString,
        pub self_service_saml_provider_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_ClientVpnEndpoint_FederatedAuthenticationRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::ClientVpnEndpoint.FederatedAuthenticationRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_ClientVpnEndpoint_FederatedAuthenticationRequest as FederatedAuthenticationRequest;
    impl crate::value::ToValue for FederatedAuthenticationRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SAMLProviderArn".to_string(),
                crate::value::ToValue::to_value(&self.saml_provider_arn),
            );
            if let Some(ref value) = self.self_service_saml_provider_arn {
                properties.insert(
                    "SelfServiceSAMLProviderArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-clientvpnendpoint-tagspecification.html
    pub struct TagSpecification_ {
        pub resource_type: crate::value::ExpString,
        pub tags: Vec<crate::Tag_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_ClientVpnEndpoint_TagSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::ClientVpnEndpoint.TagSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_ClientVpnEndpoint_TagSpecification as TagSpecification;
    impl crate::value::ToValue for TagSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceType".to_string(),
                crate::value::ToValue::to_value(&self.resource_type),
            );
            properties.insert(
                "Tags".to_string(),
                crate::value::ToValue::to_value(&self.tags),
            );
            properties.into()
        }
    }
}
pub mod ec2fleet {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-acceleratorcountrequest.html
    pub struct AcceleratorCountRequest_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_AcceleratorCountRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.AcceleratorCountRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_AcceleratorCountRequest as AcceleratorCountRequest;
    impl crate::value::ToValue for AcceleratorCountRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-acceleratortotalmemorymibrequest.html
    pub struct AcceleratorTotalMemoryMiBRequest_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_AcceleratorTotalMemoryMiBRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.AcceleratorTotalMemoryMiBRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_AcceleratorTotalMemoryMiBRequest as AcceleratorTotalMemoryMiBRequest;
    impl crate::value::ToValue for AcceleratorTotalMemoryMiBRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-baselineebsbandwidthmbpsrequest.html
    pub struct BaselineEbsBandwidthMbpsRequest_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_BaselineEbsBandwidthMbpsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.BaselineEbsBandwidthMbpsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_BaselineEbsBandwidthMbpsRequest as BaselineEbsBandwidthMbpsRequest;
    impl crate::value::ToValue for BaselineEbsBandwidthMbpsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-baselineperformancefactorsrequest.html
    pub struct BaselinePerformanceFactorsRequest_ {
        pub cpu: Option<Box<CpuPerformanceFactorRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_BaselinePerformanceFactorsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.BaselinePerformanceFactorsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_BaselinePerformanceFactorsRequest as BaselinePerformanceFactorsRequest;
    impl crate::value::ToValue for BaselinePerformanceFactorsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cpu {
                properties.insert("Cpu".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-blockdevicemapping.html
    pub struct BlockDeviceMapping_ {
        pub device_name: Option<crate::value::ExpString>,
        pub ebs: Option<Box<EbsBlockDevice_>>,
        pub no_device: Option<crate::value::ExpString>,
        pub virtual_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_BlockDeviceMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.BlockDeviceMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_BlockDeviceMapping as BlockDeviceMapping;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-capacityrebalance.html
    pub struct CapacityRebalance_ {
        pub replacement_strategy: Option<crate::value::ExpString>,
        pub termination_delay: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_CapacityRebalance {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.CapacityRebalance"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_CapacityRebalance as CapacityRebalance;
    impl crate::value::ToValue for CapacityRebalance_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.replacement_strategy {
                properties.insert(
                    "ReplacementStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.termination_delay {
                properties.insert(
                    "TerminationDelay".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-capacityreservationoptionsrequest.html
    pub struct CapacityReservationOptionsRequest_ {
        pub usage_strategy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_CapacityReservationOptionsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.CapacityReservationOptionsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_CapacityReservationOptionsRequest as CapacityReservationOptionsRequest;
    impl crate::value::ToValue for CapacityReservationOptionsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.usage_strategy {
                properties.insert(
                    "UsageStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-cpuperformancefactorrequest.html
    pub struct CpuPerformanceFactorRequest_ {
        pub references: Option<Vec<PerformanceFactorReferenceRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_CpuPerformanceFactorRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.CpuPerformanceFactorRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_CpuPerformanceFactorRequest as CpuPerformanceFactorRequest;
    impl crate::value::ToValue for CpuPerformanceFactorRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.references {
                properties.insert(
                    "References".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-ebsblockdevice.html
    pub struct EbsBlockDevice_ {
        pub delete_on_termination: Option<crate::value::ExpBool>,
        pub encrypted: Option<crate::value::ExpBool>,
        pub iops: Option<i32>,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub snapshot_id: Option<crate::value::ExpString>,
        pub volume_size: Option<i32>,
        pub volume_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_EbsBlockDevice {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.EbsBlockDevice"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_EbsBlockDevice as EbsBlockDevice;
    impl crate::value::ToValue for EbsBlockDevice_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delete_on_termination {
                properties.insert(
                    "DeleteOnTermination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
            if let Some(ref value) = self.snapshot_id {
                properties.insert(
                    "SnapshotId".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-fleetlaunchtemplateconfigrequest.html
    pub struct FleetLaunchTemplateConfigRequest_ {
        pub launch_template_specification: Option<Box<FleetLaunchTemplateSpecificationRequest_>>,
        pub overrides: Option<Vec<FleetLaunchTemplateOverridesRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_FleetLaunchTemplateConfigRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.FleetLaunchTemplateConfigRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_FleetLaunchTemplateConfigRequest as FleetLaunchTemplateConfigRequest;
    impl crate::value::ToValue for FleetLaunchTemplateConfigRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.launch_template_specification {
                properties.insert(
                    "LaunchTemplateSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.overrides {
                properties.insert(
                    "Overrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-fleetlaunchtemplateoverridesrequest.html
    pub struct FleetLaunchTemplateOverridesRequest_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub availability_zone_id: Option<crate::value::ExpString>,
        pub block_device_mappings: Option<Vec<BlockDeviceMapping_>>,
        pub instance_requirements: Option<Box<InstanceRequirementsRequest_>>,
        pub instance_type: Option<crate::value::ExpString>,
        pub max_price: Option<crate::value::ExpString>,
        pub placement: Option<Box<Placement_>>,
        pub priority: Option<f64>,
        pub subnet_id: Option<crate::value::ExpString>,
        pub weighted_capacity: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_FleetLaunchTemplateOverridesRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.FleetLaunchTemplateOverridesRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_FleetLaunchTemplateOverridesRequest as FleetLaunchTemplateOverridesRequest;
    impl crate::value::ToValue for FleetLaunchTemplateOverridesRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
            if let Some(ref value) = self.block_device_mappings {
                properties.insert(
                    "BlockDeviceMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_requirements {
                properties.insert(
                    "InstanceRequirements".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_type {
                properties.insert(
                    "InstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_price {
                properties.insert(
                    "MaxPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.placement {
                properties.insert(
                    "Placement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.priority {
                properties.insert(
                    "Priority".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnet_id {
                properties.insert(
                    "SubnetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weighted_capacity {
                properties.insert(
                    "WeightedCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-fleetlaunchtemplatespecificationrequest.html
    pub struct FleetLaunchTemplateSpecificationRequest_ {
        pub launch_template_id: Option<crate::value::ExpString>,
        pub launch_template_name: Option<crate::value::ExpString>,
        pub version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_FleetLaunchTemplateSpecificationRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.FleetLaunchTemplateSpecificationRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_FleetLaunchTemplateSpecificationRequest as FleetLaunchTemplateSpecificationRequest;
    impl crate::value::ToValue for FleetLaunchTemplateSpecificationRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.launch_template_id {
                properties.insert(
                    "LaunchTemplateId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.launch_template_name {
                properties.insert(
                    "LaunchTemplateName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Version".to_string(),
                crate::value::ToValue::to_value(&self.version),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-instancerequirementsrequest.html
    pub struct InstanceRequirementsRequest_ {
        pub accelerator_count: Option<Box<AcceleratorCountRequest_>>,
        pub accelerator_manufacturers: Option<Vec<crate::value::ExpString>>,
        pub accelerator_names: Option<Vec<crate::value::ExpString>>,
        pub accelerator_total_memory_mi_b: Option<Box<AcceleratorTotalMemoryMiBRequest_>>,
        pub accelerator_types: Option<Vec<crate::value::ExpString>>,
        pub allowed_instance_types: Option<Vec<crate::value::ExpString>>,
        pub bare_metal: Option<crate::value::ExpString>,
        pub baseline_ebs_bandwidth_mbps: Option<Box<BaselineEbsBandwidthMbpsRequest_>>,
        pub baseline_performance_factors: Option<Box<BaselinePerformanceFactorsRequest_>>,
        pub burstable_performance: Option<crate::value::ExpString>,
        pub cpu_manufacturers: Option<Vec<crate::value::ExpString>>,
        pub excluded_instance_types: Option<Vec<crate::value::ExpString>>,
        pub instance_generations: Option<Vec<crate::value::ExpString>>,
        pub local_storage: Option<crate::value::ExpString>,
        pub local_storage_types: Option<Vec<crate::value::ExpString>>,
        pub max_spot_price_as_percentage_of_optimal_on_demand_price: Option<i32>,
        pub memory_gi_b_per_v_cpu: Option<Box<MemoryGiBPerVCpuRequest_>>,
        pub memory_mi_b: Option<Box<MemoryMiBRequest_>>,
        pub network_bandwidth_gbps: Option<Box<NetworkBandwidthGbpsRequest_>>,
        pub network_interface_count: Option<Box<NetworkInterfaceCountRequest_>>,
        pub on_demand_max_price_percentage_over_lowest_price: Option<i32>,
        pub require_encryption_in_transit: Option<crate::value::ExpBool>,
        pub require_hibernate_support: Option<crate::value::ExpBool>,
        pub spot_max_price_percentage_over_lowest_price: Option<i32>,
        pub total_local_storage_gb: Option<Box<TotalLocalStorageGBRequest_>>,
        pub v_cpu_count: Option<Box<VCpuCountRangeRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_InstanceRequirementsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.InstanceRequirementsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_InstanceRequirementsRequest as InstanceRequirementsRequest;
    impl crate::value::ToValue for InstanceRequirementsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.accelerator_count {
                properties.insert(
                    "AcceleratorCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.accelerator_manufacturers {
                properties.insert(
                    "AcceleratorManufacturers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.accelerator_names {
                properties.insert(
                    "AcceleratorNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.accelerator_total_memory_mi_b {
                properties.insert(
                    "AcceleratorTotalMemoryMiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.accelerator_types {
                properties.insert(
                    "AcceleratorTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allowed_instance_types {
                properties.insert(
                    "AllowedInstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bare_metal {
                properties.insert(
                    "BareMetal".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.baseline_ebs_bandwidth_mbps {
                properties.insert(
                    "BaselineEbsBandwidthMbps".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.baseline_performance_factors {
                properties.insert(
                    "BaselinePerformanceFactors".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.burstable_performance {
                properties.insert(
                    "BurstablePerformance".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cpu_manufacturers {
                properties.insert(
                    "CpuManufacturers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.excluded_instance_types {
                properties.insert(
                    "ExcludedInstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_generations {
                properties.insert(
                    "InstanceGenerations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.local_storage {
                properties.insert(
                    "LocalStorage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.local_storage_types {
                properties.insert(
                    "LocalStorageTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_spot_price_as_percentage_of_optimal_on_demand_price {
                properties.insert(
                    "MaxSpotPriceAsPercentageOfOptimalOnDemandPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory_gi_b_per_v_cpu {
                properties.insert(
                    "MemoryGiBPerVCpu".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory_mi_b {
                properties.insert(
                    "MemoryMiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_bandwidth_gbps {
                properties.insert(
                    "NetworkBandwidthGbps".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_interface_count {
                properties.insert(
                    "NetworkInterfaceCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_demand_max_price_percentage_over_lowest_price {
                properties.insert(
                    "OnDemandMaxPricePercentageOverLowestPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_encryption_in_transit {
                properties.insert(
                    "RequireEncryptionInTransit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_hibernate_support {
                properties.insert(
                    "RequireHibernateSupport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_max_price_percentage_over_lowest_price {
                properties.insert(
                    "SpotMaxPricePercentageOverLowestPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.total_local_storage_gb {
                properties.insert(
                    "TotalLocalStorageGB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.v_cpu_count {
                properties.insert(
                    "VCpuCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-maintenancestrategies.html
    pub struct MaintenanceStrategies_ {
        pub capacity_rebalance: Option<Box<CapacityRebalance_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_MaintenanceStrategies {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.MaintenanceStrategies"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_MaintenanceStrategies as MaintenanceStrategies;
    impl crate::value::ToValue for MaintenanceStrategies_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity_rebalance {
                properties.insert(
                    "CapacityRebalance".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-memorygibpervcpurequest.html
    pub struct MemoryGiBPerVCpuRequest_ {
        pub max: Option<f64>,
        pub min: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_MemoryGiBPerVCpuRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.MemoryGiBPerVCpuRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_MemoryGiBPerVCpuRequest as MemoryGiBPerVCpuRequest;
    impl crate::value::ToValue for MemoryGiBPerVCpuRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-memorymibrequest.html
    pub struct MemoryMiBRequest_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_MemoryMiBRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.MemoryMiBRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_MemoryMiBRequest as MemoryMiBRequest;
    impl crate::value::ToValue for MemoryMiBRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-networkbandwidthgbpsrequest.html
    pub struct NetworkBandwidthGbpsRequest_ {
        pub max: Option<f64>,
        pub min: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_NetworkBandwidthGbpsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.NetworkBandwidthGbpsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_NetworkBandwidthGbpsRequest as NetworkBandwidthGbpsRequest;
    impl crate::value::ToValue for NetworkBandwidthGbpsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-networkinterfacecountrequest.html
    pub struct NetworkInterfaceCountRequest_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_NetworkInterfaceCountRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.NetworkInterfaceCountRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_NetworkInterfaceCountRequest as NetworkInterfaceCountRequest;
    impl crate::value::ToValue for NetworkInterfaceCountRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-ondemandoptionsrequest.html
    pub struct OnDemandOptionsRequest_ {
        pub allocation_strategy: Option<crate::value::ExpString>,
        pub capacity_reservation_options: Option<Box<CapacityReservationOptionsRequest_>>,
        pub max_total_price: Option<crate::value::ExpString>,
        pub min_target_capacity: Option<i32>,
        pub single_availability_zone: Option<crate::value::ExpBool>,
        pub single_instance_type: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_OnDemandOptionsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.OnDemandOptionsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_OnDemandOptionsRequest as OnDemandOptionsRequest;
    impl crate::value::ToValue for OnDemandOptionsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allocation_strategy {
                properties.insert(
                    "AllocationStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.capacity_reservation_options {
                properties.insert(
                    "CapacityReservationOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_total_price {
                properties.insert(
                    "MaxTotalPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_target_capacity {
                properties.insert(
                    "MinTargetCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.single_availability_zone {
                properties.insert(
                    "SingleAvailabilityZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.single_instance_type {
                properties.insert(
                    "SingleInstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-performancefactorreferencerequest.html
    pub struct PerformanceFactorReferenceRequest_ {
        pub instance_family: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_PerformanceFactorReferenceRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.PerformanceFactorReferenceRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_PerformanceFactorReferenceRequest as PerformanceFactorReferenceRequest;
    impl crate::value::ToValue for PerformanceFactorReferenceRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instance_family {
                properties.insert(
                    "InstanceFamily".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-placement.html
    pub struct Placement_ {
        pub affinity: Option<crate::value::ExpString>,
        pub availability_zone: Option<crate::value::ExpString>,
        pub group_name: Option<crate::value::ExpString>,
        pub host_id: Option<crate::value::ExpString>,
        pub host_resource_group_arn: Option<crate::value::ExpString>,
        pub partition_number: Option<i32>,
        pub spread_domain: Option<crate::value::ExpString>,
        pub tenancy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_Placement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.Placement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_Placement as Placement;
    impl crate::value::ToValue for Placement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.affinity {
                properties.insert(
                    "Affinity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.availability_zone {
                properties.insert(
                    "AvailabilityZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.group_name {
                properties.insert(
                    "GroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.host_id {
                properties.insert("HostId".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.host_resource_group_arn {
                properties.insert(
                    "HostResourceGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.partition_number {
                properties.insert(
                    "PartitionNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spread_domain {
                properties.insert(
                    "SpreadDomain".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-spotoptionsrequest.html
    pub struct SpotOptionsRequest_ {
        pub allocation_strategy: Option<crate::value::ExpString>,
        pub instance_interruption_behavior: Option<crate::value::ExpString>,
        pub instance_pools_to_use_count: Option<i32>,
        pub maintenance_strategies: Option<Box<MaintenanceStrategies_>>,
        pub max_total_price: Option<crate::value::ExpString>,
        pub min_target_capacity: Option<i32>,
        pub single_availability_zone: Option<crate::value::ExpBool>,
        pub single_instance_type: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_SpotOptionsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.SpotOptionsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_SpotOptionsRequest as SpotOptionsRequest;
    impl crate::value::ToValue for SpotOptionsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allocation_strategy {
                properties.insert(
                    "AllocationStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_interruption_behavior {
                properties.insert(
                    "InstanceInterruptionBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_pools_to_use_count {
                properties.insert(
                    "InstancePoolsToUseCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maintenance_strategies {
                properties.insert(
                    "MaintenanceStrategies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_total_price {
                properties.insert(
                    "MaxTotalPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_target_capacity {
                properties.insert(
                    "MinTargetCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.single_availability_zone {
                properties.insert(
                    "SingleAvailabilityZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.single_instance_type {
                properties.insert(
                    "SingleInstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-tagspecification.html
    pub struct TagSpecification_ {
        pub resource_type: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_TagSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.TagSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_TagSpecification as TagSpecification;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-targetcapacityspecificationrequest.html
    pub struct TargetCapacitySpecificationRequest_ {
        pub default_target_capacity_type: Option<crate::value::ExpString>,
        pub on_demand_target_capacity: Option<i32>,
        pub spot_target_capacity: Option<i32>,
        pub target_capacity_unit_type: Option<crate::value::ExpString>,
        pub total_target_capacity: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_TargetCapacitySpecificationRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.TargetCapacitySpecificationRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_TargetCapacitySpecificationRequest as TargetCapacitySpecificationRequest;
    impl crate::value::ToValue for TargetCapacitySpecificationRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_target_capacity_type {
                properties.insert(
                    "DefaultTargetCapacityType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_demand_target_capacity {
                properties.insert(
                    "OnDemandTargetCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_target_capacity {
                properties.insert(
                    "SpotTargetCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_capacity_unit_type {
                properties.insert(
                    "TargetCapacityUnitType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TotalTargetCapacity".to_string(),
                crate::value::ToValue::to_value(&self.total_target_capacity),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-totallocalstoragegbrequest.html
    pub struct TotalLocalStorageGBRequest_ {
        pub max: Option<f64>,
        pub min: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_TotalLocalStorageGBRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.TotalLocalStorageGBRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_TotalLocalStorageGBRequest as TotalLocalStorageGBRequest;
    impl crate::value::ToValue for TotalLocalStorageGBRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ec2fleet-vcpucountrangerequest.html
    pub struct VCpuCountRangeRequest_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_EC2Fleet_VCpuCountRangeRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::EC2Fleet.VCpuCountRangeRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_EC2Fleet_VCpuCountRangeRequest as VCpuCountRangeRequest;
    impl crate::value::ToValue for VCpuCountRangeRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod flowlog {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-flowlog-destinationoptions.html
    pub struct DestinationOptions_ {
        pub file_format: crate::value::ExpString,
        pub hive_compatible_partitions: crate::value::ExpBool,
        pub per_hour_partition: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_FlowLog_DestinationOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::FlowLog.DestinationOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_FlowLog_DestinationOptions as DestinationOptions;
    impl crate::value::ToValue for DestinationOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FileFormat".to_string(),
                crate::value::ToValue::to_value(&self.file_format),
            );
            properties.insert(
                "HiveCompatiblePartitions".to_string(),
                crate::value::ToValue::to_value(&self.hive_compatible_partitions),
            );
            properties.insert(
                "PerHourPartition".to_string(),
                crate::value::ToValue::to_value(&self.per_hour_partition),
            );
            properties.into()
        }
    }
}
pub mod ipam {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ipam-ipamoperatingregion.html
    pub struct IpamOperatingRegion_ {
        pub region_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_IPAM_IpamOperatingRegion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::IPAM.IpamOperatingRegion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_IPAM_IpamOperatingRegion as IpamOperatingRegion;
    impl crate::value::ToValue for IpamOperatingRegion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RegionName".to_string(),
                crate::value::ToValue::to_value(&self.region_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ipam-ipamorganizationalunitexclusion.html
    pub struct IpamOrganizationalUnitExclusion_ {
        pub organizations_entity_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_IPAM_IpamOrganizationalUnitExclusion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::IPAM.IpamOrganizationalUnitExclusion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_IPAM_IpamOrganizationalUnitExclusion as IpamOrganizationalUnitExclusion;
    impl crate::value::ToValue for IpamOrganizationalUnitExclusion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OrganizationsEntityPath".to_string(),
                crate::value::ToValue::to_value(&self.organizations_entity_path),
            );
            properties.into()
        }
    }
}
pub mod ipampool {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ipampool-provisionedcidr.html
    pub struct ProvisionedCidr_ {
        pub cidr: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_IPAMPool_ProvisionedCidr {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::IPAMPool.ProvisionedCidr"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_IPAMPool_ProvisionedCidr as ProvisionedCidr;
    impl crate::value::ToValue for ProvisionedCidr_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Cidr".to_string(),
                crate::value::ToValue::to_value(&self.cidr),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ipampool-sourceresource.html
    pub struct SourceResource_ {
        pub resource_id: crate::value::ExpString,
        pub resource_owner: crate::value::ExpString,
        pub resource_region: crate::value::ExpString,
        pub resource_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_IPAMPool_SourceResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::IPAMPool.SourceResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_IPAMPool_SourceResource as SourceResource;
    impl crate::value::ToValue for SourceResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceId".to_string(),
                crate::value::ToValue::to_value(&self.resource_id),
            );
            properties.insert(
                "ResourceOwner".to_string(),
                crate::value::ToValue::to_value(&self.resource_owner),
            );
            properties.insert(
                "ResourceRegion".to_string(),
                crate::value::ToValue::to_value(&self.resource_region),
            );
            properties.insert(
                "ResourceType".to_string(),
                crate::value::ToValue::to_value(&self.resource_type),
            );
            properties.into()
        }
    }
}
pub mod ipamresourcediscovery {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ipamresourcediscovery-ipamoperatingregion.html
    pub struct IpamOperatingRegion_ {
        pub region_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_IPAMResourceDiscovery_IpamOperatingRegion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::IPAMResourceDiscovery.IpamOperatingRegion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_IPAMResourceDiscovery_IpamOperatingRegion as IpamOperatingRegion;
    impl crate::value::ToValue for IpamOperatingRegion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RegionName".to_string(),
                crate::value::ToValue::to_value(&self.region_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ipamresourcediscovery-ipamresourcediscoveryorganizationalunitexclusion.html
    pub struct IpamResourceDiscoveryOrganizationalUnitExclusion_ {
        pub organizations_entity_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_IPAMResourceDiscovery_IpamResourceDiscoveryOrganizationalUnitExclusion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::IPAMResourceDiscovery.IpamResourceDiscoveryOrganizationalUnitExclusion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_IPAMResourceDiscovery_IpamResourceDiscoveryOrganizationalUnitExclusion as IpamResourceDiscoveryOrganizationalUnitExclusion;
    impl crate::value::ToValue for IpamResourceDiscoveryOrganizationalUnitExclusion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OrganizationsEntityPath".to_string(),
                crate::value::ToValue::to_value(&self.organizations_entity_path),
            );
            properties.into()
        }
    }
}
pub mod ipamscope {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-ipamscope-ipamscopeexternalauthorityconfiguration.html
    pub struct IpamScopeExternalAuthorityConfiguration_ {
        pub external_resource_identifier: crate::value::ExpString,
        pub ipam_scope_external_authority_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_IPAMScope_IpamScopeExternalAuthorityConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::IPAMScope.IpamScopeExternalAuthorityConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_IPAMScope_IpamScopeExternalAuthorityConfiguration as IpamScopeExternalAuthorityConfiguration;
    impl crate::value::ToValue for IpamScopeExternalAuthorityConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ExternalResourceIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.external_resource_identifier),
            );
            properties.insert(
                "IpamScopeExternalAuthorityType".to_string(),
                crate::value::ToValue::to_value(&self.ipam_scope_external_authority_type),
            );
            properties.into()
        }
    }
}
pub mod instance {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-associationparameter.html
    pub struct AssociationParameter_ {
        pub key: crate::value::ExpString,
        pub value: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_AssociationParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.AssociationParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_AssociationParameter as AssociationParameter;
    impl crate::value::ToValue for AssociationParameter_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-blockdevicemapping.html
    pub struct BlockDeviceMapping_ {
        pub device_name: crate::value::ExpString,
        pub ebs: Option<Box<Ebs_>>,
        pub no_device: Option<serde_json::Value>,
        pub virtual_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_BlockDeviceMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.BlockDeviceMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_BlockDeviceMapping as BlockDeviceMapping;
    impl crate::value::ToValue for BlockDeviceMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DeviceName".to_string(),
                crate::value::ToValue::to_value(&self.device_name),
            );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-cpuoptions.html
    pub struct CpuOptions_ {
        pub core_count: Option<i32>,
        pub threads_per_core: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_CpuOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.CpuOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_CpuOptions as CpuOptions;
    impl crate::value::ToValue for CpuOptions_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-creditspecification.html
    pub struct CreditSpecification_ {
        pub cpu_credits: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_CreditSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.CreditSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_CreditSpecification as CreditSpecification;
    impl crate::value::ToValue for CreditSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cpu_credits {
                properties.insert(
                    "CPUCredits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-ebs.html
    pub struct Ebs_ {
        pub delete_on_termination: Option<crate::value::ExpBool>,
        pub encrypted: Option<crate::value::ExpBool>,
        pub iops: Option<i32>,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub snapshot_id: Option<crate::value::ExpString>,
        pub volume_size: Option<i32>,
        pub volume_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_Ebs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.Ebs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_Ebs as Ebs;
    impl crate::value::ToValue for Ebs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delete_on_termination {
                properties.insert(
                    "DeleteOnTermination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
            if let Some(ref value) = self.snapshot_id {
                properties.insert(
                    "SnapshotId".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-elasticgpuspecification.html
    pub struct ElasticGpuSpecification_ {
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_ElasticGpuSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.ElasticGpuSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_ElasticGpuSpecification as ElasticGpuSpecification;
    impl crate::value::ToValue for ElasticGpuSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-elasticinferenceaccelerator.html
    pub struct ElasticInferenceAccelerator_ {
        pub count: Option<i32>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_ElasticInferenceAccelerator {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.ElasticInferenceAccelerator"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_ElasticInferenceAccelerator as ElasticInferenceAccelerator;
    impl crate::value::ToValue for ElasticInferenceAccelerator_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.count {
                properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-enasrdspecification.html
    pub struct EnaSrdSpecification_ {
        pub ena_srd_enabled: Option<crate::value::ExpBool>,
        pub ena_srd_udp_specification: Option<Box<EnaSrdUdpSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_EnaSrdSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.EnaSrdSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_EnaSrdSpecification as EnaSrdSpecification;
    impl crate::value::ToValue for EnaSrdSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ena_srd_enabled {
                properties.insert(
                    "EnaSrdEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ena_srd_udp_specification {
                properties.insert(
                    "EnaSrdUdpSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-enasrdudpspecification.html
    pub struct EnaSrdUdpSpecification_ {
        pub ena_srd_udp_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_EnaSrdUdpSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.EnaSrdUdpSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_EnaSrdUdpSpecification as EnaSrdUdpSpecification;
    impl crate::value::ToValue for EnaSrdUdpSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ena_srd_udp_enabled {
                properties.insert(
                    "EnaSrdUdpEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-enclaveoptions.html
    pub struct EnclaveOptions_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_EnclaveOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.EnclaveOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_EnclaveOptions as EnclaveOptions;
    impl crate::value::ToValue for EnclaveOptions_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-hibernationoptions.html
    pub struct HibernationOptions_ {
        pub configured: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_HibernationOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.HibernationOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_HibernationOptions as HibernationOptions;
    impl crate::value::ToValue for HibernationOptions_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-instanceipv6address.html
    pub struct InstanceIpv6Address_ {
        pub ipv6_address: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_InstanceIpv6Address {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.InstanceIpv6Address"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_InstanceIpv6Address as InstanceIpv6Address;
    impl crate::value::ToValue for InstanceIpv6Address_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Ipv6Address".to_string(),
                crate::value::ToValue::to_value(&self.ipv6_address),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-launchtemplatespecification.html
    pub struct LaunchTemplateSpecification_ {
        pub launch_template_id: Option<crate::value::ExpString>,
        pub launch_template_name: Option<crate::value::ExpString>,
        pub version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_LaunchTemplateSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.LaunchTemplateSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_LaunchTemplateSpecification as LaunchTemplateSpecification;
    impl crate::value::ToValue for LaunchTemplateSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.launch_template_id {
                properties.insert(
                    "LaunchTemplateId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.launch_template_name {
                properties.insert(
                    "LaunchTemplateName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Version".to_string(),
                crate::value::ToValue::to_value(&self.version),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-licensespecification.html
    pub struct LicenseSpecification_ {
        pub license_configuration_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_LicenseSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.LicenseSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_LicenseSpecification as LicenseSpecification;
    impl crate::value::ToValue for LicenseSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LicenseConfigurationArn".to_string(),
                crate::value::ToValue::to_value(&self.license_configuration_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-metadataoptions.html
    pub struct MetadataOptions_ {
        pub http_endpoint: Option<crate::value::ExpString>,
        pub http_protocol_ipv6: Option<crate::value::ExpString>,
        pub http_put_response_hop_limit: Option<i32>,
        pub http_tokens: Option<crate::value::ExpString>,
        pub instance_metadata_tags: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_MetadataOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.MetadataOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_MetadataOptions as MetadataOptions;
    impl crate::value::ToValue for MetadataOptions_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-networkinterface.html
    pub struct NetworkInterface_ {
        pub associate_carrier_ip_address: Option<crate::value::ExpBool>,
        pub associate_public_ip_address: Option<crate::value::ExpBool>,
        pub delete_on_termination: Option<crate::value::ExpBool>,
        pub description: Option<crate::value::ExpString>,
        pub device_index: crate::value::ExpString,
        pub ena_srd_specification: Option<Box<EnaSrdSpecification_>>,
        pub group_set: Option<Vec<crate::value::ExpString>>,
        pub ipv6_address_count: Option<i32>,
        pub ipv6_addresses: Option<Vec<InstanceIpv6Address_>>,
        pub network_interface_id: Option<crate::value::ExpString>,
        pub private_ip_address: Option<crate::value::ExpString>,
        pub private_ip_addresses: Option<Vec<PrivateIpAddressSpecification_>>,
        pub secondary_private_ip_address_count: Option<i32>,
        pub subnet_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_NetworkInterface {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.NetworkInterface"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_NetworkInterface as NetworkInterface;
    impl crate::value::ToValue for NetworkInterface_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.associate_carrier_ip_address {
                properties.insert(
                    "AssociateCarrierIpAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.associate_public_ip_address {
                properties.insert(
                    "AssociatePublicIpAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.delete_on_termination {
                properties.insert(
                    "DeleteOnTermination".to_string(),
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
                "DeviceIndex".to_string(),
                crate::value::ToValue::to_value(&self.device_index),
            );
            if let Some(ref value) = self.ena_srd_specification {
                properties.insert(
                    "EnaSrdSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.group_set {
                properties.insert(
                    "GroupSet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv6_address_count {
                properties.insert(
                    "Ipv6AddressCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv6_addresses {
                properties.insert(
                    "Ipv6Addresses".to_string(),
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
            if let Some(ref value) = self.private_ip_addresses {
                properties.insert(
                    "PrivateIpAddresses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secondary_private_ip_address_count {
                properties.insert(
                    "SecondaryPrivateIpAddressCount".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-privatednsnameoptions.html
    pub struct PrivateDnsNameOptions_ {
        pub enable_resource_name_dns_aaaa_record: Option<crate::value::ExpBool>,
        pub enable_resource_name_dns_a_record: Option<crate::value::ExpBool>,
        pub hostname_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_PrivateDnsNameOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.PrivateDnsNameOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_PrivateDnsNameOptions as PrivateDnsNameOptions;
    impl crate::value::ToValue for PrivateDnsNameOptions_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-privateipaddressspecification.html
    pub struct PrivateIpAddressSpecification_ {
        pub primary: crate::value::ExpBool,
        pub private_ip_address: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_PrivateIpAddressSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.PrivateIpAddressSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_PrivateIpAddressSpecification as PrivateIpAddressSpecification;
    impl crate::value::ToValue for PrivateIpAddressSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Primary".to_string(),
                crate::value::ToValue::to_value(&self.primary),
            );
            properties.insert(
                "PrivateIpAddress".to_string(),
                crate::value::ToValue::to_value(&self.private_ip_address),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-ssmassociation.html
    pub struct SsmAssociation_ {
        pub association_parameters: Option<Vec<AssociationParameter_>>,
        pub document_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_SsmAssociation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.SsmAssociation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_SsmAssociation as SsmAssociation;
    impl crate::value::ToValue for SsmAssociation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.association_parameters {
                properties.insert(
                    "AssociationParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DocumentName".to_string(),
                crate::value::ToValue::to_value(&self.document_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-state.html
    pub struct State_ {
        pub code: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_State {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.State"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_State as State;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-instance-volume.html
    pub struct Volume_ {
        pub device: crate::value::ExpString,
        pub volume_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Instance_Volume {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Instance.Volume"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Instance_Volume as Volume;
    impl crate::value::ToValue for Volume_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Device".to_string(),
                crate::value::ToValue::to_value(&self.device),
            );
            properties.insert(
                "VolumeId".to_string(),
                crate::value::ToValue::to_value(&self.volume_id),
            );
            properties.into()
        }
    }
}
pub mod launchtemplate {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-acceleratorcount.html
    pub struct AcceleratorCount_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_AcceleratorCount {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.AcceleratorCount"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_AcceleratorCount as AcceleratorCount;
    impl crate::value::ToValue for AcceleratorCount_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-acceleratortotalmemorymib.html
    pub struct AcceleratorTotalMemoryMiB_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_AcceleratorTotalMemoryMiB {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.AcceleratorTotalMemoryMiB"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_AcceleratorTotalMemoryMiB as AcceleratorTotalMemoryMiB;
    impl crate::value::ToValue for AcceleratorTotalMemoryMiB_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-baselineebsbandwidthmbps.html
    pub struct BaselineEbsBandwidthMbps_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_BaselineEbsBandwidthMbps {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.BaselineEbsBandwidthMbps"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_BaselineEbsBandwidthMbps as BaselineEbsBandwidthMbps;
    impl crate::value::ToValue for BaselineEbsBandwidthMbps_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-baselineperformancefactors.html
    pub struct BaselinePerformanceFactors_ {
        pub cpu: Option<Box<Cpu_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_BaselinePerformanceFactors {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.BaselinePerformanceFactors"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_BaselinePerformanceFactors as BaselinePerformanceFactors;
    impl crate::value::ToValue for BaselinePerformanceFactors_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cpu {
                properties.insert("Cpu".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-blockdevicemapping.html
    pub struct BlockDeviceMapping_ {
        pub device_name: Option<crate::value::ExpString>,
        pub ebs: Option<Box<Ebs_>>,
        pub no_device: Option<crate::value::ExpString>,
        pub virtual_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_BlockDeviceMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.BlockDeviceMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_BlockDeviceMapping as BlockDeviceMapping;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-capacityreservationspecification.html
    pub struct CapacityReservationSpecification_ {
        pub capacity_reservation_preference: Option<crate::value::ExpString>,
        pub capacity_reservation_target: Option<Box<CapacityReservationTarget_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_CapacityReservationSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.CapacityReservationSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_CapacityReservationSpecification as CapacityReservationSpecification;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-capacityreservationtarget.html
    pub struct CapacityReservationTarget_ {
        pub capacity_reservation_id: Option<crate::value::ExpString>,
        pub capacity_reservation_resource_group_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_CapacityReservationTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.CapacityReservationTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_CapacityReservationTarget as CapacityReservationTarget;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-connectiontrackingspecification.html
    pub struct ConnectionTrackingSpecification_ {
        pub tcp_established_timeout: Option<i32>,
        pub udp_stream_timeout: Option<i32>,
        pub udp_timeout: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_ConnectionTrackingSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.ConnectionTrackingSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_ConnectionTrackingSpecification as ConnectionTrackingSpecification;
    impl crate::value::ToValue for ConnectionTrackingSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.tcp_established_timeout {
                properties.insert(
                    "TcpEstablishedTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.udp_stream_timeout {
                properties.insert(
                    "UdpStreamTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.udp_timeout {
                properties.insert(
                    "UdpTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-cpu.html
    pub struct Cpu_ {
        pub references: Option<Vec<Reference_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_Cpu {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.Cpu"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_Cpu as Cpu;
    impl crate::value::ToValue for Cpu_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.references {
                properties.insert(
                    "References".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-cpuoptions.html
    pub struct CpuOptions_ {
        pub amd_sev_snp: Option<crate::value::ExpString>,
        pub core_count: Option<i32>,
        pub threads_per_core: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_CpuOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.CpuOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_CpuOptions as CpuOptions;
    impl crate::value::ToValue for CpuOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.amd_sev_snp {
                properties.insert(
                    "AmdSevSnp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-creditspecification.html
    pub struct CreditSpecification_ {
        pub cpu_credits: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_CreditSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.CreditSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_CreditSpecification as CreditSpecification;
    impl crate::value::ToValue for CreditSpecification_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-ebs.html
    pub struct Ebs_ {
        pub delete_on_termination: Option<crate::value::ExpBool>,
        pub ebs_card_index: Option<i32>,
        pub encrypted: Option<crate::value::ExpBool>,
        pub iops: Option<i32>,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub snapshot_id: Option<crate::value::ExpString>,
        pub throughput: Option<i32>,
        pub volume_initialization_rate: Option<i32>,
        pub volume_size: Option<i32>,
        pub volume_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_Ebs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.Ebs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_Ebs as Ebs;
    impl crate::value::ToValue for Ebs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delete_on_termination {
                properties.insert(
                    "DeleteOnTermination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebs_card_index {
                properties.insert(
                    "EbsCardIndex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
            if let Some(ref value) = self.snapshot_id {
                properties.insert(
                    "SnapshotId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.throughput {
                properties.insert(
                    "Throughput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.volume_initialization_rate {
                properties.insert(
                    "VolumeInitializationRate".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-enasrdspecification.html
    pub struct EnaSrdSpecification_ {
        pub ena_srd_enabled: Option<crate::value::ExpBool>,
        pub ena_srd_udp_specification: Option<Box<EnaSrdUdpSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_EnaSrdSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.EnaSrdSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_EnaSrdSpecification as EnaSrdSpecification;
    impl crate::value::ToValue for EnaSrdSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ena_srd_enabled {
                properties.insert(
                    "EnaSrdEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ena_srd_udp_specification {
                properties.insert(
                    "EnaSrdUdpSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-enasrdudpspecification.html
    pub struct EnaSrdUdpSpecification_ {
        pub ena_srd_udp_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_EnaSrdUdpSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.EnaSrdUdpSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_EnaSrdUdpSpecification as EnaSrdUdpSpecification;
    impl crate::value::ToValue for EnaSrdUdpSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ena_srd_udp_enabled {
                properties.insert(
                    "EnaSrdUdpEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-enclaveoptions.html
    pub struct EnclaveOptions_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_EnclaveOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.EnclaveOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_EnclaveOptions as EnclaveOptions;
    impl crate::value::ToValue for EnclaveOptions_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-hibernationoptions.html
    pub struct HibernationOptions_ {
        pub configured: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_HibernationOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.HibernationOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_HibernationOptions as HibernationOptions;
    impl crate::value::ToValue for HibernationOptions_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-iaminstanceprofile.html
    pub struct IamInstanceProfile_ {
        pub arn: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_IamInstanceProfile {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.IamInstanceProfile"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_IamInstanceProfile as IamInstanceProfile;
    impl crate::value::ToValue for IamInstanceProfile_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-instancemarketoptions.html
    pub struct InstanceMarketOptions_ {
        pub market_type: Option<crate::value::ExpString>,
        pub spot_options: Option<Box<SpotOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_InstanceMarketOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.InstanceMarketOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_InstanceMarketOptions as InstanceMarketOptions;
    impl crate::value::ToValue for InstanceMarketOptions_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-instancerequirements.html
    pub struct InstanceRequirements_ {
        pub accelerator_count: Option<Box<AcceleratorCount_>>,
        pub accelerator_manufacturers: Option<Vec<crate::value::ExpString>>,
        pub accelerator_names: Option<Vec<crate::value::ExpString>>,
        pub accelerator_total_memory_mi_b: Option<Box<AcceleratorTotalMemoryMiB_>>,
        pub accelerator_types: Option<Vec<crate::value::ExpString>>,
        pub allowed_instance_types: Option<Vec<crate::value::ExpString>>,
        pub bare_metal: Option<crate::value::ExpString>,
        pub baseline_ebs_bandwidth_mbps: Option<Box<BaselineEbsBandwidthMbps_>>,
        pub baseline_performance_factors: Option<Box<BaselinePerformanceFactors_>>,
        pub burstable_performance: Option<crate::value::ExpString>,
        pub cpu_manufacturers: Option<Vec<crate::value::ExpString>>,
        pub excluded_instance_types: Option<Vec<crate::value::ExpString>>,
        pub instance_generations: Option<Vec<crate::value::ExpString>>,
        pub local_storage: Option<crate::value::ExpString>,
        pub local_storage_types: Option<Vec<crate::value::ExpString>>,
        pub max_spot_price_as_percentage_of_optimal_on_demand_price: Option<i32>,
        pub memory_gi_b_per_v_cpu: Option<Box<MemoryGiBPerVCpu_>>,
        pub memory_mi_b: Option<Box<MemoryMiB_>>,
        pub network_bandwidth_gbps: Option<Box<NetworkBandwidthGbps_>>,
        pub network_interface_count: Option<Box<NetworkInterfaceCount_>>,
        pub on_demand_max_price_percentage_over_lowest_price: Option<i32>,
        pub require_hibernate_support: Option<crate::value::ExpBool>,
        pub spot_max_price_percentage_over_lowest_price: Option<i32>,
        pub total_local_storage_gb: Option<Box<TotalLocalStorageGB_>>,
        pub v_cpu_count: Option<Box<VCpuCount_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_InstanceRequirements {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.InstanceRequirements"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_InstanceRequirements as InstanceRequirements;
    impl crate::value::ToValue for InstanceRequirements_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.accelerator_count {
                properties.insert(
                    "AcceleratorCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.accelerator_manufacturers {
                properties.insert(
                    "AcceleratorManufacturers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.accelerator_names {
                properties.insert(
                    "AcceleratorNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.accelerator_total_memory_mi_b {
                properties.insert(
                    "AcceleratorTotalMemoryMiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.accelerator_types {
                properties.insert(
                    "AcceleratorTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allowed_instance_types {
                properties.insert(
                    "AllowedInstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bare_metal {
                properties.insert(
                    "BareMetal".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.baseline_ebs_bandwidth_mbps {
                properties.insert(
                    "BaselineEbsBandwidthMbps".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.baseline_performance_factors {
                properties.insert(
                    "BaselinePerformanceFactors".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.burstable_performance {
                properties.insert(
                    "BurstablePerformance".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cpu_manufacturers {
                properties.insert(
                    "CpuManufacturers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.excluded_instance_types {
                properties.insert(
                    "ExcludedInstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_generations {
                properties.insert(
                    "InstanceGenerations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.local_storage {
                properties.insert(
                    "LocalStorage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.local_storage_types {
                properties.insert(
                    "LocalStorageTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_spot_price_as_percentage_of_optimal_on_demand_price {
                properties.insert(
                    "MaxSpotPriceAsPercentageOfOptimalOnDemandPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory_gi_b_per_v_cpu {
                properties.insert(
                    "MemoryGiBPerVCpu".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory_mi_b {
                properties.insert(
                    "MemoryMiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_bandwidth_gbps {
                properties.insert(
                    "NetworkBandwidthGbps".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_interface_count {
                properties.insert(
                    "NetworkInterfaceCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_demand_max_price_percentage_over_lowest_price {
                properties.insert(
                    "OnDemandMaxPricePercentageOverLowestPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_hibernate_support {
                properties.insert(
                    "RequireHibernateSupport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_max_price_percentage_over_lowest_price {
                properties.insert(
                    "SpotMaxPricePercentageOverLowestPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.total_local_storage_gb {
                properties.insert(
                    "TotalLocalStorageGB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.v_cpu_count {
                properties.insert(
                    "VCpuCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-ipv4prefixspecification.html
    pub struct Ipv4PrefixSpecification_ {
        pub ipv4_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_Ipv4PrefixSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.Ipv4PrefixSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_Ipv4PrefixSpecification as Ipv4PrefixSpecification;
    impl crate::value::ToValue for Ipv4PrefixSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ipv4_prefix {
                properties.insert(
                    "Ipv4Prefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-ipv6add.html
    pub struct Ipv6Add_ {
        pub ipv6_address: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_Ipv6Add {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.Ipv6Add"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_Ipv6Add as Ipv6Add;
    impl crate::value::ToValue for Ipv6Add_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ipv6_address {
                properties.insert(
                    "Ipv6Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-ipv6prefixspecification.html
    pub struct Ipv6PrefixSpecification_ {
        pub ipv6_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_Ipv6PrefixSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.Ipv6PrefixSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_Ipv6PrefixSpecification as Ipv6PrefixSpecification;
    impl crate::value::ToValue for Ipv6PrefixSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ipv6_prefix {
                properties.insert(
                    "Ipv6Prefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-launchtemplatedata.html
    pub struct LaunchTemplateData_ {
        pub block_device_mappings: Option<Vec<BlockDeviceMapping_>>,
        pub capacity_reservation_specification: Option<Box<CapacityReservationSpecification_>>,
        pub cpu_options: Option<Box<CpuOptions_>>,
        pub credit_specification: Option<Box<CreditSpecification_>>,
        pub disable_api_stop: Option<crate::value::ExpBool>,
        pub disable_api_termination: Option<crate::value::ExpBool>,
        pub ebs_optimized: Option<crate::value::ExpBool>,
        pub enclave_options: Option<Box<EnclaveOptions_>>,
        pub hibernation_options: Option<Box<HibernationOptions_>>,
        pub iam_instance_profile: Option<Box<IamInstanceProfile_>>,
        pub image_id: Option<crate::value::ExpString>,
        pub instance_initiated_shutdown_behavior: Option<crate::value::ExpString>,
        pub instance_market_options: Option<Box<InstanceMarketOptions_>>,
        pub instance_requirements: Option<Box<InstanceRequirements_>>,
        pub instance_type: Option<crate::value::ExpString>,
        pub kernel_id: Option<crate::value::ExpString>,
        pub key_name: Option<crate::value::ExpString>,
        pub license_specifications: Option<Vec<LicenseSpecification_>>,
        pub maintenance_options: Option<Box<MaintenanceOptions_>>,
        pub metadata_options: Option<Box<MetadataOptions_>>,
        pub monitoring: Option<Box<Monitoring_>>,
        pub network_interfaces: Option<Vec<NetworkInterface_>>,
        pub network_performance_options: Option<Box<NetworkPerformanceOptions_>>,
        pub placement: Option<Box<Placement_>>,
        pub private_dns_name_options: Option<Box<PrivateDnsNameOptions_>>,
        pub ram_disk_id: Option<crate::value::ExpString>,
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub security_groups: Option<Vec<crate::value::ExpString>>,
        pub tag_specifications: Option<Vec<TagSpecification_>>,
        pub user_data: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_LaunchTemplateData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.LaunchTemplateData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_LaunchTemplateData as LaunchTemplateData;
    impl crate::value::ToValue for LaunchTemplateData_ {
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
            if let Some(ref value) = self.disable_api_termination {
                properties.insert(
                    "DisableApiTermination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebs_optimized {
                properties.insert(
                    "EbsOptimized".to_string(),
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
            if let Some(ref value) = self.image_id {
                properties.insert(
                    "ImageId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_initiated_shutdown_behavior {
                properties.insert(
                    "InstanceInitiatedShutdownBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_market_options {
                properties.insert(
                    "InstanceMarketOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_requirements {
                properties.insert(
                    "InstanceRequirements".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_type {
                properties.insert(
                    "InstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kernel_id {
                properties.insert(
                    "KernelId".to_string(),
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
            if let Some(ref value) = self.ram_disk_id {
                properties.insert(
                    "RamDiskId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_groups {
                properties.insert(
                    "SecurityGroups".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-launchtemplatetagspecification.html
    pub struct LaunchTemplateTagSpecification_ {
        pub resource_type: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_LaunchTemplateTagSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.LaunchTemplateTagSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_LaunchTemplateTagSpecification as LaunchTemplateTagSpecification;
    impl crate::value::ToValue for LaunchTemplateTagSpecification_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-licensespecification.html
    pub struct LicenseSpecification_ {
        pub license_configuration_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_LicenseSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.LicenseSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_LicenseSpecification as LicenseSpecification;
    impl crate::value::ToValue for LicenseSpecification_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-maintenanceoptions.html
    pub struct MaintenanceOptions_ {
        pub auto_recovery: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_MaintenanceOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.MaintenanceOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_MaintenanceOptions as MaintenanceOptions;
    impl crate::value::ToValue for MaintenanceOptions_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-memorygibpervcpu.html
    pub struct MemoryGiBPerVCpu_ {
        pub max: Option<f64>,
        pub min: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_MemoryGiBPerVCpu {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.MemoryGiBPerVCpu"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_MemoryGiBPerVCpu as MemoryGiBPerVCpu;
    impl crate::value::ToValue for MemoryGiBPerVCpu_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-memorymib.html
    pub struct MemoryMiB_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_MemoryMiB {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.MemoryMiB"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_MemoryMiB as MemoryMiB;
    impl crate::value::ToValue for MemoryMiB_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-metadataoptions.html
    pub struct MetadataOptions_ {
        pub http_endpoint: Option<crate::value::ExpString>,
        pub http_protocol_ipv6: Option<crate::value::ExpString>,
        pub http_put_response_hop_limit: Option<i32>,
        pub http_tokens: Option<crate::value::ExpString>,
        pub instance_metadata_tags: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_MetadataOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.MetadataOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_MetadataOptions as MetadataOptions;
    impl crate::value::ToValue for MetadataOptions_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-monitoring.html
    pub struct Monitoring_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_Monitoring {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.Monitoring"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_Monitoring as Monitoring;
    impl crate::value::ToValue for Monitoring_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-networkbandwidthgbps.html
    pub struct NetworkBandwidthGbps_ {
        pub max: Option<f64>,
        pub min: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_NetworkBandwidthGbps {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.NetworkBandwidthGbps"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_NetworkBandwidthGbps as NetworkBandwidthGbps;
    impl crate::value::ToValue for NetworkBandwidthGbps_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-networkinterface.html
    pub struct NetworkInterface_ {
        pub associate_carrier_ip_address: Option<crate::value::ExpBool>,
        pub associate_public_ip_address: Option<crate::value::ExpBool>,
        pub connection_tracking_specification: Option<Box<ConnectionTrackingSpecification_>>,
        pub delete_on_termination: Option<crate::value::ExpBool>,
        pub description: Option<crate::value::ExpString>,
        pub device_index: Option<i32>,
        pub ena_queue_count: Option<i32>,
        pub ena_srd_specification: Option<Box<EnaSrdSpecification_>>,
        pub groups: Option<Vec<crate::value::ExpString>>,
        pub interface_type: Option<crate::value::ExpString>,
        pub ipv4_prefix_count: Option<i32>,
        pub ipv4_prefixes: Option<Vec<Ipv4PrefixSpecification_>>,
        pub ipv6_address_count: Option<i32>,
        pub ipv6_addresses: Option<Vec<Ipv6Add_>>,
        pub ipv6_prefix_count: Option<i32>,
        pub ipv6_prefixes: Option<Vec<Ipv6PrefixSpecification_>>,
        pub network_card_index: Option<i32>,
        pub network_interface_id: Option<crate::value::ExpString>,
        pub primary_ipv6: Option<crate::value::ExpBool>,
        pub private_ip_address: Option<crate::value::ExpString>,
        pub private_ip_addresses: Option<Vec<PrivateIpAdd_>>,
        pub secondary_private_ip_address_count: Option<i32>,
        pub subnet_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_NetworkInterface {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.NetworkInterface"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_NetworkInterface as NetworkInterface;
    impl crate::value::ToValue for NetworkInterface_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.associate_carrier_ip_address {
                properties.insert(
                    "AssociateCarrierIpAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.associate_public_ip_address {
                properties.insert(
                    "AssociatePublicIpAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connection_tracking_specification {
                properties.insert(
                    "ConnectionTrackingSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.delete_on_termination {
                properties.insert(
                    "DeleteOnTermination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
            if let Some(ref value) = self.ena_queue_count {
                properties.insert(
                    "EnaQueueCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ena_srd_specification {
                properties.insert(
                    "EnaSrdSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.groups {
                properties.insert("Groups".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.interface_type {
                properties.insert(
                    "InterfaceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv4_prefix_count {
                properties.insert(
                    "Ipv4PrefixCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv4_prefixes {
                properties.insert(
                    "Ipv4Prefixes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv6_address_count {
                properties.insert(
                    "Ipv6AddressCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv6_addresses {
                properties.insert(
                    "Ipv6Addresses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv6_prefix_count {
                properties.insert(
                    "Ipv6PrefixCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv6_prefixes {
                properties.insert(
                    "Ipv6Prefixes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_card_index {
                properties.insert(
                    "NetworkCardIndex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_interface_id {
                properties.insert(
                    "NetworkInterfaceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.primary_ipv6 {
                properties.insert(
                    "PrimaryIpv6".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.private_ip_address {
                properties.insert(
                    "PrivateIpAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.private_ip_addresses {
                properties.insert(
                    "PrivateIpAddresses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secondary_private_ip_address_count {
                properties.insert(
                    "SecondaryPrivateIpAddressCount".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-networkinterfacecount.html
    pub struct NetworkInterfaceCount_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_NetworkInterfaceCount {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.NetworkInterfaceCount"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_NetworkInterfaceCount as NetworkInterfaceCount;
    impl crate::value::ToValue for NetworkInterfaceCount_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-networkperformanceoptions.html
    pub struct NetworkPerformanceOptions_ {
        pub bandwidth_weighting: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_NetworkPerformanceOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.NetworkPerformanceOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_NetworkPerformanceOptions as NetworkPerformanceOptions;
    impl crate::value::ToValue for NetworkPerformanceOptions_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-placement.html
    pub struct Placement_ {
        pub affinity: Option<crate::value::ExpString>,
        pub availability_zone: Option<crate::value::ExpString>,
        pub group_id: Option<crate::value::ExpString>,
        pub group_name: Option<crate::value::ExpString>,
        pub host_id: Option<crate::value::ExpString>,
        pub host_resource_group_arn: Option<crate::value::ExpString>,
        pub partition_number: Option<i32>,
        pub spread_domain: Option<crate::value::ExpString>,
        pub tenancy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_Placement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.Placement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_Placement as Placement;
    impl crate::value::ToValue for Placement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.affinity {
                properties.insert(
                    "Affinity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
            if let Some(ref value) = self.host_id {
                properties.insert("HostId".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.host_resource_group_arn {
                properties.insert(
                    "HostResourceGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.partition_number {
                properties.insert(
                    "PartitionNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spread_domain {
                properties.insert(
                    "SpreadDomain".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-privatednsnameoptions.html
    pub struct PrivateDnsNameOptions_ {
        pub enable_resource_name_dns_aaaa_record: Option<crate::value::ExpBool>,
        pub enable_resource_name_dns_a_record: Option<crate::value::ExpBool>,
        pub hostname_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_PrivateDnsNameOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.PrivateDnsNameOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_PrivateDnsNameOptions as PrivateDnsNameOptions;
    impl crate::value::ToValue for PrivateDnsNameOptions_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-privateipadd.html
    pub struct PrivateIpAdd_ {
        pub primary: Option<crate::value::ExpBool>,
        pub private_ip_address: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_PrivateIpAdd {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.PrivateIpAdd"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_PrivateIpAdd as PrivateIpAdd;
    impl crate::value::ToValue for PrivateIpAdd_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.primary {
                properties.insert(
                    "Primary".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.private_ip_address {
                properties.insert(
                    "PrivateIpAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-reference.html
    pub struct Reference_ {
        pub instance_family: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_Reference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.Reference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_Reference as Reference;
    impl crate::value::ToValue for Reference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instance_family {
                properties.insert(
                    "InstanceFamily".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-spotoptions.html
    pub struct SpotOptions_ {
        pub block_duration_minutes: Option<i32>,
        pub instance_interruption_behavior: Option<crate::value::ExpString>,
        pub max_price: Option<crate::value::ExpString>,
        pub spot_instance_type: Option<crate::value::ExpString>,
        pub valid_until: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_SpotOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.SpotOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_SpotOptions as SpotOptions;
    impl crate::value::ToValue for SpotOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.block_duration_minutes {
                properties.insert(
                    "BlockDurationMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
            if let Some(ref value) = self.valid_until {
                properties.insert(
                    "ValidUntil".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-tagspecification.html
    pub struct TagSpecification_ {
        pub resource_type: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_TagSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.TagSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_TagSpecification as TagSpecification;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-totallocalstoragegb.html
    pub struct TotalLocalStorageGB_ {
        pub max: Option<f64>,
        pub min: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_TotalLocalStorageGB {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.TotalLocalStorageGB"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_TotalLocalStorageGB as TotalLocalStorageGB;
    impl crate::value::ToValue for TotalLocalStorageGB_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-launchtemplate-vcpucount.html
    pub struct VCpuCount_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_LaunchTemplate_VCpuCount {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::LaunchTemplate.VCpuCount"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_LaunchTemplate_VCpuCount as VCpuCount;
    impl crate::value::ToValue for VCpuCount_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod natgateway {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-natgateway-availabilityzoneaddress.html
    pub struct AvailabilityZoneAddress_ {
        pub allocation_ids: Vec<crate::value::ExpString>,
        pub availability_zone: Option<crate::value::ExpString>,
        pub availability_zone_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NatGateway_AvailabilityZoneAddress {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NatGateway.AvailabilityZoneAddress"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NatGateway_AvailabilityZoneAddress as AvailabilityZoneAddress;
    impl crate::value::ToValue for AvailabilityZoneAddress_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AllocationIds".to_string(),
                crate::value::ToValue::to_value(&self.allocation_ids),
            );
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
            properties.into()
        }
    }
}
pub mod networkaclentry {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkaclentry-icmp.html
    pub struct Icmp_ {
        pub code: Option<i32>,
        pub r#type: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkAclEntry_Icmp {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkAclEntry.Icmp"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkAclEntry_Icmp as Icmp;
    impl crate::value::ToValue for Icmp_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.code {
                properties.insert("Code".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkaclentry-portrange.html
    pub struct PortRange_ {
        pub from: Option<i32>,
        pub to: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkAclEntry_PortRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkAclEntry.PortRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkAclEntry_PortRange as PortRange;
    impl crate::value::ToValue for PortRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.from {
                properties.insert("From".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.to {
                properties.insert("To".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod networkinsightsaccessscope {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsaccessscope-accessscopepathrequest.html
    pub struct AccessScopePathRequest_ {
        pub destination: Option<Box<PathStatementRequest_>>,
        pub source: Option<Box<PathStatementRequest_>>,
        pub through_resources: Option<Vec<ThroughResourcesStatementRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAccessScope_AccessScopePathRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAccessScope.AccessScopePathRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAccessScope_AccessScopePathRequest as AccessScopePathRequest;
    impl crate::value::ToValue for AccessScopePathRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.through_resources {
                properties.insert(
                    "ThroughResources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsaccessscope-packetheaderstatementrequest.html
    pub struct PacketHeaderStatementRequest_ {
        pub destination_addresses: Option<Vec<crate::value::ExpString>>,
        pub destination_ports: Option<Vec<crate::value::ExpString>>,
        pub destination_prefix_lists: Option<Vec<crate::value::ExpString>>,
        pub protocols: Option<Vec<crate::value::ExpString>>,
        pub source_addresses: Option<Vec<crate::value::ExpString>>,
        pub source_ports: Option<Vec<crate::value::ExpString>>,
        pub source_prefix_lists: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAccessScope_PacketHeaderStatementRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAccessScope.PacketHeaderStatementRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAccessScope_PacketHeaderStatementRequest as PacketHeaderStatementRequest;
    impl crate::value::ToValue for PacketHeaderStatementRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_addresses {
                properties.insert(
                    "DestinationAddresses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_ports {
                properties.insert(
                    "DestinationPorts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_prefix_lists {
                properties.insert(
                    "DestinationPrefixLists".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.protocols {
                properties.insert(
                    "Protocols".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_addresses {
                properties.insert(
                    "SourceAddresses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_ports {
                properties.insert(
                    "SourcePorts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_prefix_lists {
                properties.insert(
                    "SourcePrefixLists".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsaccessscope-pathstatementrequest.html
    pub struct PathStatementRequest_ {
        pub packet_header_statement: Option<Box<PacketHeaderStatementRequest_>>,
        pub resource_statement: Option<Box<ResourceStatementRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAccessScope_PathStatementRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAccessScope.PathStatementRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAccessScope_PathStatementRequest as PathStatementRequest;
    impl crate::value::ToValue for PathStatementRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.packet_header_statement {
                properties.insert(
                    "PacketHeaderStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_statement {
                properties.insert(
                    "ResourceStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsaccessscope-resourcestatementrequest.html
    pub struct ResourceStatementRequest_ {
        pub resource_types: Option<Vec<crate::value::ExpString>>,
        pub resources: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAccessScope_ResourceStatementRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAccessScope.ResourceStatementRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAccessScope_ResourceStatementRequest as ResourceStatementRequest;
    impl crate::value::ToValue for ResourceStatementRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.resource_types {
                properties.insert(
                    "ResourceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resources {
                properties.insert(
                    "Resources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsaccessscope-throughresourcesstatementrequest.html
    pub struct ThroughResourcesStatementRequest_ {
        pub resource_statement: Option<Box<ResourceStatementRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAccessScope_ThroughResourcesStatementRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAccessScope.ThroughResourcesStatementRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAccessScope_ThroughResourcesStatementRequest as ThroughResourcesStatementRequest;
    impl crate::value::ToValue for ThroughResourcesStatementRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.resource_statement {
                properties.insert(
                    "ResourceStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod networkinsightsanalysis {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsanalysis-additionaldetail.html
    pub struct AdditionalDetail_ {
        pub additional_detail_type: Option<crate::value::ExpString>,
        pub component: Option<Box<AnalysisComponent_>>,
        pub load_balancers: Option<Vec<AnalysisComponent_>>,
        pub service_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAnalysis_AdditionalDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAnalysis.AdditionalDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAnalysis_AdditionalDetail as AdditionalDetail;
    impl crate::value::ToValue for AdditionalDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_detail_type {
                properties.insert(
                    "AdditionalDetailType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.component {
                properties.insert(
                    "Component".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.load_balancers {
                properties.insert(
                    "LoadBalancers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_name {
                properties.insert(
                    "ServiceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsanalysis-alternatepathhint.html
    pub struct AlternatePathHint_ {
        pub component_arn: Option<crate::value::ExpString>,
        pub component_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAnalysis_AlternatePathHint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAnalysis.AlternatePathHint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAnalysis_AlternatePathHint as AlternatePathHint;
    impl crate::value::ToValue for AlternatePathHint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.component_arn {
                properties.insert(
                    "ComponentArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.component_id {
                properties.insert(
                    "ComponentId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsanalysis-analysisaclrule.html
    pub struct AnalysisAclRule_ {
        pub cidr: Option<crate::value::ExpString>,
        pub egress: Option<crate::value::ExpBool>,
        pub port_range: Option<Box<PortRange_>>,
        pub protocol: Option<crate::value::ExpString>,
        pub rule_action: Option<crate::value::ExpString>,
        pub rule_number: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAnalysis_AnalysisAclRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAnalysis.AnalysisAclRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAnalysis_AnalysisAclRule as AnalysisAclRule;
    impl crate::value::ToValue for AnalysisAclRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cidr {
                properties.insert("Cidr".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.egress {
                properties.insert("Egress".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.port_range {
                properties.insert(
                    "PortRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule_action {
                properties.insert(
                    "RuleAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule_number {
                properties.insert(
                    "RuleNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsanalysis-analysiscomponent.html
    pub struct AnalysisComponent_ {
        pub arn: Option<crate::value::ExpString>,
        pub id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAnalysis_AnalysisComponent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAnalysis.AnalysisComponent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAnalysis_AnalysisComponent as AnalysisComponent;
    impl crate::value::ToValue for AnalysisComponent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsanalysis-analysisloadbalancerlistener.html
    pub struct AnalysisLoadBalancerListener_ {
        pub instance_port: Option<i32>,
        pub load_balancer_port: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAnalysis_AnalysisLoadBalancerListener {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAnalysis.AnalysisLoadBalancerListener"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAnalysis_AnalysisLoadBalancerListener as AnalysisLoadBalancerListener;
    impl crate::value::ToValue for AnalysisLoadBalancerListener_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instance_port {
                properties.insert(
                    "InstancePort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.load_balancer_port {
                properties.insert(
                    "LoadBalancerPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsanalysis-analysisloadbalancertarget.html
    pub struct AnalysisLoadBalancerTarget_ {
        pub address: Option<crate::value::ExpString>,
        pub availability_zone: Option<crate::value::ExpString>,
        pub instance: Option<Box<AnalysisComponent_>>,
        pub port: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAnalysis_AnalysisLoadBalancerTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAnalysis.AnalysisLoadBalancerTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAnalysis_AnalysisLoadBalancerTarget as AnalysisLoadBalancerTarget;
    impl crate::value::ToValue for AnalysisLoadBalancerTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.address {
                properties.insert(
                    "Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.availability_zone {
                properties.insert(
                    "AvailabilityZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance {
                properties.insert(
                    "Instance".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsanalysis-analysispacketheader.html
    pub struct AnalysisPacketHeader_ {
        pub destination_addresses: Option<Vec<crate::value::ExpString>>,
        pub destination_port_ranges: Option<Vec<PortRange_>>,
        pub protocol: Option<crate::value::ExpString>,
        pub source_addresses: Option<Vec<crate::value::ExpString>>,
        pub source_port_ranges: Option<Vec<PortRange_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAnalysis_AnalysisPacketHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAnalysis.AnalysisPacketHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAnalysis_AnalysisPacketHeader as AnalysisPacketHeader;
    impl crate::value::ToValue for AnalysisPacketHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_addresses {
                properties.insert(
                    "DestinationAddresses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_port_ranges {
                properties.insert(
                    "DestinationPortRanges".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_addresses {
                properties.insert(
                    "SourceAddresses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_port_ranges {
                properties.insert(
                    "SourcePortRanges".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsanalysis-analysisroutetableroute.html
    pub struct AnalysisRouteTableRoute_ {
        pub nat_gateway_id: Option<crate::value::ExpString>,
        pub network_interface_id: Option<crate::value::ExpString>,
        pub origin: Option<crate::value::ExpString>,
        pub state: Option<crate::value::ExpString>,
        pub transit_gateway_id: Option<crate::value::ExpString>,
        pub vpc_peering_connection_id: Option<crate::value::ExpString>,
        pub destination_cidr: Option<crate::value::ExpString>,
        pub destination_prefix_list_id: Option<crate::value::ExpString>,
        pub egress_only_internet_gateway_id: Option<crate::value::ExpString>,
        pub gateway_id: Option<crate::value::ExpString>,
        pub instance_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAnalysis_AnalysisRouteTableRoute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAnalysis.AnalysisRouteTableRoute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAnalysis_AnalysisRouteTableRoute as AnalysisRouteTableRoute;
    impl crate::value::ToValue for AnalysisRouteTableRoute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.nat_gateway_id {
                properties.insert(
                    "NatGatewayId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_interface_id {
                properties.insert(
                    "NetworkInterfaceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.origin {
                properties.insert("Origin".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.transit_gateway_id {
                properties.insert(
                    "TransitGatewayId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_peering_connection_id {
                properties.insert(
                    "VpcPeeringConnectionId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_cidr {
                properties.insert(
                    "destinationCidr".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_prefix_list_id {
                properties.insert(
                    "destinationPrefixListId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.egress_only_internet_gateway_id {
                properties.insert(
                    "egressOnlyInternetGatewayId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gateway_id {
                properties.insert(
                    "gatewayId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_id {
                properties.insert(
                    "instanceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsanalysis-analysissecuritygrouprule.html
    pub struct AnalysisSecurityGroupRule_ {
        pub cidr: Option<crate::value::ExpString>,
        pub direction: Option<crate::value::ExpString>,
        pub port_range: Option<Box<PortRange_>>,
        pub prefix_list_id: Option<crate::value::ExpString>,
        pub protocol: Option<crate::value::ExpString>,
        pub security_group_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAnalysis_AnalysisSecurityGroupRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAnalysis.AnalysisSecurityGroupRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAnalysis_AnalysisSecurityGroupRule as AnalysisSecurityGroupRule;
    impl crate::value::ToValue for AnalysisSecurityGroupRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cidr {
                properties.insert("Cidr".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.direction {
                properties.insert(
                    "Direction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port_range {
                properties.insert(
                    "PortRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix_list_id {
                properties.insert(
                    "PrefixListId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_group_id {
                properties.insert(
                    "SecurityGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsanalysis-explanation.html
    pub struct Explanation_ {
        pub acl: Option<Box<AnalysisComponent_>>,
        pub acl_rule: Option<Box<AnalysisAclRule_>>,
        pub address: Option<crate::value::ExpString>,
        pub addresses: Option<Vec<crate::value::ExpString>>,
        pub attached_to: Option<Box<AnalysisComponent_>>,
        pub availability_zones: Option<Vec<crate::value::ExpString>>,
        pub cidrs: Option<Vec<crate::value::ExpString>>,
        pub classic_load_balancer_listener: Option<Box<AnalysisLoadBalancerListener_>>,
        pub component: Option<Box<AnalysisComponent_>>,
        pub component_account: Option<crate::value::ExpString>,
        pub component_region: Option<crate::value::ExpString>,
        pub customer_gateway: Option<Box<AnalysisComponent_>>,
        pub destination: Option<Box<AnalysisComponent_>>,
        pub destination_vpc: Option<Box<AnalysisComponent_>>,
        pub direction: Option<crate::value::ExpString>,
        pub elastic_load_balancer_listener: Option<Box<AnalysisComponent_>>,
        pub explanation_code: Option<crate::value::ExpString>,
        pub ingress_route_table: Option<Box<AnalysisComponent_>>,
        pub internet_gateway: Option<Box<AnalysisComponent_>>,
        pub load_balancer_arn: Option<crate::value::ExpString>,
        pub load_balancer_listener_port: Option<i32>,
        pub load_balancer_target: Option<Box<AnalysisLoadBalancerTarget_>>,
        pub load_balancer_target_group: Option<Box<AnalysisComponent_>>,
        pub load_balancer_target_groups: Option<Vec<AnalysisComponent_>>,
        pub load_balancer_target_port: Option<i32>,
        pub missing_component: Option<crate::value::ExpString>,
        pub nat_gateway: Option<Box<AnalysisComponent_>>,
        pub network_interface: Option<Box<AnalysisComponent_>>,
        pub packet_field: Option<crate::value::ExpString>,
        pub port: Option<i32>,
        pub port_ranges: Option<Vec<PortRange_>>,
        pub prefix_list: Option<Box<AnalysisComponent_>>,
        pub protocols: Option<Vec<crate::value::ExpString>>,
        pub route_table: Option<Box<AnalysisComponent_>>,
        pub route_table_route: Option<Box<AnalysisRouteTableRoute_>>,
        pub security_group: Option<Box<AnalysisComponent_>>,
        pub security_group_rule: Option<Box<AnalysisSecurityGroupRule_>>,
        pub security_groups: Option<Vec<AnalysisComponent_>>,
        pub source_vpc: Option<Box<AnalysisComponent_>>,
        pub state: Option<crate::value::ExpString>,
        pub subnet: Option<Box<AnalysisComponent_>>,
        pub subnet_route_table: Option<Box<AnalysisComponent_>>,
        pub transit_gateway: Option<Box<AnalysisComponent_>>,
        pub transit_gateway_attachment: Option<Box<AnalysisComponent_>>,
        pub transit_gateway_route_table: Option<Box<AnalysisComponent_>>,
        pub transit_gateway_route_table_route: Option<Box<TransitGatewayRouteTableRoute_>>,
        pub vpc: Option<Box<AnalysisComponent_>>,
        pub vpc_peering_connection: Option<Box<AnalysisComponent_>>,
        pub vpn_connection: Option<Box<AnalysisComponent_>>,
        pub vpn_gateway: Option<Box<AnalysisComponent_>>,
        pub vpc_endpoint: Option<Box<AnalysisComponent_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAnalysis_Explanation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAnalysis.Explanation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAnalysis_Explanation as Explanation;
    impl crate::value::ToValue for Explanation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.acl {
                properties.insert("Acl".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.acl_rule {
                properties.insert(
                    "AclRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.address {
                properties.insert(
                    "Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.addresses {
                properties.insert(
                    "Addresses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.attached_to {
                properties.insert(
                    "AttachedTo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.availability_zones {
                properties.insert(
                    "AvailabilityZones".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cidrs {
                properties.insert("Cidrs".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.classic_load_balancer_listener {
                properties.insert(
                    "ClassicLoadBalancerListener".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.component {
                properties.insert(
                    "Component".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.component_account {
                properties.insert(
                    "ComponentAccount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.component_region {
                properties.insert(
                    "ComponentRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.customer_gateway {
                properties.insert(
                    "CustomerGateway".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_vpc {
                properties.insert(
                    "DestinationVpc".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.direction {
                properties.insert(
                    "Direction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.elastic_load_balancer_listener {
                properties.insert(
                    "ElasticLoadBalancerListener".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.explanation_code {
                properties.insert(
                    "ExplanationCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ingress_route_table {
                properties.insert(
                    "IngressRouteTable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.internet_gateway {
                properties.insert(
                    "InternetGateway".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.load_balancer_arn {
                properties.insert(
                    "LoadBalancerArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.load_balancer_listener_port {
                properties.insert(
                    "LoadBalancerListenerPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.load_balancer_target {
                properties.insert(
                    "LoadBalancerTarget".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.load_balancer_target_group {
                properties.insert(
                    "LoadBalancerTargetGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.load_balancer_target_groups {
                properties.insert(
                    "LoadBalancerTargetGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.load_balancer_target_port {
                properties.insert(
                    "LoadBalancerTargetPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.missing_component {
                properties.insert(
                    "MissingComponent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nat_gateway {
                properties.insert(
                    "NatGateway".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_interface {
                properties.insert(
                    "NetworkInterface".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.packet_field {
                properties.insert(
                    "PacketField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.port_ranges {
                properties.insert(
                    "PortRanges".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix_list {
                properties.insert(
                    "PrefixList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.protocols {
                properties.insert(
                    "Protocols".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.route_table {
                properties.insert(
                    "RouteTable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.route_table_route {
                properties.insert(
                    "RouteTableRoute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_group {
                properties.insert(
                    "SecurityGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_group_rule {
                properties.insert(
                    "SecurityGroupRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_groups {
                properties.insert(
                    "SecurityGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_vpc {
                properties.insert(
                    "SourceVpc".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.subnet {
                properties.insert("Subnet".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.subnet_route_table {
                properties.insert(
                    "SubnetRouteTable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transit_gateway {
                properties.insert(
                    "TransitGateway".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transit_gateway_attachment {
                properties.insert(
                    "TransitGatewayAttachment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transit_gateway_route_table {
                properties.insert(
                    "TransitGatewayRouteTable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transit_gateway_route_table_route {
                properties.insert(
                    "TransitGatewayRouteTableRoute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc {
                properties.insert("Vpc".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.vpc_peering_connection {
                properties.insert(
                    "VpcPeeringConnection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpn_connection {
                properties.insert(
                    "VpnConnection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpn_gateway {
                properties.insert(
                    "VpnGateway".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_endpoint {
                properties.insert(
                    "vpcEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsanalysis-pathcomponent.html
    pub struct PathComponent_ {
        pub acl_rule: Option<Box<AnalysisAclRule_>>,
        pub additional_details: Option<Vec<AdditionalDetail_>>,
        pub component: Option<Box<AnalysisComponent_>>,
        pub destination_vpc: Option<Box<AnalysisComponent_>>,
        pub elastic_load_balancer_listener: Option<Box<AnalysisComponent_>>,
        pub explanations: Option<Vec<Explanation_>>,
        pub inbound_header: Option<Box<AnalysisPacketHeader_>>,
        pub outbound_header: Option<Box<AnalysisPacketHeader_>>,
        pub route_table_route: Option<Box<AnalysisRouteTableRoute_>>,
        pub security_group_rule: Option<Box<AnalysisSecurityGroupRule_>>,
        pub sequence_number: Option<i32>,
        pub service_name: Option<crate::value::ExpString>,
        pub source_vpc: Option<Box<AnalysisComponent_>>,
        pub subnet: Option<Box<AnalysisComponent_>>,
        pub transit_gateway: Option<Box<AnalysisComponent_>>,
        pub transit_gateway_route_table_route: Option<Box<TransitGatewayRouteTableRoute_>>,
        pub vpc: Option<Box<AnalysisComponent_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAnalysis_PathComponent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAnalysis.PathComponent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAnalysis_PathComponent as PathComponent;
    impl crate::value::ToValue for PathComponent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.acl_rule {
                properties.insert(
                    "AclRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.additional_details {
                properties.insert(
                    "AdditionalDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.component {
                properties.insert(
                    "Component".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_vpc {
                properties.insert(
                    "DestinationVpc".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.elastic_load_balancer_listener {
                properties.insert(
                    "ElasticLoadBalancerListener".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.explanations {
                properties.insert(
                    "Explanations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inbound_header {
                properties.insert(
                    "InboundHeader".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.outbound_header {
                properties.insert(
                    "OutboundHeader".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.route_table_route {
                properties.insert(
                    "RouteTableRoute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_group_rule {
                properties.insert(
                    "SecurityGroupRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sequence_number {
                properties.insert(
                    "SequenceNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_name {
                properties.insert(
                    "ServiceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_vpc {
                properties.insert(
                    "SourceVpc".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnet {
                properties.insert("Subnet".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.transit_gateway {
                properties.insert(
                    "TransitGateway".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transit_gateway_route_table_route {
                properties.insert(
                    "TransitGatewayRouteTableRoute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc {
                properties.insert("Vpc".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsanalysis-portrange.html
    pub struct PortRange_ {
        pub from: Option<i32>,
        pub to: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAnalysis_PortRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAnalysis.PortRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAnalysis_PortRange as PortRange;
    impl crate::value::ToValue for PortRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.from {
                properties.insert("From".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.to {
                properties.insert("To".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightsanalysis-transitgatewayroutetableroute.html
    pub struct TransitGatewayRouteTableRoute_ {
        pub attachment_id: Option<crate::value::ExpString>,
        pub destination_cidr: Option<crate::value::ExpString>,
        pub prefix_list_id: Option<crate::value::ExpString>,
        pub resource_id: Option<crate::value::ExpString>,
        pub resource_type: Option<crate::value::ExpString>,
        pub route_origin: Option<crate::value::ExpString>,
        pub state: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsAnalysis_TransitGatewayRouteTableRoute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsAnalysis.TransitGatewayRouteTableRoute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsAnalysis_TransitGatewayRouteTableRoute as TransitGatewayRouteTableRoute;
    impl crate::value::ToValue for TransitGatewayRouteTableRoute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attachment_id {
                properties.insert(
                    "AttachmentId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_cidr {
                properties.insert(
                    "DestinationCidr".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix_list_id {
                properties.insert(
                    "PrefixListId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_id {
                properties.insert(
                    "ResourceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_type {
                properties.insert(
                    "ResourceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.route_origin {
                properties.insert(
                    "RouteOrigin".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod networkinsightspath {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightspath-filterportrange.html
    pub struct FilterPortRange_ {
        pub from_port: Option<i32>,
        pub to_port: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsPath_FilterPortRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsPath.FilterPortRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsPath_FilterPortRange as FilterPortRange;
    impl crate::value::ToValue for FilterPortRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.from_port {
                properties.insert(
                    "FromPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.to_port {
                properties.insert("ToPort".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinsightspath-pathfilter.html
    pub struct PathFilter_ {
        pub destination_address: Option<crate::value::ExpString>,
        pub destination_port_range: Option<Box<FilterPortRange_>>,
        pub source_address: Option<crate::value::ExpString>,
        pub source_port_range: Option<Box<FilterPortRange_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInsightsPath_PathFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInsightsPath.PathFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInsightsPath_PathFilter as PathFilter;
    impl crate::value::ToValue for PathFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_address {
                properties.insert(
                    "DestinationAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_port_range {
                properties.insert(
                    "DestinationPortRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_address {
                properties.insert(
                    "SourceAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_port_range {
                properties.insert(
                    "SourcePortRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod networkinterface {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinterface-connectiontrackingspecification.html
    pub struct ConnectionTrackingSpecification_ {
        pub tcp_established_timeout: Option<i32>,
        pub udp_stream_timeout: Option<i32>,
        pub udp_timeout: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInterface_ConnectionTrackingSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInterface.ConnectionTrackingSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInterface_ConnectionTrackingSpecification as ConnectionTrackingSpecification;
    impl crate::value::ToValue for ConnectionTrackingSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.tcp_established_timeout {
                properties.insert(
                    "TcpEstablishedTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.udp_stream_timeout {
                properties.insert(
                    "UdpStreamTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.udp_timeout {
                properties.insert(
                    "UdpTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinterface-instanceipv6address.html
    pub struct InstanceIpv6Address_ {
        pub ipv6_address: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInterface_InstanceIpv6Address {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInterface.InstanceIpv6Address"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInterface_InstanceIpv6Address as InstanceIpv6Address;
    impl crate::value::ToValue for InstanceIpv6Address_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Ipv6Address".to_string(),
                crate::value::ToValue::to_value(&self.ipv6_address),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinterface-ipv4prefixspecification.html
    pub struct Ipv4PrefixSpecification_ {
        pub ipv4_prefix: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInterface_Ipv4PrefixSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInterface.Ipv4PrefixSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInterface_Ipv4PrefixSpecification as Ipv4PrefixSpecification;
    impl crate::value::ToValue for Ipv4PrefixSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Ipv4Prefix".to_string(),
                crate::value::ToValue::to_value(&self.ipv4_prefix),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinterface-ipv6prefixspecification.html
    pub struct Ipv6PrefixSpecification_ {
        pub ipv6_prefix: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInterface_Ipv6PrefixSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInterface.Ipv6PrefixSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInterface_Ipv6PrefixSpecification as Ipv6PrefixSpecification;
    impl crate::value::ToValue for Ipv6PrefixSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Ipv6Prefix".to_string(),
                crate::value::ToValue::to_value(&self.ipv6_prefix),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinterface-privateipaddressspecification.html
    pub struct PrivateIpAddressSpecification_ {
        pub primary: crate::value::ExpBool,
        pub private_ip_address: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInterface_PrivateIpAddressSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInterface.PrivateIpAddressSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInterface_PrivateIpAddressSpecification as PrivateIpAddressSpecification;
    impl crate::value::ToValue for PrivateIpAddressSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Primary".to_string(),
                crate::value::ToValue::to_value(&self.primary),
            );
            properties.insert(
                "PrivateIpAddress".to_string(),
                crate::value::ToValue::to_value(&self.private_ip_address),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinterface-publicipdnsnameoptions.html
    pub struct PublicIpDnsNameOptions_ {
        pub dns_hostname_type: Option<crate::value::ExpString>,
        pub public_dual_stack_dns_name: Option<crate::value::ExpString>,
        pub public_ipv4_dns_name: Option<crate::value::ExpString>,
        pub public_ipv6_dns_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInterface_PublicIpDnsNameOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInterface.PublicIpDnsNameOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInterface_PublicIpDnsNameOptions as PublicIpDnsNameOptions;
    impl crate::value::ToValue for PublicIpDnsNameOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dns_hostname_type {
                properties.insert(
                    "DnsHostnameType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.public_dual_stack_dns_name {
                properties.insert(
                    "PublicDualStackDnsName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.public_ipv4_dns_name {
                properties.insert(
                    "PublicIpv4DnsName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.public_ipv6_dns_name {
                properties.insert(
                    "PublicIpv6DnsName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod networkinterfaceattachment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinterfaceattachment-enasrdspecification.html
    pub struct EnaSrdSpecification_ {
        pub ena_srd_enabled: Option<crate::value::ExpBool>,
        pub ena_srd_udp_specification: Option<Box<EnaSrdUdpSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInterfaceAttachment_EnaSrdSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInterfaceAttachment.EnaSrdSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInterfaceAttachment_EnaSrdSpecification as EnaSrdSpecification;
    impl crate::value::ToValue for EnaSrdSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ena_srd_enabled {
                properties.insert(
                    "EnaSrdEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ena_srd_udp_specification {
                properties.insert(
                    "EnaSrdUdpSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-networkinterfaceattachment-enasrdudpspecification.html
    pub struct EnaSrdUdpSpecification_ {
        pub ena_srd_udp_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_NetworkInterfaceAttachment_EnaSrdUdpSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::NetworkInterfaceAttachment.EnaSrdUdpSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_NetworkInterfaceAttachment_EnaSrdUdpSpecification as EnaSrdUdpSpecification;
    impl crate::value::ToValue for EnaSrdUdpSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ena_srd_udp_enabled {
                properties.insert(
                    "EnaSrdUdpEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod prefixlist {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-prefixlist-entry.html
    pub struct Entry_ {
        pub cidr: crate::value::ExpString,
        pub description: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_PrefixList_Entry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::PrefixList.Entry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_PrefixList_Entry as Entry;
    impl crate::value::ToValue for Entry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Cidr".to_string(),
                crate::value::ToValue::to_value(&self.cidr),
            );
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod routeserverpeer {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-routeserverpeer-bgpoptions.html
    pub struct BgpOptions_ {
        pub peer_asn: Option<i64>,
        pub peer_liveness_detection: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_RouteServerPeer_BgpOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::RouteServerPeer.BgpOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_RouteServerPeer_BgpOptions as BgpOptions;
    impl crate::value::ToValue for BgpOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.peer_asn {
                properties.insert(
                    "PeerAsn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.peer_liveness_detection {
                properties.insert(
                    "PeerLivenessDetection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod securitygroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-securitygroup-egress.html
    pub struct Egress_ {
        pub cidr_ip: Option<crate::value::ExpString>,
        pub cidr_ipv6: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub destination_prefix_list_id: Option<crate::value::ExpString>,
        pub destination_security_group_id: Option<crate::value::ExpString>,
        pub from_port: Option<i32>,
        pub ip_protocol: crate::value::ExpString,
        pub to_port: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SecurityGroup_Egress {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SecurityGroup.Egress"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SecurityGroup_Egress as Egress;
    impl crate::value::ToValue for Egress_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cidr_ip {
                properties.insert("CidrIp".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.cidr_ipv6 {
                properties.insert(
                    "CidrIpv6".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_prefix_list_id {
                properties.insert(
                    "DestinationPrefixListId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_security_group_id {
                properties.insert(
                    "DestinationSecurityGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.from_port {
                properties.insert(
                    "FromPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IpProtocol".to_string(),
                crate::value::ToValue::to_value(&self.ip_protocol),
            );
            if let Some(ref value) = self.to_port {
                properties.insert("ToPort".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-securitygroup-ingress.html
    pub struct Ingress_ {
        pub cidr_ip: Option<crate::value::ExpString>,
        pub cidr_ipv6: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub from_port: Option<i32>,
        pub ip_protocol: crate::value::ExpString,
        pub source_prefix_list_id: Option<crate::value::ExpString>,
        pub source_security_group_id: Option<crate::value::ExpString>,
        pub source_security_group_name: Option<crate::value::ExpString>,
        pub source_security_group_owner_id: Option<crate::value::ExpString>,
        pub to_port: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SecurityGroup_Ingress {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SecurityGroup.Ingress"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SecurityGroup_Ingress as Ingress;
    impl crate::value::ToValue for Ingress_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cidr_ip {
                properties.insert("CidrIp".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.cidr_ipv6 {
                properties.insert(
                    "CidrIpv6".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.from_port {
                properties.insert(
                    "FromPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IpProtocol".to_string(),
                crate::value::ToValue::to_value(&self.ip_protocol),
            );
            if let Some(ref value) = self.source_prefix_list_id {
                properties.insert(
                    "SourcePrefixListId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_security_group_id {
                properties.insert(
                    "SourceSecurityGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_security_group_name {
                properties.insert(
                    "SourceSecurityGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_security_group_owner_id {
                properties.insert(
                    "SourceSecurityGroupOwnerId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.to_port {
                properties.insert("ToPort".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod spotfleet {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-acceleratorcountrequest.html
    pub struct AcceleratorCountRequest_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_AcceleratorCountRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.AcceleratorCountRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_AcceleratorCountRequest as AcceleratorCountRequest;
    impl crate::value::ToValue for AcceleratorCountRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-acceleratortotalmemorymibrequest.html
    pub struct AcceleratorTotalMemoryMiBRequest_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_AcceleratorTotalMemoryMiBRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.AcceleratorTotalMemoryMiBRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_AcceleratorTotalMemoryMiBRequest as AcceleratorTotalMemoryMiBRequest;
    impl crate::value::ToValue for AcceleratorTotalMemoryMiBRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-baselineebsbandwidthmbpsrequest.html
    pub struct BaselineEbsBandwidthMbpsRequest_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_BaselineEbsBandwidthMbpsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.BaselineEbsBandwidthMbpsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_BaselineEbsBandwidthMbpsRequest as BaselineEbsBandwidthMbpsRequest;
    impl crate::value::ToValue for BaselineEbsBandwidthMbpsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-baselineperformancefactorsrequest.html
    pub struct BaselinePerformanceFactorsRequest_ {
        pub cpu: Option<Box<CpuPerformanceFactorRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_BaselinePerformanceFactorsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.BaselinePerformanceFactorsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_BaselinePerformanceFactorsRequest as BaselinePerformanceFactorsRequest;
    impl crate::value::ToValue for BaselinePerformanceFactorsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cpu {
                properties.insert("Cpu".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-blockdevicemapping.html
    pub struct BlockDeviceMapping_ {
        pub device_name: crate::value::ExpString,
        pub ebs: Option<Box<EbsBlockDevice_>>,
        pub no_device: Option<crate::value::ExpString>,
        pub virtual_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_BlockDeviceMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.BlockDeviceMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_BlockDeviceMapping as BlockDeviceMapping;
    impl crate::value::ToValue for BlockDeviceMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DeviceName".to_string(),
                crate::value::ToValue::to_value(&self.device_name),
            );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-classicloadbalancer.html
    pub struct ClassicLoadBalancer_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_ClassicLoadBalancer {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.ClassicLoadBalancer"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_ClassicLoadBalancer as ClassicLoadBalancer;
    impl crate::value::ToValue for ClassicLoadBalancer_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-classicloadbalancersconfig.html
    pub struct ClassicLoadBalancersConfig_ {
        pub classic_load_balancers: Vec<ClassicLoadBalancer_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_ClassicLoadBalancersConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.ClassicLoadBalancersConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_ClassicLoadBalancersConfig as ClassicLoadBalancersConfig;
    impl crate::value::ToValue for ClassicLoadBalancersConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClassicLoadBalancers".to_string(),
                crate::value::ToValue::to_value(&self.classic_load_balancers),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-cpuperformancefactorrequest.html
    pub struct CpuPerformanceFactorRequest_ {
        pub references: Option<Vec<PerformanceFactorReferenceRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_CpuPerformanceFactorRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.CpuPerformanceFactorRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_CpuPerformanceFactorRequest as CpuPerformanceFactorRequest;
    impl crate::value::ToValue for CpuPerformanceFactorRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.references {
                properties.insert(
                    "References".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-ebsblockdevice.html
    pub struct EbsBlockDevice_ {
        pub delete_on_termination: Option<crate::value::ExpBool>,
        pub encrypted: Option<crate::value::ExpBool>,
        pub iops: Option<i32>,
        pub snapshot_id: Option<crate::value::ExpString>,
        pub volume_size: Option<i32>,
        pub volume_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_EbsBlockDevice {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.EbsBlockDevice"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_EbsBlockDevice as EbsBlockDevice;
    impl crate::value::ToValue for EbsBlockDevice_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delete_on_termination {
                properties.insert(
                    "DeleteOnTermination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encrypted {
                properties.insert(
                    "Encrypted".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iops {
                properties.insert("Iops".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.snapshot_id {
                properties.insert(
                    "SnapshotId".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-fleetlaunchtemplatespecification.html
    pub struct FleetLaunchTemplateSpecification_ {
        pub launch_template_id: Option<crate::value::ExpString>,
        pub launch_template_name: Option<crate::value::ExpString>,
        pub version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_FleetLaunchTemplateSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.FleetLaunchTemplateSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_FleetLaunchTemplateSpecification as FleetLaunchTemplateSpecification;
    impl crate::value::ToValue for FleetLaunchTemplateSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.launch_template_id {
                properties.insert(
                    "LaunchTemplateId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.launch_template_name {
                properties.insert(
                    "LaunchTemplateName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Version".to_string(),
                crate::value::ToValue::to_value(&self.version),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-groupidentifier.html
    pub struct GroupIdentifier_ {
        pub group_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_GroupIdentifier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.GroupIdentifier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_GroupIdentifier as GroupIdentifier;
    impl crate::value::ToValue for GroupIdentifier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GroupId".to_string(),
                crate::value::ToValue::to_value(&self.group_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-iaminstanceprofilespecification.html
    pub struct IamInstanceProfileSpecification_ {
        pub arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_IamInstanceProfileSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.IamInstanceProfileSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_IamInstanceProfileSpecification as IamInstanceProfileSpecification;
    impl crate::value::ToValue for IamInstanceProfileSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-instanceipv6address.html
    pub struct InstanceIpv6Address_ {
        pub ipv6_address: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_InstanceIpv6Address {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.InstanceIpv6Address"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_InstanceIpv6Address as InstanceIpv6Address;
    impl crate::value::ToValue for InstanceIpv6Address_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Ipv6Address".to_string(),
                crate::value::ToValue::to_value(&self.ipv6_address),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-instancenetworkinterfacespecification.html
    pub struct InstanceNetworkInterfaceSpecification_ {
        pub associate_public_ip_address: Option<crate::value::ExpBool>,
        pub delete_on_termination: Option<crate::value::ExpBool>,
        pub description: Option<crate::value::ExpString>,
        pub device_index: Option<i32>,
        pub groups: Option<Vec<crate::value::ExpString>>,
        pub ipv6_address_count: Option<i32>,
        pub ipv6_addresses: Option<Vec<InstanceIpv6Address_>>,
        pub network_interface_id: Option<crate::value::ExpString>,
        pub private_ip_addresses: Option<Vec<PrivateIpAddressSpecification_>>,
        pub secondary_private_ip_address_count: Option<i32>,
        pub subnet_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_InstanceNetworkInterfaceSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.InstanceNetworkInterfaceSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_InstanceNetworkInterfaceSpecification as InstanceNetworkInterfaceSpecification;
    impl crate::value::ToValue for InstanceNetworkInterfaceSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.associate_public_ip_address {
                properties.insert(
                    "AssociatePublicIpAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.delete_on_termination {
                properties.insert(
                    "DeleteOnTermination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
            if let Some(ref value) = self.ipv6_address_count {
                properties.insert(
                    "Ipv6AddressCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv6_addresses {
                properties.insert(
                    "Ipv6Addresses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_interface_id {
                properties.insert(
                    "NetworkInterfaceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.private_ip_addresses {
                properties.insert(
                    "PrivateIpAddresses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secondary_private_ip_address_count {
                properties.insert(
                    "SecondaryPrivateIpAddressCount".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-instancerequirementsrequest.html
    pub struct InstanceRequirementsRequest_ {
        pub accelerator_count: Option<Box<AcceleratorCountRequest_>>,
        pub accelerator_manufacturers: Option<Vec<crate::value::ExpString>>,
        pub accelerator_names: Option<Vec<crate::value::ExpString>>,
        pub accelerator_total_memory_mi_b: Option<Box<AcceleratorTotalMemoryMiBRequest_>>,
        pub accelerator_types: Option<Vec<crate::value::ExpString>>,
        pub allowed_instance_types: Option<Vec<crate::value::ExpString>>,
        pub bare_metal: Option<crate::value::ExpString>,
        pub baseline_ebs_bandwidth_mbps: Option<Box<BaselineEbsBandwidthMbpsRequest_>>,
        pub baseline_performance_factors: Option<Box<BaselinePerformanceFactorsRequest_>>,
        pub burstable_performance: Option<crate::value::ExpString>,
        pub cpu_manufacturers: Option<Vec<crate::value::ExpString>>,
        pub excluded_instance_types: Option<Vec<crate::value::ExpString>>,
        pub instance_generations: Option<Vec<crate::value::ExpString>>,
        pub local_storage: Option<crate::value::ExpString>,
        pub local_storage_types: Option<Vec<crate::value::ExpString>>,
        pub max_spot_price_as_percentage_of_optimal_on_demand_price: Option<i32>,
        pub memory_gi_b_per_v_cpu: Option<Box<MemoryGiBPerVCpuRequest_>>,
        pub memory_mi_b: Option<Box<MemoryMiBRequest_>>,
        pub network_bandwidth_gbps: Option<Box<NetworkBandwidthGbpsRequest_>>,
        pub network_interface_count: Option<Box<NetworkInterfaceCountRequest_>>,
        pub on_demand_max_price_percentage_over_lowest_price: Option<i32>,
        pub require_encryption_in_transit: Option<crate::value::ExpBool>,
        pub require_hibernate_support: Option<crate::value::ExpBool>,
        pub spot_max_price_percentage_over_lowest_price: Option<i32>,
        pub total_local_storage_gb: Option<Box<TotalLocalStorageGBRequest_>>,
        pub v_cpu_count: Option<Box<VCpuCountRangeRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_InstanceRequirementsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.InstanceRequirementsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_InstanceRequirementsRequest as InstanceRequirementsRequest;
    impl crate::value::ToValue for InstanceRequirementsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.accelerator_count {
                properties.insert(
                    "AcceleratorCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.accelerator_manufacturers {
                properties.insert(
                    "AcceleratorManufacturers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.accelerator_names {
                properties.insert(
                    "AcceleratorNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.accelerator_total_memory_mi_b {
                properties.insert(
                    "AcceleratorTotalMemoryMiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.accelerator_types {
                properties.insert(
                    "AcceleratorTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allowed_instance_types {
                properties.insert(
                    "AllowedInstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bare_metal {
                properties.insert(
                    "BareMetal".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.baseline_ebs_bandwidth_mbps {
                properties.insert(
                    "BaselineEbsBandwidthMbps".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.baseline_performance_factors {
                properties.insert(
                    "BaselinePerformanceFactors".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.burstable_performance {
                properties.insert(
                    "BurstablePerformance".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cpu_manufacturers {
                properties.insert(
                    "CpuManufacturers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.excluded_instance_types {
                properties.insert(
                    "ExcludedInstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_generations {
                properties.insert(
                    "InstanceGenerations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.local_storage {
                properties.insert(
                    "LocalStorage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.local_storage_types {
                properties.insert(
                    "LocalStorageTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_spot_price_as_percentage_of_optimal_on_demand_price {
                properties.insert(
                    "MaxSpotPriceAsPercentageOfOptimalOnDemandPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory_gi_b_per_v_cpu {
                properties.insert(
                    "MemoryGiBPerVCpu".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory_mi_b {
                properties.insert(
                    "MemoryMiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_bandwidth_gbps {
                properties.insert(
                    "NetworkBandwidthGbps".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_interface_count {
                properties.insert(
                    "NetworkInterfaceCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_demand_max_price_percentage_over_lowest_price {
                properties.insert(
                    "OnDemandMaxPricePercentageOverLowestPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_encryption_in_transit {
                properties.insert(
                    "RequireEncryptionInTransit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_hibernate_support {
                properties.insert(
                    "RequireHibernateSupport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_max_price_percentage_over_lowest_price {
                properties.insert(
                    "SpotMaxPricePercentageOverLowestPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.total_local_storage_gb {
                properties.insert(
                    "TotalLocalStorageGB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.v_cpu_count {
                properties.insert(
                    "VCpuCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-launchtemplateconfig.html
    pub struct LaunchTemplateConfig_ {
        pub launch_template_specification: Option<Box<FleetLaunchTemplateSpecification_>>,
        pub overrides: Option<Vec<LaunchTemplateOverrides_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_LaunchTemplateConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.LaunchTemplateConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_LaunchTemplateConfig as LaunchTemplateConfig;
    impl crate::value::ToValue for LaunchTemplateConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.launch_template_specification {
                properties.insert(
                    "LaunchTemplateSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.overrides {
                properties.insert(
                    "Overrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-launchtemplateoverrides.html
    pub struct LaunchTemplateOverrides_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub availability_zone_id: Option<crate::value::ExpString>,
        pub instance_requirements: Option<Box<InstanceRequirementsRequest_>>,
        pub instance_type: Option<crate::value::ExpString>,
        pub priority: Option<f64>,
        pub spot_price: Option<crate::value::ExpString>,
        pub subnet_id: Option<crate::value::ExpString>,
        pub weighted_capacity: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_LaunchTemplateOverrides {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.LaunchTemplateOverrides"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_LaunchTemplateOverrides as LaunchTemplateOverrides;
    impl crate::value::ToValue for LaunchTemplateOverrides_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
            if let Some(ref value) = self.instance_requirements {
                properties.insert(
                    "InstanceRequirements".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_type {
                properties.insert(
                    "InstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.priority {
                properties.insert(
                    "Priority".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_price {
                properties.insert(
                    "SpotPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnet_id {
                properties.insert(
                    "SubnetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weighted_capacity {
                properties.insert(
                    "WeightedCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-loadbalancersconfig.html
    pub struct LoadBalancersConfig_ {
        pub classic_load_balancers_config: Option<Box<ClassicLoadBalancersConfig_>>,
        pub target_groups_config: Option<Box<TargetGroupsConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_LoadBalancersConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.LoadBalancersConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_LoadBalancersConfig as LoadBalancersConfig;
    impl crate::value::ToValue for LoadBalancersConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.classic_load_balancers_config {
                properties.insert(
                    "ClassicLoadBalancersConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_groups_config {
                properties.insert(
                    "TargetGroupsConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-memorygibpervcpurequest.html
    pub struct MemoryGiBPerVCpuRequest_ {
        pub max: Option<f64>,
        pub min: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_MemoryGiBPerVCpuRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.MemoryGiBPerVCpuRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_MemoryGiBPerVCpuRequest as MemoryGiBPerVCpuRequest;
    impl crate::value::ToValue for MemoryGiBPerVCpuRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-memorymibrequest.html
    pub struct MemoryMiBRequest_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_MemoryMiBRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.MemoryMiBRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_MemoryMiBRequest as MemoryMiBRequest;
    impl crate::value::ToValue for MemoryMiBRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-networkbandwidthgbpsrequest.html
    pub struct NetworkBandwidthGbpsRequest_ {
        pub max: Option<f64>,
        pub min: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_NetworkBandwidthGbpsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.NetworkBandwidthGbpsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_NetworkBandwidthGbpsRequest as NetworkBandwidthGbpsRequest;
    impl crate::value::ToValue for NetworkBandwidthGbpsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-networkinterfacecountrequest.html
    pub struct NetworkInterfaceCountRequest_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_NetworkInterfaceCountRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.NetworkInterfaceCountRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_NetworkInterfaceCountRequest as NetworkInterfaceCountRequest;
    impl crate::value::ToValue for NetworkInterfaceCountRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-performancefactorreferencerequest.html
    pub struct PerformanceFactorReferenceRequest_ {
        pub instance_family: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_PerformanceFactorReferenceRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.PerformanceFactorReferenceRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_PerformanceFactorReferenceRequest as PerformanceFactorReferenceRequest;
    impl crate::value::ToValue for PerformanceFactorReferenceRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instance_family {
                properties.insert(
                    "InstanceFamily".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-privateipaddressspecification.html
    pub struct PrivateIpAddressSpecification_ {
        pub primary: Option<crate::value::ExpBool>,
        pub private_ip_address: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_PrivateIpAddressSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.PrivateIpAddressSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_PrivateIpAddressSpecification as PrivateIpAddressSpecification;
    impl crate::value::ToValue for PrivateIpAddressSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.primary {
                properties.insert(
                    "Primary".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PrivateIpAddress".to_string(),
                crate::value::ToValue::to_value(&self.private_ip_address),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotcapacityrebalance.html
    pub struct SpotCapacityRebalance_ {
        pub replacement_strategy: Option<crate::value::ExpString>,
        pub termination_delay: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_SpotCapacityRebalance {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.SpotCapacityRebalance"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_SpotCapacityRebalance as SpotCapacityRebalance;
    impl crate::value::ToValue for SpotCapacityRebalance_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.replacement_strategy {
                properties.insert(
                    "ReplacementStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.termination_delay {
                properties.insert(
                    "TerminationDelay".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetlaunchspecification.html
    pub struct SpotFleetLaunchSpecification_ {
        pub block_device_mappings: Option<Vec<BlockDeviceMapping_>>,
        pub ebs_optimized: Option<crate::value::ExpBool>,
        pub iam_instance_profile: Option<Box<IamInstanceProfileSpecification_>>,
        pub image_id: crate::value::ExpString,
        pub instance_requirements: Option<Box<InstanceRequirementsRequest_>>,
        pub instance_type: Option<crate::value::ExpString>,
        pub kernel_id: Option<crate::value::ExpString>,
        pub key_name: Option<crate::value::ExpString>,
        pub monitoring: Option<Box<SpotFleetMonitoring_>>,
        pub network_interfaces: Option<Vec<InstanceNetworkInterfaceSpecification_>>,
        pub placement: Option<Box<SpotPlacement_>>,
        pub ramdisk_id: Option<crate::value::ExpString>,
        pub security_groups: Option<Vec<GroupIdentifier_>>,
        pub spot_price: Option<crate::value::ExpString>,
        pub subnet_id: Option<crate::value::ExpString>,
        pub tag_specifications: Option<Vec<SpotFleetTagSpecification_>>,
        pub user_data: Option<crate::value::ExpString>,
        pub weighted_capacity: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_SpotFleetLaunchSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.SpotFleetLaunchSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_SpotFleetLaunchSpecification as SpotFleetLaunchSpecification;
    impl crate::value::ToValue for SpotFleetLaunchSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.block_device_mappings {
                properties.insert(
                    "BlockDeviceMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebs_optimized {
                properties.insert(
                    "EbsOptimized".to_string(),
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
            if let Some(ref value) = self.instance_requirements {
                properties.insert(
                    "InstanceRequirements".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_type {
                properties.insert(
                    "InstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kernel_id {
                properties.insert(
                    "KernelId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_name {
                properties.insert(
                    "KeyName".to_string(),
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
            if let Some(ref value) = self.placement {
                properties.insert(
                    "Placement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ramdisk_id {
                properties.insert(
                    "RamdiskId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_groups {
                properties.insert(
                    "SecurityGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_price {
                properties.insert(
                    "SpotPrice".to_string(),
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
            if let Some(ref value) = self.weighted_capacity {
                properties.insert(
                    "WeightedCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetmonitoring.html
    pub struct SpotFleetMonitoring_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_SpotFleetMonitoring {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.SpotFleetMonitoring"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_SpotFleetMonitoring as SpotFleetMonitoring;
    impl crate::value::ToValue for SpotFleetMonitoring_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleetrequestconfigdata.html
    pub struct SpotFleetRequestConfigData_ {
        pub allocation_strategy: Option<crate::value::ExpString>,
        pub context: Option<crate::value::ExpString>,
        pub excess_capacity_termination_policy: Option<crate::value::ExpString>,
        pub iam_fleet_role: crate::value::ExpString,
        pub instance_interruption_behavior: Option<crate::value::ExpString>,
        pub instance_pools_to_use_count: Option<i32>,
        pub launch_specifications: Option<Vec<SpotFleetLaunchSpecification_>>,
        pub launch_template_configs: Option<Vec<LaunchTemplateConfig_>>,
        pub load_balancers_config: Option<Box<LoadBalancersConfig_>>,
        pub on_demand_allocation_strategy: Option<crate::value::ExpString>,
        pub on_demand_max_total_price: Option<crate::value::ExpString>,
        pub on_demand_target_capacity: Option<i32>,
        pub replace_unhealthy_instances: Option<crate::value::ExpBool>,
        pub spot_maintenance_strategies: Option<Box<SpotMaintenanceStrategies_>>,
        pub spot_max_total_price: Option<crate::value::ExpString>,
        pub spot_price: Option<crate::value::ExpString>,
        pub tag_specifications: Option<Vec<SpotFleetTagSpecification_>>,
        pub target_capacity: i32,
        pub target_capacity_unit_type: Option<crate::value::ExpString>,
        pub terminate_instances_with_expiration: Option<crate::value::ExpBool>,
        pub r#type: Option<crate::value::ExpString>,
        pub valid_from: Option<crate::value::ExpString>,
        pub valid_until: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_SpotFleetRequestConfigData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.SpotFleetRequestConfigData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_SpotFleetRequestConfigData as SpotFleetRequestConfigData;
    impl crate::value::ToValue for SpotFleetRequestConfigData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allocation_strategy {
                properties.insert(
                    "AllocationStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.context {
                properties.insert(
                    "Context".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.excess_capacity_termination_policy {
                properties.insert(
                    "ExcessCapacityTerminationPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IamFleetRole".to_string(),
                crate::value::ToValue::to_value(&self.iam_fleet_role),
            );
            if let Some(ref value) = self.instance_interruption_behavior {
                properties.insert(
                    "InstanceInterruptionBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_pools_to_use_count {
                properties.insert(
                    "InstancePoolsToUseCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.launch_specifications {
                properties.insert(
                    "LaunchSpecifications".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.launch_template_configs {
                properties.insert(
                    "LaunchTemplateConfigs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.load_balancers_config {
                properties.insert(
                    "LoadBalancersConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_demand_allocation_strategy {
                properties.insert(
                    "OnDemandAllocationStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_demand_max_total_price {
                properties.insert(
                    "OnDemandMaxTotalPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_demand_target_capacity {
                properties.insert(
                    "OnDemandTargetCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.replace_unhealthy_instances {
                properties.insert(
                    "ReplaceUnhealthyInstances".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_maintenance_strategies {
                properties.insert(
                    "SpotMaintenanceStrategies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_max_total_price {
                properties.insert(
                    "SpotMaxTotalPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_price {
                properties.insert(
                    "SpotPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_specifications {
                properties.insert(
                    "TagSpecifications".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetCapacity".to_string(),
                crate::value::ToValue::to_value(&self.target_capacity),
            );
            if let Some(ref value) = self.target_capacity_unit_type {
                properties.insert(
                    "TargetCapacityUnitType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.terminate_instances_with_expiration {
                properties.insert(
                    "TerminateInstancesWithExpiration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.valid_from {
                properties.insert(
                    "ValidFrom".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.valid_until {
                properties.insert(
                    "ValidUntil".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotfleettagspecification.html
    pub struct SpotFleetTagSpecification_ {
        pub resource_type: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_SpotFleetTagSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.SpotFleetTagSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_SpotFleetTagSpecification as SpotFleetTagSpecification;
    impl crate::value::ToValue for SpotFleetTagSpecification_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotmaintenancestrategies.html
    pub struct SpotMaintenanceStrategies_ {
        pub capacity_rebalance: Option<Box<SpotCapacityRebalance_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_SpotMaintenanceStrategies {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.SpotMaintenanceStrategies"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_SpotMaintenanceStrategies as SpotMaintenanceStrategies;
    impl crate::value::ToValue for SpotMaintenanceStrategies_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity_rebalance {
                properties.insert(
                    "CapacityRebalance".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-spotplacement.html
    pub struct SpotPlacement_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub availability_zone_id: Option<crate::value::ExpString>,
        pub group_name: Option<crate::value::ExpString>,
        pub tenancy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_SpotPlacement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.SpotPlacement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_SpotPlacement as SpotPlacement;
    impl crate::value::ToValue for SpotPlacement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
            if let Some(ref value) = self.group_name {
                properties.insert(
                    "GroupName".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-targetgroup.html
    pub struct TargetGroup_ {
        pub arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_TargetGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.TargetGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_TargetGroup as TargetGroup;
    impl crate::value::ToValue for TargetGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-targetgroupsconfig.html
    pub struct TargetGroupsConfig_ {
        pub target_groups: Vec<TargetGroup_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_TargetGroupsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.TargetGroupsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_TargetGroupsConfig as TargetGroupsConfig;
    impl crate::value::ToValue for TargetGroupsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TargetGroups".to_string(),
                crate::value::ToValue::to_value(&self.target_groups),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-totallocalstoragegbrequest.html
    pub struct TotalLocalStorageGBRequest_ {
        pub max: Option<f64>,
        pub min: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_TotalLocalStorageGBRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.TotalLocalStorageGBRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_TotalLocalStorageGBRequest as TotalLocalStorageGBRequest;
    impl crate::value::ToValue for TotalLocalStorageGBRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-spotfleet-vcpucountrangerequest.html
    pub struct VCpuCountRangeRequest_ {
        pub max: Option<i32>,
        pub min: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_SpotFleet_VCpuCountRangeRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::SpotFleet.VCpuCountRangeRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_SpotFleet_VCpuCountRangeRequest as VCpuCountRangeRequest;
    impl crate::value::ToValue for VCpuCountRangeRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod subnet {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-subnet-blockpublicaccessstates.html
    pub struct BlockPublicAccessStates_ {
        pub internet_gateway_block_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Subnet_BlockPublicAccessStates {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Subnet.BlockPublicAccessStates"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Subnet_BlockPublicAccessStates as BlockPublicAccessStates;
    impl crate::value::ToValue for BlockPublicAccessStates_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.internet_gateway_block_mode {
                properties.insert(
                    "InternetGatewayBlockMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-subnet-privatednsnameoptionsonlaunch.html
    pub struct PrivateDnsNameOptionsOnLaunch_ {
        pub enable_resource_name_dns_aaaa_record: Option<crate::value::ExpBool>,
        pub enable_resource_name_dns_a_record: Option<crate::value::ExpBool>,
        pub hostname_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_Subnet_PrivateDnsNameOptionsOnLaunch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::Subnet.PrivateDnsNameOptionsOnLaunch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_Subnet_PrivateDnsNameOptionsOnLaunch as PrivateDnsNameOptionsOnLaunch;
    impl crate::value::ToValue for PrivateDnsNameOptionsOnLaunch_ {
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
}
pub mod trafficmirrorfilterrule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-trafficmirrorfilterrule-trafficmirrorportrange.html
    pub struct TrafficMirrorPortRange_ {
        pub from_port: i32,
        pub to_port: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_TrafficMirrorFilterRule_TrafficMirrorPortRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::TrafficMirrorFilterRule.TrafficMirrorPortRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_TrafficMirrorFilterRule_TrafficMirrorPortRange as TrafficMirrorPortRange;
    impl crate::value::ToValue for TrafficMirrorPortRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FromPort".to_string(),
                crate::value::ToValue::to_value(&self.from_port),
            );
            properties.insert(
                "ToPort".to_string(),
                crate::value::ToValue::to_value(&self.to_port),
            );
            properties.into()
        }
    }
}
pub mod transitgatewayattachment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-transitgatewayattachment-options.html
    pub struct Options_ {
        pub appliance_mode_support: Option<crate::value::ExpString>,
        pub dns_support: Option<crate::value::ExpString>,
        pub ipv6_support: Option<crate::value::ExpString>,
        pub security_group_referencing_support: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_TransitGatewayAttachment_Options {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::TransitGatewayAttachment.Options"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_TransitGatewayAttachment_Options as Options;
    impl crate::value::ToValue for Options_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.appliance_mode_support {
                properties.insert(
                    "ApplianceModeSupport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dns_support {
                properties.insert(
                    "DnsSupport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv6_support {
                properties.insert(
                    "Ipv6Support".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_group_referencing_support {
                properties.insert(
                    "SecurityGroupReferencingSupport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod transitgatewayconnect {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-transitgatewayconnect-transitgatewayconnectoptions.html
    pub struct TransitGatewayConnectOptions_ {
        pub protocol: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_TransitGatewayConnect_TransitGatewayConnectOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::TransitGatewayConnect.TransitGatewayConnectOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_TransitGatewayConnect_TransitGatewayConnectOptions as TransitGatewayConnectOptions;
    impl crate::value::ToValue for TransitGatewayConnectOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod transitgatewayconnectpeer {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-transitgatewayconnectpeer-transitgatewayattachmentbgpconfiguration.html
    pub struct TransitGatewayAttachmentBgpConfiguration_ {
        pub bgp_status: Option<crate::value::ExpString>,
        pub peer_address: Option<crate::value::ExpString>,
        pub peer_asn: Option<f64>,
        pub transit_gateway_address: Option<crate::value::ExpString>,
        pub transit_gateway_asn: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_TransitGatewayConnectPeer_TransitGatewayAttachmentBgpConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::TransitGatewayConnectPeer.TransitGatewayAttachmentBgpConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_TransitGatewayConnectPeer_TransitGatewayAttachmentBgpConfiguration as TransitGatewayAttachmentBgpConfiguration;
    impl crate::value::ToValue for TransitGatewayAttachmentBgpConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bgp_status {
                properties.insert(
                    "BgpStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.peer_address {
                properties.insert(
                    "PeerAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.peer_asn {
                properties.insert(
                    "PeerAsn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transit_gateway_address {
                properties.insert(
                    "TransitGatewayAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transit_gateway_asn {
                properties.insert(
                    "TransitGatewayAsn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-transitgatewayconnectpeer-transitgatewayconnectpeerconfiguration.html
    pub struct TransitGatewayConnectPeerConfiguration_ {
        pub bgp_configurations: Option<Vec<TransitGatewayAttachmentBgpConfiguration_>>,
        pub inside_cidr_blocks: Vec<crate::value::ExpString>,
        pub peer_address: crate::value::ExpString,
        pub protocol: Option<crate::value::ExpString>,
        pub transit_gateway_address: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_TransitGatewayConnectPeer_TransitGatewayConnectPeerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::TransitGatewayConnectPeer.TransitGatewayConnectPeerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_TransitGatewayConnectPeer_TransitGatewayConnectPeerConfiguration as TransitGatewayConnectPeerConfiguration;
    impl crate::value::ToValue for TransitGatewayConnectPeerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bgp_configurations {
                properties.insert(
                    "BgpConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InsideCidrBlocks".to_string(),
                crate::value::ToValue::to_value(&self.inside_cidr_blocks),
            );
            properties.insert(
                "PeerAddress".to_string(),
                crate::value::ToValue::to_value(&self.peer_address),
            );
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transit_gateway_address {
                properties.insert(
                    "TransitGatewayAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod transitgatewaymulticastdomain {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-transitgatewaymulticastdomain-options.html
    pub struct Options_ {
        pub auto_accept_shared_associations: Option<crate::value::ExpString>,
        pub igmpv2_support: Option<crate::value::ExpString>,
        pub static_sources_support: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_TransitGatewayMulticastDomain_Options {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::TransitGatewayMulticastDomain.Options"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_TransitGatewayMulticastDomain_Options as Options;
    impl crate::value::ToValue for Options_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_accept_shared_associations {
                properties.insert(
                    "AutoAcceptSharedAssociations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.igmpv2_support {
                properties.insert(
                    "Igmpv2Support".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.static_sources_support {
                properties.insert(
                    "StaticSourcesSupport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod transitgatewaypeeringattachment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-transitgatewaypeeringattachment-peeringattachmentstatus.html
    pub struct PeeringAttachmentStatus_ {
        pub code: Option<crate::value::ExpString>,
        pub message: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_TransitGatewayPeeringAttachment_PeeringAttachmentStatus {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::TransitGatewayPeeringAttachment.PeeringAttachmentStatus"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_TransitGatewayPeeringAttachment_PeeringAttachmentStatus as PeeringAttachmentStatus;
    impl crate::value::ToValue for PeeringAttachmentStatus_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.code {
                properties.insert("Code".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.message {
                properties.insert(
                    "Message".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod transitgatewayvpcattachment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-transitgatewayvpcattachment-options.html
    pub struct Options_ {
        pub appliance_mode_support: Option<crate::value::ExpString>,
        pub dns_support: Option<crate::value::ExpString>,
        pub ipv6_support: Option<crate::value::ExpString>,
        pub security_group_referencing_support: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_TransitGatewayVpcAttachment_Options {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::TransitGatewayVpcAttachment.Options"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_TransitGatewayVpcAttachment_Options as Options;
    impl crate::value::ToValue for Options_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.appliance_mode_support {
                properties.insert(
                    "ApplianceModeSupport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dns_support {
                properties.insert(
                    "DnsSupport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv6_support {
                properties.insert(
                    "Ipv6Support".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_group_referencing_support {
                properties.insert(
                    "SecurityGroupReferencingSupport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod vpcencryptioncontrol {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-vpcencryptioncontrol-resourceexclusions.html
    pub struct ResourceExclusions_ {
        pub egress_only_internet_gateway: Option<Box<VpcEncryptionControlExclusion_>>,
        pub elastic_file_system: Option<Box<VpcEncryptionControlExclusion_>>,
        pub internet_gateway: Option<Box<VpcEncryptionControlExclusion_>>,
        pub lambda: Option<Box<VpcEncryptionControlExclusion_>>,
        pub nat_gateway: Option<Box<VpcEncryptionControlExclusion_>>,
        pub virtual_private_gateway: Option<Box<VpcEncryptionControlExclusion_>>,
        pub vpc_lattice: Option<Box<VpcEncryptionControlExclusion_>>,
        pub vpc_peering: Option<Box<VpcEncryptionControlExclusion_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VPCEncryptionControl_ResourceExclusions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VPCEncryptionControl.ResourceExclusions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VPCEncryptionControl_ResourceExclusions as ResourceExclusions;
    impl crate::value::ToValue for ResourceExclusions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.egress_only_internet_gateway {
                properties.insert(
                    "EgressOnlyInternetGateway".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.elastic_file_system {
                properties.insert(
                    "ElasticFileSystem".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.internet_gateway {
                properties.insert(
                    "InternetGateway".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda {
                properties.insert("Lambda".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.nat_gateway {
                properties.insert(
                    "NatGateway".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.virtual_private_gateway {
                properties.insert(
                    "VirtualPrivateGateway".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_lattice {
                properties.insert(
                    "VpcLattice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_peering {
                properties.insert(
                    "VpcPeering".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-vpcencryptioncontrol-vpcencryptioncontrolexclusion.html
    pub struct VpcEncryptionControlExclusion_ {
        pub state: Option<crate::value::ExpString>,
        pub state_message: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VPCEncryptionControl_VpcEncryptionControlExclusion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VPCEncryptionControl.VpcEncryptionControlExclusion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VPCEncryptionControl_VpcEncryptionControlExclusion as VpcEncryptionControlExclusion;
    impl crate::value::ToValue for VpcEncryptionControlExclusion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.state_message {
                properties.insert(
                    "StateMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod vpcendpoint {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-vpcendpoint-dnsoptionsspecification.html
    pub struct DnsOptionsSpecification_ {
        pub dns_record_ip_type: Option<crate::value::ExpString>,
        pub private_dns_only_for_inbound_resolver_endpoint: Option<crate::value::ExpString>,
        pub private_dns_preference: Option<crate::value::ExpString>,
        pub private_dns_specified_domains: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VPCEndpoint_DnsOptionsSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VPCEndpoint.DnsOptionsSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VPCEndpoint_DnsOptionsSpecification as DnsOptionsSpecification;
    impl crate::value::ToValue for DnsOptionsSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dns_record_ip_type {
                properties.insert(
                    "DnsRecordIpType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.private_dns_only_for_inbound_resolver_endpoint {
                properties.insert(
                    "PrivateDnsOnlyForInboundResolverEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.private_dns_preference {
                properties.insert(
                    "PrivateDnsPreference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.private_dns_specified_domains {
                properties.insert(
                    "PrivateDnsSpecifiedDomains".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod vpnconnection {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-vpnconnection-cloudwatchlogoptionsspecification.html
    pub struct CloudwatchLogOptionsSpecification_ {
        pub bgp_log_enabled: Option<crate::value::ExpBool>,
        pub bgp_log_group_arn: Option<crate::value::ExpString>,
        pub bgp_log_output_format: Option<crate::value::ExpString>,
        pub log_enabled: Option<crate::value::ExpBool>,
        pub log_group_arn: Option<crate::value::ExpString>,
        pub log_output_format: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VPNConnection_CloudwatchLogOptionsSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VPNConnection.CloudwatchLogOptionsSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VPNConnection_CloudwatchLogOptionsSpecification as CloudwatchLogOptionsSpecification;
    impl crate::value::ToValue for CloudwatchLogOptionsSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bgp_log_enabled {
                properties.insert(
                    "BgpLogEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bgp_log_group_arn {
                properties.insert(
                    "BgpLogGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bgp_log_output_format {
                properties.insert(
                    "BgpLogOutputFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_enabled {
                properties.insert(
                    "LogEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_group_arn {
                properties.insert(
                    "LogGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_output_format {
                properties.insert(
                    "LogOutputFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-vpnconnection-ikeversionsrequestlistvalue.html
    pub struct IKEVersionsRequestListValue_ {
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VPNConnection_IKEVersionsRequestListValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VPNConnection.IKEVersionsRequestListValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VPNConnection_IKEVersionsRequestListValue as IKEVersionsRequestListValue;
    impl crate::value::ToValue for IKEVersionsRequestListValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-vpnconnection-phase1dhgroupnumbersrequestlistvalue.html
    pub struct Phase1DHGroupNumbersRequestListValue_ {
        pub value: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VPNConnection_Phase1DHGroupNumbersRequestListValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VPNConnection.Phase1DHGroupNumbersRequestListValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VPNConnection_Phase1DHGroupNumbersRequestListValue as Phase1DHGroupNumbersRequestListValue;
    impl crate::value::ToValue for Phase1DHGroupNumbersRequestListValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-vpnconnection-phase1encryptionalgorithmsrequestlistvalue.html
    pub struct Phase1EncryptionAlgorithmsRequestListValue_ {
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VPNConnection_Phase1EncryptionAlgorithmsRequestListValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VPNConnection.Phase1EncryptionAlgorithmsRequestListValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VPNConnection_Phase1EncryptionAlgorithmsRequestListValue as Phase1EncryptionAlgorithmsRequestListValue;
    impl crate::value::ToValue for Phase1EncryptionAlgorithmsRequestListValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-vpnconnection-phase1integrityalgorithmsrequestlistvalue.html
    pub struct Phase1IntegrityAlgorithmsRequestListValue_ {
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VPNConnection_Phase1IntegrityAlgorithmsRequestListValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VPNConnection.Phase1IntegrityAlgorithmsRequestListValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VPNConnection_Phase1IntegrityAlgorithmsRequestListValue as Phase1IntegrityAlgorithmsRequestListValue;
    impl crate::value::ToValue for Phase1IntegrityAlgorithmsRequestListValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-vpnconnection-phase2dhgroupnumbersrequestlistvalue.html
    pub struct Phase2DHGroupNumbersRequestListValue_ {
        pub value: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VPNConnection_Phase2DHGroupNumbersRequestListValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VPNConnection.Phase2DHGroupNumbersRequestListValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VPNConnection_Phase2DHGroupNumbersRequestListValue as Phase2DHGroupNumbersRequestListValue;
    impl crate::value::ToValue for Phase2DHGroupNumbersRequestListValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-vpnconnection-phase2encryptionalgorithmsrequestlistvalue.html
    pub struct Phase2EncryptionAlgorithmsRequestListValue_ {
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VPNConnection_Phase2EncryptionAlgorithmsRequestListValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VPNConnection.Phase2EncryptionAlgorithmsRequestListValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VPNConnection_Phase2EncryptionAlgorithmsRequestListValue as Phase2EncryptionAlgorithmsRequestListValue;
    impl crate::value::ToValue for Phase2EncryptionAlgorithmsRequestListValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-vpnconnection-phase2integrityalgorithmsrequestlistvalue.html
    pub struct Phase2IntegrityAlgorithmsRequestListValue_ {
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VPNConnection_Phase2IntegrityAlgorithmsRequestListValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VPNConnection.Phase2IntegrityAlgorithmsRequestListValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VPNConnection_Phase2IntegrityAlgorithmsRequestListValue as Phase2IntegrityAlgorithmsRequestListValue;
    impl crate::value::ToValue for Phase2IntegrityAlgorithmsRequestListValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-vpnconnection-vpntunnellogoptionsspecification.html
    pub struct VpnTunnelLogOptionsSpecification_ {
        pub cloudwatch_log_options: Option<Box<CloudwatchLogOptionsSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VPNConnection_VpnTunnelLogOptionsSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VPNConnection.VpnTunnelLogOptionsSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VPNConnection_VpnTunnelLogOptionsSpecification as VpnTunnelLogOptionsSpecification;
    impl crate::value::ToValue for VpnTunnelLogOptionsSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloudwatch_log_options {
                properties.insert(
                    "CloudwatchLogOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-vpnconnection-vpntunneloptionsspecification.html
    pub struct VpnTunnelOptionsSpecification_ {
        pub dpd_timeout_action: Option<crate::value::ExpString>,
        pub dpd_timeout_seconds: Option<i32>,
        pub enable_tunnel_lifecycle_control: Option<crate::value::ExpBool>,
        pub ike_versions: Option<Vec<IKEVersionsRequestListValue_>>,
        pub log_options: Option<Box<VpnTunnelLogOptionsSpecification_>>,
        pub phase1_dh_group_numbers: Option<Vec<Phase1DHGroupNumbersRequestListValue_>>,
        pub phase1_encryption_algorithms: Option<Vec<Phase1EncryptionAlgorithmsRequestListValue_>>,
        pub phase1_integrity_algorithms: Option<Vec<Phase1IntegrityAlgorithmsRequestListValue_>>,
        pub phase1_lifetime_seconds: Option<i32>,
        pub phase2_dh_group_numbers: Option<Vec<Phase2DHGroupNumbersRequestListValue_>>,
        pub phase2_encryption_algorithms: Option<Vec<Phase2EncryptionAlgorithmsRequestListValue_>>,
        pub phase2_integrity_algorithms: Option<Vec<Phase2IntegrityAlgorithmsRequestListValue_>>,
        pub phase2_lifetime_seconds: Option<i32>,
        pub pre_shared_key: Option<crate::value::ExpString>,
        pub rekey_fuzz_percentage: Option<i32>,
        pub rekey_margin_time_seconds: Option<i32>,
        pub replay_window_size: Option<i32>,
        pub startup_action: Option<crate::value::ExpString>,
        pub tunnel_inside_cidr: Option<crate::value::ExpString>,
        pub tunnel_inside_ipv6_cidr: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VPNConnection_VpnTunnelOptionsSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VPNConnection.VpnTunnelOptionsSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VPNConnection_VpnTunnelOptionsSpecification as VpnTunnelOptionsSpecification;
    impl crate::value::ToValue for VpnTunnelOptionsSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dpd_timeout_action {
                properties.insert(
                    "DPDTimeoutAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dpd_timeout_seconds {
                properties.insert(
                    "DPDTimeoutSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_tunnel_lifecycle_control {
                properties.insert(
                    "EnableTunnelLifecycleControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ike_versions {
                properties.insert(
                    "IKEVersions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_options {
                properties.insert(
                    "LogOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.phase1_dh_group_numbers {
                properties.insert(
                    "Phase1DHGroupNumbers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.phase1_encryption_algorithms {
                properties.insert(
                    "Phase1EncryptionAlgorithms".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.phase1_integrity_algorithms {
                properties.insert(
                    "Phase1IntegrityAlgorithms".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.phase1_lifetime_seconds {
                properties.insert(
                    "Phase1LifetimeSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.phase2_dh_group_numbers {
                properties.insert(
                    "Phase2DHGroupNumbers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.phase2_encryption_algorithms {
                properties.insert(
                    "Phase2EncryptionAlgorithms".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.phase2_integrity_algorithms {
                properties.insert(
                    "Phase2IntegrityAlgorithms".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.phase2_lifetime_seconds {
                properties.insert(
                    "Phase2LifetimeSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pre_shared_key {
                properties.insert(
                    "PreSharedKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rekey_fuzz_percentage {
                properties.insert(
                    "RekeyFuzzPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rekey_margin_time_seconds {
                properties.insert(
                    "RekeyMarginTimeSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.replay_window_size {
                properties.insert(
                    "ReplayWindowSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.startup_action {
                properties.insert(
                    "StartupAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tunnel_inside_cidr {
                properties.insert(
                    "TunnelInsideCidr".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tunnel_inside_ipv6_cidr {
                properties.insert(
                    "TunnelInsideIpv6Cidr".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod verifiedaccessendpoint {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-verifiedaccessendpoint-cidroptions.html
    pub struct CidrOptions_ {
        pub cidr: Option<crate::value::ExpString>,
        pub port_ranges: Option<Vec<PortRange_>>,
        pub protocol: Option<crate::value::ExpString>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VerifiedAccessEndpoint_CidrOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VerifiedAccessEndpoint.CidrOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VerifiedAccessEndpoint_CidrOptions as CidrOptions;
    impl crate::value::ToValue for CidrOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cidr {
                properties.insert("Cidr".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.port_ranges {
                properties.insert(
                    "PortRanges".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnet_ids {
                properties.insert(
                    "SubnetIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-verifiedaccessendpoint-loadbalanceroptions.html
    pub struct LoadBalancerOptions_ {
        pub load_balancer_arn: Option<crate::value::ExpString>,
        pub port: Option<i32>,
        pub port_ranges: Option<Vec<PortRange_>>,
        pub protocol: Option<crate::value::ExpString>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VerifiedAccessEndpoint_LoadBalancerOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VerifiedAccessEndpoint.LoadBalancerOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VerifiedAccessEndpoint_LoadBalancerOptions as LoadBalancerOptions;
    impl crate::value::ToValue for LoadBalancerOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.load_balancer_arn {
                properties.insert(
                    "LoadBalancerArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.port_ranges {
                properties.insert(
                    "PortRanges".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnet_ids {
                properties.insert(
                    "SubnetIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-verifiedaccessendpoint-networkinterfaceoptions.html
    pub struct NetworkInterfaceOptions_ {
        pub network_interface_id: Option<crate::value::ExpString>,
        pub port: Option<i32>,
        pub port_ranges: Option<Vec<PortRange_>>,
        pub protocol: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VerifiedAccessEndpoint_NetworkInterfaceOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VerifiedAccessEndpoint.NetworkInterfaceOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VerifiedAccessEndpoint_NetworkInterfaceOptions as NetworkInterfaceOptions;
    impl crate::value::ToValue for NetworkInterfaceOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.network_interface_id {
                properties.insert(
                    "NetworkInterfaceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.port_ranges {
                properties.insert(
                    "PortRanges".to_string(),
                    crate::value::ToValue::to_value(value),
                );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-verifiedaccessendpoint-portrange.html
    pub struct PortRange_ {
        pub from_port: Option<i32>,
        pub to_port: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VerifiedAccessEndpoint_PortRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VerifiedAccessEndpoint.PortRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VerifiedAccessEndpoint_PortRange as PortRange;
    impl crate::value::ToValue for PortRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.from_port {
                properties.insert(
                    "FromPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.to_port {
                properties.insert("ToPort".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-verifiedaccessendpoint-rdsoptions.html
    pub struct RdsOptions_ {
        pub port: Option<i32>,
        pub protocol: Option<crate::value::ExpString>,
        pub rds_db_cluster_arn: Option<crate::value::ExpString>,
        pub rds_db_instance_arn: Option<crate::value::ExpString>,
        pub rds_db_proxy_arn: Option<crate::value::ExpString>,
        pub rds_endpoint: Option<crate::value::ExpString>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VerifiedAccessEndpoint_RdsOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VerifiedAccessEndpoint.RdsOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VerifiedAccessEndpoint_RdsOptions as RdsOptions;
    impl crate::value::ToValue for RdsOptions_ {
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
            if let Some(ref value) = self.rds_db_cluster_arn {
                properties.insert(
                    "RdsDbClusterArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rds_db_instance_arn {
                properties.insert(
                    "RdsDbInstanceArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rds_db_proxy_arn {
                properties.insert(
                    "RdsDbProxyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rds_endpoint {
                properties.insert(
                    "RdsEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnet_ids {
                properties.insert(
                    "SubnetIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-verifiedaccessendpoint-ssespecification.html
    pub struct SseSpecification_ {
        pub customer_managed_key_enabled: Option<crate::value::ExpBool>,
        pub kms_key_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VerifiedAccessEndpoint_SseSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VerifiedAccessEndpoint.SseSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VerifiedAccessEndpoint_SseSpecification as SseSpecification;
    impl crate::value::ToValue for SseSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customer_managed_key_enabled {
                properties.insert(
                    "CustomerManagedKeyEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod verifiedaccessgroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-verifiedaccessgroup-ssespecification.html
    pub struct SseSpecification_ {
        pub customer_managed_key_enabled: Option<crate::value::ExpBool>,
        pub kms_key_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VerifiedAccessGroup_SseSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VerifiedAccessGroup.SseSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VerifiedAccessGroup_SseSpecification as SseSpecification;
    impl crate::value::ToValue for SseSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customer_managed_key_enabled {
                properties.insert(
                    "CustomerManagedKeyEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod verifiedaccessinstance {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-verifiedaccessinstance-cloudwatchlogs.html
    pub struct CloudWatchLogs_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub log_group: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VerifiedAccessInstance_CloudWatchLogs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VerifiedAccessInstance.CloudWatchLogs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VerifiedAccessInstance_CloudWatchLogs as CloudWatchLogs;
    impl crate::value::ToValue for CloudWatchLogs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_group {
                properties.insert(
                    "LogGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-verifiedaccessinstance-kinesisdatafirehose.html
    pub struct KinesisDataFirehose_ {
        pub delivery_stream: Option<crate::value::ExpString>,
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VerifiedAccessInstance_KinesisDataFirehose {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VerifiedAccessInstance.KinesisDataFirehose"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VerifiedAccessInstance_KinesisDataFirehose as KinesisDataFirehose;
    impl crate::value::ToValue for KinesisDataFirehose_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delivery_stream {
                properties.insert(
                    "DeliveryStream".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-verifiedaccessinstance-s3.html
    pub struct S3_ {
        pub bucket_name: Option<crate::value::ExpString>,
        pub bucket_owner: Option<crate::value::ExpString>,
        pub enabled: Option<crate::value::ExpBool>,
        pub prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VerifiedAccessInstance_S3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VerifiedAccessInstance.S3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VerifiedAccessInstance_S3 as S3;
    impl crate::value::ToValue for S3_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_name {
                properties.insert(
                    "BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bucket_owner {
                properties.insert(
                    "BucketOwner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-verifiedaccessinstance-verifiedaccesslogs.html
    pub struct VerifiedAccessLogs_ {
        pub cloud_watch_logs: Option<Box<CloudWatchLogs_>>,
        pub include_trust_context: Option<crate::value::ExpBool>,
        pub kinesis_data_firehose: Option<Box<KinesisDataFirehose_>>,
        pub log_version: Option<crate::value::ExpString>,
        pub s3: Option<Box<S3_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VerifiedAccessInstance_VerifiedAccessLogs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VerifiedAccessInstance.VerifiedAccessLogs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VerifiedAccessInstance_VerifiedAccessLogs as VerifiedAccessLogs;
    impl crate::value::ToValue for VerifiedAccessLogs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_logs {
                properties.insert(
                    "CloudWatchLogs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_trust_context {
                properties.insert(
                    "IncludeTrustContext".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kinesis_data_firehose {
                properties.insert(
                    "KinesisDataFirehose".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_version {
                properties.insert(
                    "LogVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-verifiedaccessinstance-verifiedaccesstrustprovider.html
    pub struct VerifiedAccessTrustProvider_ {
        pub description: Option<crate::value::ExpString>,
        pub device_trust_provider_type: Option<crate::value::ExpString>,
        pub trust_provider_type: Option<crate::value::ExpString>,
        pub user_trust_provider_type: Option<crate::value::ExpString>,
        pub verified_access_trust_provider_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VerifiedAccessInstance_VerifiedAccessTrustProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VerifiedAccessInstance.VerifiedAccessTrustProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VerifiedAccessInstance_VerifiedAccessTrustProvider as VerifiedAccessTrustProvider;
    impl crate::value::ToValue for VerifiedAccessTrustProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.device_trust_provider_type {
                properties.insert(
                    "DeviceTrustProviderType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.trust_provider_type {
                properties.insert(
                    "TrustProviderType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_trust_provider_type {
                properties.insert(
                    "UserTrustProviderType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.verified_access_trust_provider_id {
                properties.insert(
                    "VerifiedAccessTrustProviderId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod verifiedaccesstrustprovider {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-verifiedaccesstrustprovider-deviceoptions.html
    pub struct DeviceOptions_ {
        pub public_signing_key_url: Option<crate::value::ExpString>,
        pub tenant_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VerifiedAccessTrustProvider_DeviceOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VerifiedAccessTrustProvider.DeviceOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VerifiedAccessTrustProvider_DeviceOptions as DeviceOptions;
    impl crate::value::ToValue for DeviceOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.public_signing_key_url {
                properties.insert(
                    "PublicSigningKeyUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tenant_id {
                properties.insert(
                    "TenantId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-verifiedaccesstrustprovider-nativeapplicationoidcoptions.html
    pub struct NativeApplicationOidcOptions_ {
        pub authorization_endpoint: Option<crate::value::ExpString>,
        pub client_id: Option<crate::value::ExpString>,
        pub client_secret: Option<crate::value::ExpString>,
        pub issuer: Option<crate::value::ExpString>,
        pub public_signing_key_endpoint: Option<crate::value::ExpString>,
        pub scope: Option<crate::value::ExpString>,
        pub token_endpoint: Option<crate::value::ExpString>,
        pub user_info_endpoint: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VerifiedAccessTrustProvider_NativeApplicationOidcOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VerifiedAccessTrustProvider.NativeApplicationOidcOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VerifiedAccessTrustProvider_NativeApplicationOidcOptions as NativeApplicationOidcOptions;
    impl crate::value::ToValue for NativeApplicationOidcOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authorization_endpoint {
                properties.insert(
                    "AuthorizationEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_id {
                properties.insert(
                    "ClientId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_secret {
                properties.insert(
                    "ClientSecret".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.issuer {
                properties.insert("Issuer".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.public_signing_key_endpoint {
                properties.insert(
                    "PublicSigningKeyEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scope {
                properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.token_endpoint {
                properties.insert(
                    "TokenEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_info_endpoint {
                properties.insert(
                    "UserInfoEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-verifiedaccesstrustprovider-oidcoptions.html
    pub struct OidcOptions_ {
        pub authorization_endpoint: Option<crate::value::ExpString>,
        pub client_id: Option<crate::value::ExpString>,
        pub client_secret: Option<crate::value::ExpString>,
        pub issuer: Option<crate::value::ExpString>,
        pub scope: Option<crate::value::ExpString>,
        pub token_endpoint: Option<crate::value::ExpString>,
        pub user_info_endpoint: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VerifiedAccessTrustProvider_OidcOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VerifiedAccessTrustProvider.OidcOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VerifiedAccessTrustProvider_OidcOptions as OidcOptions;
    impl crate::value::ToValue for OidcOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authorization_endpoint {
                properties.insert(
                    "AuthorizationEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_id {
                properties.insert(
                    "ClientId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_secret {
                properties.insert(
                    "ClientSecret".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.issuer {
                properties.insert("Issuer".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.scope {
                properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.token_endpoint {
                properties.insert(
                    "TokenEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_info_endpoint {
                properties.insert(
                    "UserInfoEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ec2-verifiedaccesstrustprovider-ssespecification.html
    pub struct SseSpecification_ {
        pub customer_managed_key_enabled: Option<crate::value::ExpBool>,
        pub kms_key_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ec2_VerifiedAccessTrustProvider_SseSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EC2::VerifiedAccessTrustProvider.SseSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ec2_VerifiedAccessTrustProvider_SseSpecification as SseSpecification;
    impl crate::value::ToValue for SseSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customer_managed_key_enabled {
                properties.insert(
                    "CustomerManagedKeyEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-capacitymanagerdataexport.html
pub struct CapacityManagerDataExport_ {
    pub output_format: crate::value::ExpString,
    pub s3_bucket_name: crate::value::ExpString,
    pub s3_bucket_prefix: Option<crate::value::ExpString>,
    pub schedule: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_CapacityManagerDataExport {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::CapacityManagerDataExport"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_CapacityManagerDataExport as CapacityManagerDataExport;
impl crate::template::ToResource for CapacityManagerDataExport_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CapacityManagerDataExport"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "OutputFormat".to_string(),
            crate::value::ToValue::to_value(&self.output_format),
        );
        properties.insert(
            "S3BucketName".to_string(),
            crate::value::ToValue::to_value(&self.s3_bucket_name),
        );
        if let Some(ref value) = self.s3_bucket_prefix {
            properties.insert(
                "S3BucketPrefix".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Schedule".to_string(),
            crate::value::ToValue::to_value(&self.schedule),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-capacityreservation.html
pub struct CapacityReservation_ {
    pub availability_zone: Option<crate::value::ExpString>,
    pub availability_zone_id: Option<crate::value::ExpString>,
    pub ebs_optimized: Option<crate::value::ExpBool>,
    pub end_date: Option<crate::value::ExpString>,
    pub end_date_type: Option<crate::value::ExpString>,
    pub ephemeral_storage: Option<crate::value::ExpBool>,
    pub instance_count: i32,
    pub instance_match_criteria: Option<crate::value::ExpString>,
    pub instance_platform: crate::value::ExpString,
    pub instance_type: crate::value::ExpString,
    pub out_post_arn: Option<crate::value::ExpString>,
    pub placement_group_arn: Option<crate::value::ExpString>,
    pub tag_specifications: Option<Vec<super::ec2::capacityreservation::TagSpecification_>>,
    pub tenancy: Option<crate::value::ExpString>,
    pub unused_reservation_billing_owner_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_CapacityReservation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::CapacityReservation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_CapacityReservation as CapacityReservation;
impl crate::template::ToResource for CapacityReservation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CapacityReservation"),
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
        if let Some(ref value) = self.ebs_optimized {
            properties.insert(
                "EbsOptimized".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.end_date {
            properties.insert(
                "EndDate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.end_date_type {
            properties.insert(
                "EndDateType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ephemeral_storage {
            properties.insert(
                "EphemeralStorage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceCount".to_string(),
            crate::value::ToValue::to_value(&self.instance_count),
        );
        if let Some(ref value) = self.instance_match_criteria {
            properties.insert(
                "InstanceMatchCriteria".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstancePlatform".to_string(),
            crate::value::ToValue::to_value(&self.instance_platform),
        );
        properties.insert(
            "InstanceType".to_string(),
            crate::value::ToValue::to_value(&self.instance_type),
        );
        if let Some(ref value) = self.out_post_arn {
            properties.insert(
                "OutPostArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.placement_group_arn {
            properties.insert(
                "PlacementGroupArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tag_specifications {
            properties.insert(
                "TagSpecifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tenancy {
            properties.insert(
                "Tenancy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.unused_reservation_billing_owner_id {
            properties.insert(
                "UnusedReservationBillingOwnerId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-capacityreservationfleet.html
pub struct CapacityReservationFleet_ {
    pub allocation_strategy: Option<crate::value::ExpString>,
    pub end_date: Option<crate::value::ExpString>,
    pub instance_match_criteria: Option<crate::value::ExpString>,
    pub instance_type_specifications:
        Option<Vec<super::ec2::capacityreservationfleet::InstanceTypeSpecification_>>,
    pub no_remove_end_date: Option<crate::value::ExpBool>,
    pub remove_end_date: Option<crate::value::ExpBool>,
    pub tag_specifications: Option<Vec<super::ec2::capacityreservationfleet::TagSpecification_>>,
    pub tenancy: Option<crate::value::ExpString>,
    pub total_target_capacity: Option<i32>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_CapacityReservationFleet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::CapacityReservationFleet"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_CapacityReservationFleet as CapacityReservationFleet;
impl crate::template::ToResource for CapacityReservationFleet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CapacityReservationFleet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allocation_strategy {
            properties.insert(
                "AllocationStrategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.end_date {
            properties.insert(
                "EndDate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_match_criteria {
            properties.insert(
                "InstanceMatchCriteria".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_type_specifications {
            properties.insert(
                "InstanceTypeSpecifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.no_remove_end_date {
            properties.insert(
                "NoRemoveEndDate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.remove_end_date {
            properties.insert(
                "RemoveEndDate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tag_specifications {
            properties.insert(
                "TagSpecifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tenancy {
            properties.insert(
                "Tenancy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.total_target_capacity {
            properties.insert(
                "TotalTargetCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-carriergateway.html
pub struct CarrierGateway_ {
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_CarrierGateway {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::CarrierGateway"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_CarrierGateway as CarrierGateway;
impl crate::template::ToResource for CarrierGateway_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CarrierGateway"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-clientvpnauthorizationrule.html
pub struct ClientVpnAuthorizationRule_ {
    pub access_group_id: Option<crate::value::ExpString>,
    pub authorize_all_groups: Option<crate::value::ExpBool>,
    pub client_vpn_endpoint_id: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub target_network_cidr: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_ClientVpnAuthorizationRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::ClientVpnAuthorizationRule"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_ClientVpnAuthorizationRule as ClientVpnAuthorizationRule;
impl crate::template::ToResource for ClientVpnAuthorizationRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ClientVpnAuthorizationRule",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_group_id {
            properties.insert(
                "AccessGroupId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.authorize_all_groups {
            properties.insert(
                "AuthorizeAllGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ClientVpnEndpointId".to_string(),
            crate::value::ToValue::to_value(&self.client_vpn_endpoint_id),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TargetNetworkCidr".to_string(),
            crate::value::ToValue::to_value(&self.target_network_cidr),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-clientvpnendpoint.html
pub struct ClientVpnEndpoint_ {
    pub authentication_options: Vec<super::ec2::clientvpnendpoint::ClientAuthenticationRequest_>,
    pub client_cidr_block: Option<crate::value::ExpString>,
    pub client_connect_options: Option<super::ec2::clientvpnendpoint::ClientConnectOptions_>,
    pub client_login_banner_options:
        Option<super::ec2::clientvpnendpoint::ClientLoginBannerOptions_>,
    pub client_route_enforcement_options:
        Option<super::ec2::clientvpnendpoint::ClientRouteEnforcementOptions_>,
    pub connection_log_options: super::ec2::clientvpnendpoint::ConnectionLogOptions_,
    pub description: Option<crate::value::ExpString>,
    pub disconnect_on_session_timeout: Option<crate::value::ExpBool>,
    pub dns_servers: Option<Vec<crate::value::ExpString>>,
    pub endpoint_ip_address_type: Option<crate::value::ExpString>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub self_service_portal: Option<crate::value::ExpString>,
    pub server_certificate_arn: crate::value::ExpString,
    pub session_timeout_hours: Option<i32>,
    pub split_tunnel: Option<crate::value::ExpBool>,
    pub tag_specifications: Option<Vec<super::ec2::clientvpnendpoint::TagSpecification_>>,
    pub traffic_ip_address_type: Option<crate::value::ExpString>,
    pub transport_protocol: Option<crate::value::ExpString>,
    pub vpc_id: Option<crate::value::ExpString>,
    pub vpn_port: Option<i32>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_ClientVpnEndpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::ClientVpnEndpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_ClientVpnEndpoint as ClientVpnEndpoint;
impl crate::template::ToResource for ClientVpnEndpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ClientVpnEndpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AuthenticationOptions".to_string(),
            crate::value::ToValue::to_value(&self.authentication_options),
        );
        if let Some(ref value) = self.client_cidr_block {
            properties.insert(
                "ClientCidrBlock".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.client_connect_options {
            properties.insert(
                "ClientConnectOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.client_login_banner_options {
            properties.insert(
                "ClientLoginBannerOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.client_route_enforcement_options {
            properties.insert(
                "ClientRouteEnforcementOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ConnectionLogOptions".to_string(),
            crate::value::ToValue::to_value(&self.connection_log_options),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.disconnect_on_session_timeout {
            properties.insert(
                "DisconnectOnSessionTimeout".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dns_servers {
            properties.insert(
                "DnsServers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint_ip_address_type {
            properties.insert(
                "EndpointIpAddressType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.self_service_portal {
            properties.insert(
                "SelfServicePortal".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ServerCertificateArn".to_string(),
            crate::value::ToValue::to_value(&self.server_certificate_arn),
        );
        if let Some(ref value) = self.session_timeout_hours {
            properties.insert(
                "SessionTimeoutHours".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.split_tunnel {
            properties.insert(
                "SplitTunnel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tag_specifications {
            properties.insert(
                "TagSpecifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.traffic_ip_address_type {
            properties.insert(
                "TrafficIpAddressType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.transport_protocol {
            properties.insert(
                "TransportProtocol".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_id {
            properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpn_port {
            properties.insert(
                "VpnPort".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-clientvpnroute.html
pub struct ClientVpnRoute_ {
    pub client_vpn_endpoint_id: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub destination_cidr_block: crate::value::ExpString,
    pub target_vpc_subnet_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_ClientVpnRoute {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::ClientVpnRoute"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_ClientVpnRoute as ClientVpnRoute;
impl crate::template::ToResource for ClientVpnRoute_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ClientVpnRoute"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ClientVpnEndpointId".to_string(),
            crate::value::ToValue::to_value(&self.client_vpn_endpoint_id),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DestinationCidrBlock".to_string(),
            crate::value::ToValue::to_value(&self.destination_cidr_block),
        );
        properties.insert(
            "TargetVpcSubnetId".to_string(),
            crate::value::ToValue::to_value(&self.target_vpc_subnet_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-clientvpntargetnetworkassociation.html
pub struct ClientVpnTargetNetworkAssociation_ {
    pub client_vpn_endpoint_id: crate::value::ExpString,
    pub subnet_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_ClientVpnTargetNetworkAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::ClientVpnTargetNetworkAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_ClientVpnTargetNetworkAssociation as ClientVpnTargetNetworkAssociation;
impl crate::template::ToResource for ClientVpnTargetNetworkAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ClientVpnTargetNetworkAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ClientVpnEndpointId".to_string(),
            crate::value::ToValue::to_value(&self.client_vpn_endpoint_id),
        );
        properties.insert(
            "SubnetId".to_string(),
            crate::value::ToValue::to_value(&self.subnet_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-customergateway.html
pub struct CustomerGateway_ {
    pub bgp_asn: Option<i32>,
    pub bgp_asn_extended: Option<f64>,
    pub certificate_arn: Option<crate::value::ExpString>,
    pub device_name: Option<crate::value::ExpString>,
    pub ip_address: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_CustomerGateway {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::CustomerGateway"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_CustomerGateway as CustomerGateway;
impl crate::template::ToResource for CustomerGateway_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CustomerGateway"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.bgp_asn {
            properties.insert("BgpAsn".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.bgp_asn_extended {
            properties.insert(
                "BgpAsnExtended".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate_arn {
            properties.insert(
                "CertificateArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.device_name {
            properties.insert(
                "DeviceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IpAddress".to_string(),
            crate::value::ToValue::to_value(&self.ip_address),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-dhcpoptions.html
pub struct DHCPOptions_ {
    pub domain_name: Option<crate::value::ExpString>,
    pub domain_name_servers: Option<Vec<crate::value::ExpString>>,
    pub ipv6_address_preferred_lease_time: Option<i32>,
    pub netbios_name_servers: Option<Vec<crate::value::ExpString>>,
    pub netbios_node_type: Option<i32>,
    pub ntp_servers: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_DHCPOptions {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::DHCPOptions"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_DHCPOptions as DHCPOptions;
impl crate::template::ToResource for DHCPOptions_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DHCPOptions"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.domain_name {
            properties.insert(
                "DomainName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_name_servers {
            properties.insert(
                "DomainNameServers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_address_preferred_lease_time {
            properties.insert(
                "Ipv6AddressPreferredLeaseTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.netbios_name_servers {
            properties.insert(
                "NetbiosNameServers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.netbios_node_type {
            properties.insert(
                "NetbiosNodeType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ntp_servers {
            properties.insert(
                "NtpServers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-ec2fleet.html
pub struct EC2Fleet_ {
    pub context: Option<crate::value::ExpString>,
    pub excess_capacity_termination_policy: Option<crate::value::ExpString>,
    pub launch_template_configs: Vec<super::ec2::ec2fleet::FleetLaunchTemplateConfigRequest_>,
    pub on_demand_options: Option<super::ec2::ec2fleet::OnDemandOptionsRequest_>,
    pub replace_unhealthy_instances: Option<crate::value::ExpBool>,
    pub spot_options: Option<super::ec2::ec2fleet::SpotOptionsRequest_>,
    pub tag_specifications: Option<Vec<super::ec2::ec2fleet::TagSpecification_>>,
    pub target_capacity_specification: super::ec2::ec2fleet::TargetCapacitySpecificationRequest_,
    pub terminate_instances_with_expiration: Option<crate::value::ExpBool>,
    pub r#type: Option<crate::value::ExpString>,
    pub valid_from: Option<crate::value::ExpString>,
    pub valid_until: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_EC2Fleet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::EC2Fleet" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_EC2Fleet as EC2Fleet;
impl crate::template::ToResource for EC2Fleet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EC2Fleet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.context {
            properties.insert(
                "Context".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.excess_capacity_termination_policy {
            properties.insert(
                "ExcessCapacityTerminationPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LaunchTemplateConfigs".to_string(),
            crate::value::ToValue::to_value(&self.launch_template_configs),
        );
        if let Some(ref value) = self.on_demand_options {
            properties.insert(
                "OnDemandOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.replace_unhealthy_instances {
            properties.insert(
                "ReplaceUnhealthyInstances".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.spot_options {
            properties.insert(
                "SpotOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tag_specifications {
            properties.insert(
                "TagSpecifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TargetCapacitySpecification".to_string(),
            crate::value::ToValue::to_value(&self.target_capacity_specification),
        );
        if let Some(ref value) = self.terminate_instances_with_expiration {
            properties.insert(
                "TerminateInstancesWithExpiration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.valid_from {
            properties.insert(
                "ValidFrom".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.valid_until {
            properties.insert(
                "ValidUntil".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-eip.html
pub struct EIP_ {
    pub address: Option<crate::value::ExpString>,
    pub domain: Option<crate::value::ExpString>,
    pub instance_id: Option<crate::value::ExpString>,
    pub ipam_pool_id: Option<crate::value::ExpString>,
    pub network_border_group: Option<crate::value::ExpString>,
    pub public_ipv4_pool: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transfer_address: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_EIP {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::EIP" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_EIP as EIP;
impl crate::template::ToResource for EIP_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EIP"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.address {
            properties.insert(
                "Address".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain {
            properties.insert("Domain".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.instance_id {
            properties.insert(
                "InstanceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipam_pool_id {
            properties.insert(
                "IpamPoolId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_border_group {
            properties.insert(
                "NetworkBorderGroup".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.public_ipv4_pool {
            properties.insert(
                "PublicIpv4Pool".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.transfer_address {
            properties.insert(
                "TransferAddress".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-eipassociation.html
pub struct EIPAssociation_ {
    pub allocation_id: Option<crate::value::ExpString>,
    pub instance_id: Option<crate::value::ExpString>,
    pub network_interface_id: Option<crate::value::ExpString>,
    pub private_ip_address: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_EIPAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::EIPAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_EIPAssociation as EIPAssociation;
impl crate::template::ToResource for EIPAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EIPAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allocation_id {
            properties.insert(
                "AllocationId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_id {
            properties.insert(
                "InstanceId".to_string(),
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
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-egressonlyinternetgateway.html
pub struct EgressOnlyInternetGateway_ {
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_EgressOnlyInternetGateway {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::EgressOnlyInternetGateway"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_EgressOnlyInternetGateway as EgressOnlyInternetGateway;
impl crate::template::ToResource for EgressOnlyInternetGateway_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EgressOnlyInternetGateway"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-enclavecertificateiamroleassociation.html
pub struct EnclaveCertificateIamRoleAssociation_ {
    pub certificate_arn: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_EnclaveCertificateIamRoleAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::EnclaveCertificateIamRoleAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_EnclaveCertificateIamRoleAssociation as EnclaveCertificateIamRoleAssociation;
impl crate::template::ToResource for EnclaveCertificateIamRoleAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "EnclaveCertificateIamRoleAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CertificateArn".to_string(),
            crate::value::ToValue::to_value(&self.certificate_arn),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-flowlog.html
pub struct FlowLog_ {
    pub deliver_cross_account_role: Option<crate::value::ExpString>,
    pub deliver_logs_permission_arn: Option<crate::value::ExpString>,
    pub destination_options: Option<super::ec2::flowlog::DestinationOptions_>,
    pub log_destination: Option<crate::value::ExpString>,
    pub log_destination_type: Option<crate::value::ExpString>,
    pub log_format: Option<crate::value::ExpString>,
    pub log_group_name: Option<crate::value::ExpString>,
    pub max_aggregation_interval: Option<i32>,
    pub resource_id: crate::value::ExpString,
    pub resource_type: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub traffic_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_FlowLog {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::FlowLog" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_FlowLog as FlowLog;
impl crate::template::ToResource for FlowLog_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FlowLog"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.deliver_cross_account_role {
            properties.insert(
                "DeliverCrossAccountRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deliver_logs_permission_arn {
            properties.insert(
                "DeliverLogsPermissionArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destination_options {
            properties.insert(
                "DestinationOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_destination {
            properties.insert(
                "LogDestination".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_destination_type {
            properties.insert(
                "LogDestinationType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_format {
            properties.insert(
                "LogFormat".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_group_name {
            properties.insert(
                "LogGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_aggregation_interval {
            properties.insert(
                "MaxAggregationInterval".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResourceId".to_string(),
            crate::value::ToValue::to_value(&self.resource_id),
        );
        properties.insert(
            "ResourceType".to_string(),
            crate::value::ToValue::to_value(&self.resource_type),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.traffic_type {
            properties.insert(
                "TrafficType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-gatewayroutetableassociation.html
pub struct GatewayRouteTableAssociation_ {
    pub gateway_id: crate::value::ExpString,
    pub route_table_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_GatewayRouteTableAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::GatewayRouteTableAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_GatewayRouteTableAssociation as GatewayRouteTableAssociation;
impl crate::template::ToResource for GatewayRouteTableAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "GatewayRouteTableAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "GatewayId".to_string(),
            crate::value::ToValue::to_value(&self.gateway_id),
        );
        properties.insert(
            "RouteTableId".to_string(),
            crate::value::ToValue::to_value(&self.route_table_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-host.html
pub struct Host_ {
    pub asset_id: Option<crate::value::ExpString>,
    pub auto_placement: Option<crate::value::ExpString>,
    pub availability_zone: crate::value::ExpString,
    pub host_maintenance: Option<crate::value::ExpString>,
    pub host_recovery: Option<crate::value::ExpString>,
    pub instance_family: Option<crate::value::ExpString>,
    pub instance_type: Option<crate::value::ExpString>,
    pub outpost_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_Host {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::Host" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_Host as Host;
impl crate::template::ToResource for Host_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Host"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.asset_id {
            properties.insert(
                "AssetId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_placement {
            properties.insert(
                "AutoPlacement".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AvailabilityZone".to_string(),
            crate::value::ToValue::to_value(&self.availability_zone),
        );
        if let Some(ref value) = self.host_maintenance {
            properties.insert(
                "HostMaintenance".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.host_recovery {
            properties.insert(
                "HostRecovery".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_family {
            properties.insert(
                "InstanceFamily".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_type {
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.outpost_arn {
            properties.insert(
                "OutpostArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-ipam.html
pub struct IPAM_ {
    pub default_resource_discovery_organizational_unit_exclusions:
        Option<Vec<super::ec2::ipam::IpamOrganizationalUnitExclusion_>>,
    pub description: Option<crate::value::ExpString>,
    pub enable_private_gua: Option<crate::value::ExpBool>,
    pub metered_account: Option<crate::value::ExpString>,
    pub operating_regions: Option<Vec<super::ec2::ipam::IpamOperatingRegion_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub tier: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_IPAM {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::IPAM" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_IPAM as IPAM;
impl crate::template::ToResource for IPAM_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IPAM"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.default_resource_discovery_organizational_unit_exclusions {
            properties.insert(
                "DefaultResourceDiscoveryOrganizationalUnitExclusions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_private_gua {
            properties.insert(
                "EnablePrivateGua".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metered_account {
            properties.insert(
                "MeteredAccount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.operating_regions {
            properties.insert(
                "OperatingRegions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tier {
            properties.insert("Tier".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-ipamallocation.html
pub struct IPAMAllocation_ {
    pub cidr: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub ipam_pool_id: crate::value::ExpString,
    pub netmask_length: Option<i32>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_IPAMAllocation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::IPAMAllocation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_IPAMAllocation as IPAMAllocation;
impl crate::template::ToResource for IPAMAllocation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IPAMAllocation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cidr {
            properties.insert("Cidr".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IpamPoolId".to_string(),
            crate::value::ToValue::to_value(&self.ipam_pool_id),
        );
        if let Some(ref value) = self.netmask_length {
            properties.insert(
                "NetmaskLength".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-ipampool.html
pub struct IPAMPool_ {
    pub address_family: crate::value::ExpString,
    pub allocation_default_netmask_length: Option<i32>,
    pub allocation_max_netmask_length: Option<i32>,
    pub allocation_min_netmask_length: Option<i32>,
    pub allocation_resource_tags: Option<Vec<crate::Tag_>>,
    pub auto_import: Option<crate::value::ExpBool>,
    pub aws_service: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub ipam_scope_id: crate::value::ExpString,
    pub locale: Option<crate::value::ExpString>,
    pub provisioned_cidrs: Option<Vec<super::ec2::ipampool::ProvisionedCidr_>>,
    pub public_ip_source: Option<crate::value::ExpString>,
    pub publicly_advertisable: Option<crate::value::ExpBool>,
    pub source_ipam_pool_id: Option<crate::value::ExpString>,
    pub source_resource: Option<super::ec2::ipampool::SourceResource_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_IPAMPool {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::IPAMPool" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_IPAMPool as IPAMPool;
impl crate::template::ToResource for IPAMPool_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IPAMPool"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AddressFamily".to_string(),
            crate::value::ToValue::to_value(&self.address_family),
        );
        if let Some(ref value) = self.allocation_default_netmask_length {
            properties.insert(
                "AllocationDefaultNetmaskLength".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.allocation_max_netmask_length {
            properties.insert(
                "AllocationMaxNetmaskLength".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.allocation_min_netmask_length {
            properties.insert(
                "AllocationMinNetmaskLength".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.allocation_resource_tags {
            properties.insert(
                "AllocationResourceTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_import {
            properties.insert(
                "AutoImport".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.aws_service {
            properties.insert(
                "AwsService".to_string(),
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
            "IpamScopeId".to_string(),
            crate::value::ToValue::to_value(&self.ipam_scope_id),
        );
        if let Some(ref value) = self.locale {
            properties.insert("Locale".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.provisioned_cidrs {
            properties.insert(
                "ProvisionedCidrs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.public_ip_source {
            properties.insert(
                "PublicIpSource".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.publicly_advertisable {
            properties.insert(
                "PubliclyAdvertisable".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_ipam_pool_id {
            properties.insert(
                "SourceIpamPoolId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_resource {
            properties.insert(
                "SourceResource".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-ipampoolcidr.html
pub struct IPAMPoolCidr_ {
    pub cidr: Option<crate::value::ExpString>,
    pub ipam_pool_id: crate::value::ExpString,
    pub netmask_length: Option<i32>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_IPAMPoolCidr {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::IPAMPoolCidr"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_IPAMPoolCidr as IPAMPoolCidr;
impl crate::template::ToResource for IPAMPoolCidr_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IPAMPoolCidr"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cidr {
            properties.insert("Cidr".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "IpamPoolId".to_string(),
            crate::value::ToValue::to_value(&self.ipam_pool_id),
        );
        if let Some(ref value) = self.netmask_length {
            properties.insert(
                "NetmaskLength".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-ipamresourcediscovery.html
pub struct IPAMResourceDiscovery_ {
    pub description: Option<crate::value::ExpString>,
    pub operating_regions: Option<Vec<super::ec2::ipamresourcediscovery::IpamOperatingRegion_>>,
    pub organizational_unit_exclusions: Option<
        Vec<super::ec2::ipamresourcediscovery::IpamResourceDiscoveryOrganizationalUnitExclusion_>,
    >,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_IPAMResourceDiscovery {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::IPAMResourceDiscovery"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_IPAMResourceDiscovery as IPAMResourceDiscovery;
impl crate::template::ToResource for IPAMResourceDiscovery_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IPAMResourceDiscovery"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.operating_regions {
            properties.insert(
                "OperatingRegions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.organizational_unit_exclusions {
            properties.insert(
                "OrganizationalUnitExclusions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-ipamresourcediscoveryassociation.html
pub struct IPAMResourceDiscoveryAssociation_ {
    pub ipam_id: crate::value::ExpString,
    pub ipam_resource_discovery_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_IPAMResourceDiscoveryAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::IPAMResourceDiscoveryAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_IPAMResourceDiscoveryAssociation as IPAMResourceDiscoveryAssociation;
impl crate::template::ToResource for IPAMResourceDiscoveryAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "IPAMResourceDiscoveryAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "IpamId".to_string(),
            crate::value::ToValue::to_value(&self.ipam_id),
        );
        properties.insert(
            "IpamResourceDiscoveryId".to_string(),
            crate::value::ToValue::to_value(&self.ipam_resource_discovery_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-ipamscope.html
pub struct IPAMScope_ {
    pub description: Option<crate::value::ExpString>,
    pub external_authority_configuration:
        Option<super::ec2::ipamscope::IpamScopeExternalAuthorityConfiguration_>,
    pub ipam_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_IPAMScope {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::IPAMScope" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_IPAMScope as IPAMScope;
impl crate::template::ToResource for IPAMScope_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IPAMScope"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.external_authority_configuration {
            properties.insert(
                "ExternalAuthorityConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IpamId".to_string(),
            crate::value::ToValue::to_value(&self.ipam_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-instance.html
pub struct Instance_ {
    pub additional_info: Option<crate::value::ExpString>,
    pub affinity: Option<crate::value::ExpString>,
    pub availability_zone: Option<crate::value::ExpString>,
    pub block_device_mappings: Option<Vec<super::ec2::instance::BlockDeviceMapping_>>,
    pub cpu_options: Option<super::ec2::instance::CpuOptions_>,
    pub credit_specification: Option<super::ec2::instance::CreditSpecification_>,
    pub disable_api_termination: Option<crate::value::ExpBool>,
    pub ebs_optimized: Option<crate::value::ExpBool>,
    pub elastic_gpu_specifications: Option<Vec<super::ec2::instance::ElasticGpuSpecification_>>,
    pub elastic_inference_accelerators:
        Option<Vec<super::ec2::instance::ElasticInferenceAccelerator_>>,
    pub enclave_options: Option<super::ec2::instance::EnclaveOptions_>,
    pub hibernation_options: Option<super::ec2::instance::HibernationOptions_>,
    pub host_id: Option<crate::value::ExpString>,
    pub host_resource_group_arn: Option<crate::value::ExpString>,
    pub iam_instance_profile: Option<crate::value::ExpString>,
    pub image_id: Option<crate::value::ExpString>,
    pub instance_initiated_shutdown_behavior: Option<crate::value::ExpString>,
    pub instance_type: Option<crate::value::ExpString>,
    pub ipv6_address_count: Option<i32>,
    pub ipv6_addresses: Option<Vec<super::ec2::instance::InstanceIpv6Address_>>,
    pub kernel_id: Option<crate::value::ExpString>,
    pub key_name: Option<crate::value::ExpString>,
    pub launch_template: Option<super::ec2::instance::LaunchTemplateSpecification_>,
    pub license_specifications: Option<Vec<super::ec2::instance::LicenseSpecification_>>,
    pub metadata_options: Option<super::ec2::instance::MetadataOptions_>,
    pub monitoring: Option<crate::value::ExpBool>,
    pub network_interfaces: Option<Vec<super::ec2::instance::NetworkInterface_>>,
    pub placement_group_name: Option<crate::value::ExpString>,
    pub private_dns_name_options: Option<super::ec2::instance::PrivateDnsNameOptions_>,
    pub private_ip_address: Option<crate::value::ExpString>,
    pub propagate_tags_to_volume_on_creation: Option<crate::value::ExpBool>,
    pub ramdisk_id: Option<crate::value::ExpString>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub security_groups: Option<Vec<crate::value::ExpString>>,
    pub source_dest_check: Option<crate::value::ExpBool>,
    pub ssm_associations: Option<Vec<super::ec2::instance::SsmAssociation_>>,
    pub subnet_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub tenancy: Option<crate::value::ExpString>,
    pub user_data: Option<crate::value::ExpString>,
    pub volumes: Option<Vec<super::ec2::instance::Volume_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_Instance {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::Instance" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_Instance as Instance;
impl crate::template::ToResource for Instance_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Instance"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_info {
            properties.insert(
                "AdditionalInfo".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.affinity {
            properties.insert(
                "Affinity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zone {
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.block_device_mappings {
            properties.insert(
                "BlockDeviceMappings".to_string(),
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
        if let Some(ref value) = self.disable_api_termination {
            properties.insert(
                "DisableApiTermination".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ebs_optimized {
            properties.insert(
                "EbsOptimized".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.elastic_gpu_specifications {
            properties.insert(
                "ElasticGpuSpecifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.elastic_inference_accelerators {
            properties.insert(
                "ElasticInferenceAccelerators".to_string(),
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
        if let Some(ref value) = self.host_id {
            properties.insert("HostId".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.host_resource_group_arn {
            properties.insert(
                "HostResourceGroupArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iam_instance_profile {
            properties.insert(
                "IamInstanceProfile".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_id {
            properties.insert(
                "ImageId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_initiated_shutdown_behavior {
            properties.insert(
                "InstanceInitiatedShutdownBehavior".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_type {
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_address_count {
            properties.insert(
                "Ipv6AddressCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_addresses {
            properties.insert(
                "Ipv6Addresses".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kernel_id {
            properties.insert(
                "KernelId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.key_name {
            properties.insert(
                "KeyName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.launch_template {
            properties.insert(
                "LaunchTemplate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.license_specifications {
            properties.insert(
                "LicenseSpecifications".to_string(),
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
        if let Some(ref value) = self.placement_group_name {
            properties.insert(
                "PlacementGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.private_dns_name_options {
            properties.insert(
                "PrivateDnsNameOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.private_ip_address {
            properties.insert(
                "PrivateIpAddress".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.propagate_tags_to_volume_on_creation {
            properties.insert(
                "PropagateTagsToVolumeOnCreation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ramdisk_id {
            properties.insert(
                "RamdiskId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_groups {
            properties.insert(
                "SecurityGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_dest_check {
            properties.insert(
                "SourceDestCheck".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ssm_associations {
            properties.insert(
                "SsmAssociations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_id {
            properties.insert(
                "SubnetId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tenancy {
            properties.insert(
                "Tenancy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.user_data {
            properties.insert(
                "UserData".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.volumes {
            properties.insert(
                "Volumes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-instanceconnectendpoint.html
pub struct InstanceConnectEndpoint_ {
    pub client_token: Option<crate::value::ExpString>,
    pub preserve_client_ip: Option<crate::value::ExpBool>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub subnet_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_InstanceConnectEndpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::InstanceConnectEndpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_InstanceConnectEndpoint as InstanceConnectEndpoint;
impl crate::template::ToResource for InstanceConnectEndpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InstanceConnectEndpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.client_token {
            properties.insert(
                "ClientToken".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.preserve_client_ip {
            properties.insert(
                "PreserveClientIp".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SubnetId".to_string(),
            crate::value::ToValue::to_value(&self.subnet_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-internetgateway.html
pub struct InternetGateway_ {
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_InternetGateway {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::InternetGateway"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_InternetGateway as InternetGateway;
impl crate::template::ToResource for InternetGateway_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InternetGateway"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-ippoolroutetableassociation.html
pub struct IpPoolRouteTableAssociation_ {
    pub public_ipv4_pool: crate::value::ExpString,
    pub route_table_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_IpPoolRouteTableAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::IpPoolRouteTableAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_IpPoolRouteTableAssociation as IpPoolRouteTableAssociation;
impl crate::template::ToResource for IpPoolRouteTableAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "IpPoolRouteTableAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PublicIpv4Pool".to_string(),
            crate::value::ToValue::to_value(&self.public_ipv4_pool),
        );
        properties.insert(
            "RouteTableId".to_string(),
            crate::value::ToValue::to_value(&self.route_table_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-keypair.html
pub struct KeyPair_ {
    pub key_format: Option<crate::value::ExpString>,
    pub key_name: crate::value::ExpString,
    pub key_type: Option<crate::value::ExpString>,
    pub public_key_material: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_KeyPair {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::KeyPair" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_KeyPair as KeyPair;
impl crate::template::ToResource for KeyPair_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("KeyPair"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.key_format {
            properties.insert(
                "KeyFormat".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "KeyName".to_string(),
            crate::value::ToValue::to_value(&self.key_name),
        );
        if let Some(ref value) = self.key_type {
            properties.insert(
                "KeyType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.public_key_material {
            properties.insert(
                "PublicKeyMaterial".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-launchtemplate.html
pub struct LaunchTemplate_ {
    pub launch_template_data: super::ec2::launchtemplate::LaunchTemplateData_,
    pub launch_template_name: Option<crate::value::ExpString>,
    pub tag_specifications:
        Option<Vec<super::ec2::launchtemplate::LaunchTemplateTagSpecification_>>,
    pub version_description: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_LaunchTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::LaunchTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_LaunchTemplate as LaunchTemplate;
impl crate::template::ToResource for LaunchTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LaunchTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "LaunchTemplateData".to_string(),
            crate::value::ToValue::to_value(&self.launch_template_data),
        );
        if let Some(ref value) = self.launch_template_name {
            properties.insert(
                "LaunchTemplateName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tag_specifications {
            properties.insert(
                "TagSpecifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.version_description {
            properties.insert(
                "VersionDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-localgatewayroute.html
pub struct LocalGatewayRoute_ {
    pub destination_cidr_block: crate::value::ExpString,
    pub local_gateway_route_table_id: crate::value::ExpString,
    pub local_gateway_virtual_interface_group_id: Option<crate::value::ExpString>,
    pub network_interface_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_LocalGatewayRoute {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::LocalGatewayRoute"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_LocalGatewayRoute as LocalGatewayRoute;
impl crate::template::ToResource for LocalGatewayRoute_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LocalGatewayRoute"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DestinationCidrBlock".to_string(),
            crate::value::ToValue::to_value(&self.destination_cidr_block),
        );
        properties.insert(
            "LocalGatewayRouteTableId".to_string(),
            crate::value::ToValue::to_value(&self.local_gateway_route_table_id),
        );
        if let Some(ref value) = self.local_gateway_virtual_interface_group_id {
            properties.insert(
                "LocalGatewayVirtualInterfaceGroupId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_interface_id {
            properties.insert(
                "NetworkInterfaceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-localgatewayroutetable.html
pub struct LocalGatewayRouteTable_ {
    pub local_gateway_id: crate::value::ExpString,
    pub mode: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_LocalGatewayRouteTable {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::LocalGatewayRouteTable"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_LocalGatewayRouteTable as LocalGatewayRouteTable;
impl crate::template::ToResource for LocalGatewayRouteTable_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LocalGatewayRouteTable"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "LocalGatewayId".to_string(),
            crate::value::ToValue::to_value(&self.local_gateway_id),
        );
        if let Some(ref value) = self.mode {
            properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-localgatewayroutetablevpcassociation.html
pub struct LocalGatewayRouteTableVPCAssociation_ {
    pub local_gateway_route_table_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_LocalGatewayRouteTableVPCAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::LocalGatewayRouteTableVPCAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_LocalGatewayRouteTableVPCAssociation as LocalGatewayRouteTableVPCAssociation;
impl crate::template::ToResource for LocalGatewayRouteTableVPCAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "LocalGatewayRouteTableVPCAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "LocalGatewayRouteTableId".to_string(),
            crate::value::ToValue::to_value(&self.local_gateway_route_table_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-localgatewayroutetablevirtualinterfacegroupassociation.html
pub struct LocalGatewayRouteTableVirtualInterfaceGroupAssociation_ {
    pub local_gateway_route_table_id: crate::value::ExpString,
    pub local_gateway_virtual_interface_group_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_LocalGatewayRouteTableVirtualInterfaceGroupAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::LocalGatewayRouteTableVirtualInterfaceGroupAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_LocalGatewayRouteTableVirtualInterfaceGroupAssociation as LocalGatewayRouteTableVirtualInterfaceGroupAssociation;
impl crate::template::ToResource for LocalGatewayRouteTableVirtualInterfaceGroupAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "LocalGatewayRouteTableVirtualInterfaceGroupAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "LocalGatewayRouteTableId".to_string(),
            crate::value::ToValue::to_value(&self.local_gateway_route_table_id),
        );
        properties.insert(
            "LocalGatewayVirtualInterfaceGroupId".to_string(),
            crate::value::ToValue::to_value(&self.local_gateway_virtual_interface_group_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-localgatewayvirtualinterface.html
pub struct LocalGatewayVirtualInterface_ {
    pub local_address: crate::value::ExpString,
    pub local_gateway_virtual_interface_group_id: crate::value::ExpString,
    pub outpost_lag_id: crate::value::ExpString,
    pub peer_address: crate::value::ExpString,
    pub peer_bgp_asn: Option<i32>,
    pub peer_bgp_asn_extended: Option<i64>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vlan: i32,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_LocalGatewayVirtualInterface {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::LocalGatewayVirtualInterface"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_LocalGatewayVirtualInterface as LocalGatewayVirtualInterface;
impl crate::template::ToResource for LocalGatewayVirtualInterface_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "LocalGatewayVirtualInterface",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "LocalAddress".to_string(),
            crate::value::ToValue::to_value(&self.local_address),
        );
        properties.insert(
            "LocalGatewayVirtualInterfaceGroupId".to_string(),
            crate::value::ToValue::to_value(&self.local_gateway_virtual_interface_group_id),
        );
        properties.insert(
            "OutpostLagId".to_string(),
            crate::value::ToValue::to_value(&self.outpost_lag_id),
        );
        properties.insert(
            "PeerAddress".to_string(),
            crate::value::ToValue::to_value(&self.peer_address),
        );
        if let Some(ref value) = self.peer_bgp_asn {
            properties.insert(
                "PeerBgpAsn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.peer_bgp_asn_extended {
            properties.insert(
                "PeerBgpAsnExtended".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Vlan".to_string(),
            crate::value::ToValue::to_value(&self.vlan),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-localgatewayvirtualinterfacegroup.html
pub struct LocalGatewayVirtualInterfaceGroup_ {
    pub local_bgp_asn: Option<i32>,
    pub local_bgp_asn_extended: Option<i64>,
    pub local_gateway_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_LocalGatewayVirtualInterfaceGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::LocalGatewayVirtualInterfaceGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_LocalGatewayVirtualInterfaceGroup as LocalGatewayVirtualInterfaceGroup;
impl crate::template::ToResource for LocalGatewayVirtualInterfaceGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "LocalGatewayVirtualInterfaceGroup",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.local_bgp_asn {
            properties.insert(
                "LocalBgpAsn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.local_bgp_asn_extended {
            properties.insert(
                "LocalBgpAsnExtended".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LocalGatewayId".to_string(),
            crate::value::ToValue::to_value(&self.local_gateway_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-natgateway.html
pub struct NatGateway_ {
    pub allocation_id: Option<crate::value::ExpString>,
    pub availability_mode: Option<crate::value::ExpString>,
    pub availability_zone_addresses: Option<Vec<super::ec2::natgateway::AvailabilityZoneAddress_>>,
    pub connectivity_type: Option<crate::value::ExpString>,
    pub max_drain_duration_seconds: Option<i32>,
    pub private_ip_address: Option<crate::value::ExpString>,
    pub secondary_allocation_ids: Option<Vec<crate::value::ExpString>>,
    pub secondary_private_ip_address_count: Option<i32>,
    pub secondary_private_ip_addresses: Option<Vec<crate::value::ExpString>>,
    pub subnet_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_NatGateway {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::NatGateway" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_NatGateway as NatGateway;
impl crate::template::ToResource for NatGateway_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("NatGateway"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allocation_id {
            properties.insert(
                "AllocationId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_mode {
            properties.insert(
                "AvailabilityMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zone_addresses {
            properties.insert(
                "AvailabilityZoneAddresses".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.connectivity_type {
            properties.insert(
                "ConnectivityType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_drain_duration_seconds {
            properties.insert(
                "MaxDrainDurationSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.private_ip_address {
            properties.insert(
                "PrivateIpAddress".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.secondary_allocation_ids {
            properties.insert(
                "SecondaryAllocationIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.secondary_private_ip_address_count {
            properties.insert(
                "SecondaryPrivateIpAddressCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.secondary_private_ip_addresses {
            properties.insert(
                "SecondaryPrivateIpAddresses".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_id {
            properties.insert(
                "SubnetId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_id {
            properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-networkacl.html
pub struct NetworkAcl_ {
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_NetworkAcl {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::NetworkAcl" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_NetworkAcl as NetworkAcl;
impl crate::template::ToResource for NetworkAcl_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("NetworkAcl"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-networkaclentry.html
pub struct NetworkAclEntry_ {
    pub cidr_block: Option<crate::value::ExpString>,
    pub egress: Option<crate::value::ExpBool>,
    pub icmp: Option<super::ec2::networkaclentry::Icmp_>,
    pub ipv6_cidr_block: Option<crate::value::ExpString>,
    pub network_acl_id: crate::value::ExpString,
    pub port_range: Option<super::ec2::networkaclentry::PortRange_>,
    pub protocol: i32,
    pub rule_action: crate::value::ExpString,
    pub rule_number: i32,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_NetworkAclEntry {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::NetworkAclEntry"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_NetworkAclEntry as NetworkAclEntry;
impl crate::template::ToResource for NetworkAclEntry_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("NetworkAclEntry"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cidr_block {
            properties.insert(
                "CidrBlock".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.egress {
            properties.insert("Egress".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.icmp {
            properties.insert("Icmp".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.ipv6_cidr_block {
            properties.insert(
                "Ipv6CidrBlock".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NetworkAclId".to_string(),
            crate::value::ToValue::to_value(&self.network_acl_id),
        );
        if let Some(ref value) = self.port_range {
            properties.insert(
                "PortRange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Protocol".to_string(),
            crate::value::ToValue::to_value(&self.protocol),
        );
        properties.insert(
            "RuleAction".to_string(),
            crate::value::ToValue::to_value(&self.rule_action),
        );
        properties.insert(
            "RuleNumber".to_string(),
            crate::value::ToValue::to_value(&self.rule_number),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-networkinsightsaccessscope.html
pub struct NetworkInsightsAccessScope_ {
    pub exclude_paths: Option<Vec<super::ec2::networkinsightsaccessscope::AccessScopePathRequest_>>,
    pub match_paths: Option<Vec<super::ec2::networkinsightsaccessscope::AccessScopePathRequest_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_NetworkInsightsAccessScope {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::NetworkInsightsAccessScope"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_NetworkInsightsAccessScope as NetworkInsightsAccessScope;
impl crate::template::ToResource for NetworkInsightsAccessScope_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "NetworkInsightsAccessScope",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.exclude_paths {
            properties.insert(
                "ExcludePaths".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.match_paths {
            properties.insert(
                "MatchPaths".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-networkinsightsaccessscopeanalysis.html
pub struct NetworkInsightsAccessScopeAnalysis_ {
    pub network_insights_access_scope_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_NetworkInsightsAccessScopeAnalysis {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::NetworkInsightsAccessScopeAnalysis"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_NetworkInsightsAccessScopeAnalysis as NetworkInsightsAccessScopeAnalysis;
impl crate::template::ToResource for NetworkInsightsAccessScopeAnalysis_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "NetworkInsightsAccessScopeAnalysis",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "NetworkInsightsAccessScopeId".to_string(),
            crate::value::ToValue::to_value(&self.network_insights_access_scope_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-networkinsightsanalysis.html
pub struct NetworkInsightsAnalysis_ {
    pub additional_accounts: Option<Vec<crate::value::ExpString>>,
    pub filter_in_arns: Option<Vec<crate::value::ExpString>>,
    pub filter_out_arns: Option<Vec<crate::value::ExpString>>,
    pub network_insights_path_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_NetworkInsightsAnalysis {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::NetworkInsightsAnalysis"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_NetworkInsightsAnalysis as NetworkInsightsAnalysis;
impl crate::template::ToResource for NetworkInsightsAnalysis_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("NetworkInsightsAnalysis"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_accounts {
            properties.insert(
                "AdditionalAccounts".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.filter_in_arns {
            properties.insert(
                "FilterInArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.filter_out_arns {
            properties.insert(
                "FilterOutArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NetworkInsightsPathId".to_string(),
            crate::value::ToValue::to_value(&self.network_insights_path_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-networkinsightspath.html
pub struct NetworkInsightsPath_ {
    pub destination: Option<crate::value::ExpString>,
    pub destination_ip: Option<crate::value::ExpString>,
    pub destination_port: Option<i32>,
    pub filter_at_destination: Option<super::ec2::networkinsightspath::PathFilter_>,
    pub filter_at_source: Option<super::ec2::networkinsightspath::PathFilter_>,
    pub protocol: crate::value::ExpString,
    pub source: crate::value::ExpString,
    pub source_ip: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_NetworkInsightsPath {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::NetworkInsightsPath"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_NetworkInsightsPath as NetworkInsightsPath;
impl crate::template::ToResource for NetworkInsightsPath_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("NetworkInsightsPath"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.destination {
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destination_ip {
            properties.insert(
                "DestinationIp".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destination_port {
            properties.insert(
                "DestinationPort".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.filter_at_destination {
            properties.insert(
                "FilterAtDestination".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.filter_at_source {
            properties.insert(
                "FilterAtSource".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Protocol".to_string(),
            crate::value::ToValue::to_value(&self.protocol),
        );
        properties.insert(
            "Source".to_string(),
            crate::value::ToValue::to_value(&self.source),
        );
        if let Some(ref value) = self.source_ip {
            properties.insert(
                "SourceIp".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-networkinterface.html
pub struct NetworkInterface_ {
    pub connection_tracking_specification:
        Option<super::ec2::networkinterface::ConnectionTrackingSpecification_>,
    pub description: Option<crate::value::ExpString>,
    pub group_set: Option<Vec<crate::value::ExpString>>,
    pub interface_type: Option<crate::value::ExpString>,
    pub ipv4_prefix_count: Option<i32>,
    pub ipv4_prefixes: Option<Vec<super::ec2::networkinterface::Ipv4PrefixSpecification_>>,
    pub ipv6_address_count: Option<i32>,
    pub ipv6_addresses: Option<Vec<super::ec2::networkinterface::InstanceIpv6Address_>>,
    pub ipv6_prefix_count: Option<i32>,
    pub ipv6_prefixes: Option<Vec<super::ec2::networkinterface::Ipv6PrefixSpecification_>>,
    pub private_ip_address: Option<crate::value::ExpString>,
    pub private_ip_addresses:
        Option<Vec<super::ec2::networkinterface::PrivateIpAddressSpecification_>>,
    pub public_ip_dns_hostname_type_specification: Option<crate::value::ExpString>,
    pub secondary_private_ip_address_count: Option<i32>,
    pub source_dest_check: Option<crate::value::ExpBool>,
    pub subnet_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_NetworkInterface {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::NetworkInterface"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_NetworkInterface as NetworkInterface;
impl crate::template::ToResource for NetworkInterface_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("NetworkInterface"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.connection_tracking_specification {
            properties.insert(
                "ConnectionTrackingSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.group_set {
            properties.insert(
                "GroupSet".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.interface_type {
            properties.insert(
                "InterfaceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv4_prefix_count {
            properties.insert(
                "Ipv4PrefixCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv4_prefixes {
            properties.insert(
                "Ipv4Prefixes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_address_count {
            properties.insert(
                "Ipv6AddressCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_addresses {
            properties.insert(
                "Ipv6Addresses".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_prefix_count {
            properties.insert(
                "Ipv6PrefixCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_prefixes {
            properties.insert(
                "Ipv6Prefixes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.private_ip_address {
            properties.insert(
                "PrivateIpAddress".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.private_ip_addresses {
            properties.insert(
                "PrivateIpAddresses".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.public_ip_dns_hostname_type_specification {
            properties.insert(
                "PublicIpDnsHostnameTypeSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.secondary_private_ip_address_count {
            properties.insert(
                "SecondaryPrivateIpAddressCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_dest_check {
            properties.insert(
                "SourceDestCheck".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SubnetId".to_string(),
            crate::value::ToValue::to_value(&self.subnet_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-networkinterfaceattachment.html
pub struct NetworkInterfaceAttachment_ {
    pub delete_on_termination: Option<crate::value::ExpBool>,
    pub device_index: crate::value::ExpString,
    pub ena_queue_count: Option<i32>,
    pub ena_srd_specification: Option<super::ec2::networkinterfaceattachment::EnaSrdSpecification_>,
    pub instance_id: crate::value::ExpString,
    pub network_interface_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_NetworkInterfaceAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::NetworkInterfaceAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_NetworkInterfaceAttachment as NetworkInterfaceAttachment;
impl crate::template::ToResource for NetworkInterfaceAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "NetworkInterfaceAttachment",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.delete_on_termination {
            properties.insert(
                "DeleteOnTermination".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DeviceIndex".to_string(),
            crate::value::ToValue::to_value(&self.device_index),
        );
        if let Some(ref value) = self.ena_queue_count {
            properties.insert(
                "EnaQueueCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ena_srd_specification {
            properties.insert(
                "EnaSrdSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceId".to_string(),
            crate::value::ToValue::to_value(&self.instance_id),
        );
        properties.insert(
            "NetworkInterfaceId".to_string(),
            crate::value::ToValue::to_value(&self.network_interface_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-networkinterfacepermission.html
pub struct NetworkInterfacePermission_ {
    pub aws_account_id: crate::value::ExpString,
    pub network_interface_id: crate::value::ExpString,
    pub permission: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_NetworkInterfacePermission {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::NetworkInterfacePermission"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_NetworkInterfacePermission as NetworkInterfacePermission;
impl crate::template::ToResource for NetworkInterfacePermission_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "NetworkInterfacePermission",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AwsAccountId".to_string(),
            crate::value::ToValue::to_value(&self.aws_account_id),
        );
        properties.insert(
            "NetworkInterfaceId".to_string(),
            crate::value::ToValue::to_value(&self.network_interface_id),
        );
        properties.insert(
            "Permission".to_string(),
            crate::value::ToValue::to_value(&self.permission),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-networkperformancemetricsubscription.html
pub struct NetworkPerformanceMetricSubscription_ {
    pub destination: crate::value::ExpString,
    pub metric: crate::value::ExpString,
    pub source: crate::value::ExpString,
    pub statistic: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_NetworkPerformanceMetricSubscription {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::NetworkPerformanceMetricSubscription"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_NetworkPerformanceMetricSubscription as NetworkPerformanceMetricSubscription;
impl crate::template::ToResource for NetworkPerformanceMetricSubscription_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "NetworkPerformanceMetricSubscription",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Destination".to_string(),
            crate::value::ToValue::to_value(&self.destination),
        );
        properties.insert(
            "Metric".to_string(),
            crate::value::ToValue::to_value(&self.metric),
        );
        properties.insert(
            "Source".to_string(),
            crate::value::ToValue::to_value(&self.source),
        );
        properties.insert(
            "Statistic".to_string(),
            crate::value::ToValue::to_value(&self.statistic),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-placementgroup.html
pub struct PlacementGroup_ {
    pub partition_count: Option<i32>,
    pub spread_level: Option<crate::value::ExpString>,
    pub strategy: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_PlacementGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::PlacementGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_PlacementGroup as PlacementGroup;
impl crate::template::ToResource for PlacementGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PlacementGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.partition_count {
            properties.insert(
                "PartitionCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.spread_level {
            properties.insert(
                "SpreadLevel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.strategy {
            properties.insert(
                "Strategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-prefixlist.html
pub struct PrefixList_ {
    pub address_family: crate::value::ExpString,
    pub entries: Option<Vec<super::ec2::prefixlist::Entry_>>,
    pub max_entries: Option<i32>,
    pub prefix_list_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_PrefixList {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::PrefixList" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_PrefixList as PrefixList;
impl crate::template::ToResource for PrefixList_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PrefixList"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AddressFamily".to_string(),
            crate::value::ToValue::to_value(&self.address_family),
        );
        if let Some(ref value) = self.entries {
            properties.insert(
                "Entries".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_entries {
            properties.insert(
                "MaxEntries".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PrefixListName".to_string(),
            crate::value::ToValue::to_value(&self.prefix_list_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-route.html
pub struct Route_ {
    pub carrier_gateway_id: Option<crate::value::ExpString>,
    pub core_network_arn: Option<crate::value::ExpString>,
    pub destination_cidr_block: Option<crate::value::ExpString>,
    pub destination_ipv6_cidr_block: Option<crate::value::ExpString>,
    pub destination_prefix_list_id: Option<crate::value::ExpString>,
    pub egress_only_internet_gateway_id: Option<crate::value::ExpString>,
    pub gateway_id: Option<crate::value::ExpString>,
    pub instance_id: Option<crate::value::ExpString>,
    pub local_gateway_id: Option<crate::value::ExpString>,
    pub nat_gateway_id: Option<crate::value::ExpString>,
    pub network_interface_id: Option<crate::value::ExpString>,
    pub route_table_id: crate::value::ExpString,
    pub transit_gateway_id: Option<crate::value::ExpString>,
    pub vpc_endpoint_id: Option<crate::value::ExpString>,
    pub vpc_peering_connection_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_Route {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::Route" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_Route as Route;
impl crate::template::ToResource for Route_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Route"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.carrier_gateway_id {
            properties.insert(
                "CarrierGatewayId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.core_network_arn {
            properties.insert(
                "CoreNetworkArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destination_cidr_block {
            properties.insert(
                "DestinationCidrBlock".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destination_ipv6_cidr_block {
            properties.insert(
                "DestinationIpv6CidrBlock".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destination_prefix_list_id {
            properties.insert(
                "DestinationPrefixListId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.egress_only_internet_gateway_id {
            properties.insert(
                "EgressOnlyInternetGatewayId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.gateway_id {
            properties.insert(
                "GatewayId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_id {
            properties.insert(
                "InstanceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.local_gateway_id {
            properties.insert(
                "LocalGatewayId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.nat_gateway_id {
            properties.insert(
                "NatGatewayId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_interface_id {
            properties.insert(
                "NetworkInterfaceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RouteTableId".to_string(),
            crate::value::ToValue::to_value(&self.route_table_id),
        );
        if let Some(ref value) = self.transit_gateway_id {
            properties.insert(
                "TransitGatewayId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_endpoint_id {
            properties.insert(
                "VpcEndpointId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_peering_connection_id {
            properties.insert(
                "VpcPeeringConnectionId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-routeserver.html
pub struct RouteServer_ {
    pub amazon_side_asn: i64,
    pub persist_routes: Option<crate::value::ExpString>,
    pub persist_routes_duration: Option<i64>,
    pub sns_notifications_enabled: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_RouteServer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::RouteServer"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_RouteServer as RouteServer;
impl crate::template::ToResource for RouteServer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RouteServer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AmazonSideAsn".to_string(),
            crate::value::ToValue::to_value(&self.amazon_side_asn),
        );
        if let Some(ref value) = self.persist_routes {
            properties.insert(
                "PersistRoutes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.persist_routes_duration {
            properties.insert(
                "PersistRoutesDuration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sns_notifications_enabled {
            properties.insert(
                "SnsNotificationsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-routeserverassociation.html
pub struct RouteServerAssociation_ {
    pub route_server_id: crate::value::ExpString,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_RouteServerAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::RouteServerAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_RouteServerAssociation as RouteServerAssociation;
impl crate::template::ToResource for RouteServerAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RouteServerAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "RouteServerId".to_string(),
            crate::value::ToValue::to_value(&self.route_server_id),
        );
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-routeserverendpoint.html
pub struct RouteServerEndpoint_ {
    pub route_server_id: crate::value::ExpString,
    pub subnet_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_RouteServerEndpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::RouteServerEndpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_RouteServerEndpoint as RouteServerEndpoint;
impl crate::template::ToResource for RouteServerEndpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RouteServerEndpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "RouteServerId".to_string(),
            crate::value::ToValue::to_value(&self.route_server_id),
        );
        properties.insert(
            "SubnetId".to_string(),
            crate::value::ToValue::to_value(&self.subnet_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-routeserverpeer.html
pub struct RouteServerPeer_ {
    pub bgp_options: super::ec2::routeserverpeer::BgpOptions_,
    pub peer_address: crate::value::ExpString,
    pub route_server_endpoint_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_RouteServerPeer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::RouteServerPeer"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_RouteServerPeer as RouteServerPeer;
impl crate::template::ToResource for RouteServerPeer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RouteServerPeer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "BgpOptions".to_string(),
            crate::value::ToValue::to_value(&self.bgp_options),
        );
        properties.insert(
            "PeerAddress".to_string(),
            crate::value::ToValue::to_value(&self.peer_address),
        );
        properties.insert(
            "RouteServerEndpointId".to_string(),
            crate::value::ToValue::to_value(&self.route_server_endpoint_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-routeserverpropagation.html
pub struct RouteServerPropagation_ {
    pub route_server_id: crate::value::ExpString,
    pub route_table_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_RouteServerPropagation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::RouteServerPropagation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_RouteServerPropagation as RouteServerPropagation;
impl crate::template::ToResource for RouteServerPropagation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RouteServerPropagation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "RouteServerId".to_string(),
            crate::value::ToValue::to_value(&self.route_server_id),
        );
        properties.insert(
            "RouteTableId".to_string(),
            crate::value::ToValue::to_value(&self.route_table_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-routetable.html
pub struct RouteTable_ {
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_RouteTable {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::RouteTable" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_RouteTable as RouteTable;
impl crate::template::ToResource for RouteTable_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RouteTable"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-securitygroup.html
pub struct SecurityGroup_ {
    pub group_description: crate::value::ExpString,
    pub group_name: Option<crate::value::ExpString>,
    pub security_group_egress: Option<Vec<super::ec2::securitygroup::Egress_>>,
    pub security_group_ingress: Option<Vec<super::ec2::securitygroup::Ingress_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_SecurityGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::SecurityGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_SecurityGroup as SecurityGroup;
impl crate::template::ToResource for SecurityGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SecurityGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "GroupDescription".to_string(),
            crate::value::ToValue::to_value(&self.group_description),
        );
        if let Some(ref value) = self.group_name {
            properties.insert(
                "GroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_egress {
            properties.insert(
                "SecurityGroupEgress".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ingress {
            properties.insert(
                "SecurityGroupIngress".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_id {
            properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-securitygroupegress.html
pub struct SecurityGroupEgress_ {
    pub cidr_ip: Option<crate::value::ExpString>,
    pub cidr_ipv6: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub destination_prefix_list_id: Option<crate::value::ExpString>,
    pub destination_security_group_id: Option<crate::value::ExpString>,
    pub from_port: Option<i32>,
    pub group_id: crate::value::ExpString,
    pub ip_protocol: crate::value::ExpString,
    pub to_port: Option<i32>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_SecurityGroupEgress {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::SecurityGroupEgress"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_SecurityGroupEgress as SecurityGroupEgress;
impl crate::template::ToResource for SecurityGroupEgress_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SecurityGroupEgress"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cidr_ip {
            properties.insert("CidrIp".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.cidr_ipv6 {
            properties.insert(
                "CidrIpv6".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destination_prefix_list_id {
            properties.insert(
                "DestinationPrefixListId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destination_security_group_id {
            properties.insert(
                "DestinationSecurityGroupId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.from_port {
            properties.insert(
                "FromPort".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "GroupId".to_string(),
            crate::value::ToValue::to_value(&self.group_id),
        );
        properties.insert(
            "IpProtocol".to_string(),
            crate::value::ToValue::to_value(&self.ip_protocol),
        );
        if let Some(ref value) = self.to_port {
            properties.insert("ToPort".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-securitygroupingress.html
pub struct SecurityGroupIngress_ {
    pub cidr_ip: Option<crate::value::ExpString>,
    pub cidr_ipv6: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub from_port: Option<i32>,
    pub group_id: Option<crate::value::ExpString>,
    pub group_name: Option<crate::value::ExpString>,
    pub ip_protocol: crate::value::ExpString,
    pub source_prefix_list_id: Option<crate::value::ExpString>,
    pub source_security_group_id: Option<crate::value::ExpString>,
    pub source_security_group_name: Option<crate::value::ExpString>,
    pub source_security_group_owner_id: Option<crate::value::ExpString>,
    pub to_port: Option<i32>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_SecurityGroupIngress {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::SecurityGroupIngress"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_SecurityGroupIngress as SecurityGroupIngress;
impl crate::template::ToResource for SecurityGroupIngress_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SecurityGroupIngress"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cidr_ip {
            properties.insert("CidrIp".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.cidr_ipv6 {
            properties.insert(
                "CidrIpv6".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.from_port {
            properties.insert(
                "FromPort".to_string(),
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
        properties.insert(
            "IpProtocol".to_string(),
            crate::value::ToValue::to_value(&self.ip_protocol),
        );
        if let Some(ref value) = self.source_prefix_list_id {
            properties.insert(
                "SourcePrefixListId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_security_group_id {
            properties.insert(
                "SourceSecurityGroupId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_security_group_name {
            properties.insert(
                "SourceSecurityGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_security_group_owner_id {
            properties.insert(
                "SourceSecurityGroupOwnerId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.to_port {
            properties.insert("ToPort".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-securitygroupvpcassociation.html
pub struct SecurityGroupVpcAssociation_ {
    pub group_id: crate::value::ExpString,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_SecurityGroupVpcAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::SecurityGroupVpcAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_SecurityGroupVpcAssociation as SecurityGroupVpcAssociation;
impl crate::template::ToResource for SecurityGroupVpcAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "SecurityGroupVpcAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "GroupId".to_string(),
            crate::value::ToValue::to_value(&self.group_id),
        );
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-snapshotblockpublicaccess.html
pub struct SnapshotBlockPublicAccess_ {
    pub state: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_SnapshotBlockPublicAccess {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::SnapshotBlockPublicAccess"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_SnapshotBlockPublicAccess as SnapshotBlockPublicAccess;
impl crate::template::ToResource for SnapshotBlockPublicAccess_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SnapshotBlockPublicAccess"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "State".to_string(),
            crate::value::ToValue::to_value(&self.state),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-spotfleet.html
pub struct SpotFleet_ {
    pub spot_fleet_request_config_data: super::ec2::spotfleet::SpotFleetRequestConfigData_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_SpotFleet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::SpotFleet" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_SpotFleet as SpotFleet;
impl crate::template::ToResource for SpotFleet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SpotFleet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "SpotFleetRequestConfigData".to_string(),
            crate::value::ToValue::to_value(&self.spot_fleet_request_config_data),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-subnet.html
pub struct Subnet_ {
    pub assign_ipv6_address_on_creation: Option<crate::value::ExpBool>,
    pub availability_zone: Option<crate::value::ExpString>,
    pub availability_zone_id: Option<crate::value::ExpString>,
    pub cidr_block: Option<crate::value::ExpString>,
    pub enable_dns64: Option<crate::value::ExpBool>,
    pub enable_lni_at_device_index: Option<i32>,
    pub ipv4_ipam_pool_id: Option<crate::value::ExpString>,
    pub ipv4_netmask_length: Option<i32>,
    pub ipv6_cidr_block: Option<crate::value::ExpString>,
    pub ipv6_ipam_pool_id: Option<crate::value::ExpString>,
    pub ipv6_native: Option<crate::value::ExpBool>,
    pub ipv6_netmask_length: Option<i32>,
    pub map_public_ip_on_launch: Option<crate::value::ExpBool>,
    pub outpost_arn: Option<crate::value::ExpString>,
    pub private_dns_name_options_on_launch:
        Option<super::ec2::subnet::PrivateDnsNameOptionsOnLaunch_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_Subnet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::Subnet" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_Subnet as Subnet;
impl crate::template::ToResource for Subnet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Subnet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.assign_ipv6_address_on_creation {
            properties.insert(
                "AssignIpv6AddressOnCreation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
        if let Some(ref value) = self.cidr_block {
            properties.insert(
                "CidrBlock".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_dns64 {
            properties.insert(
                "EnableDns64".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_lni_at_device_index {
            properties.insert(
                "EnableLniAtDeviceIndex".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv4_ipam_pool_id {
            properties.insert(
                "Ipv4IpamPoolId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv4_netmask_length {
            properties.insert(
                "Ipv4NetmaskLength".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_cidr_block {
            properties.insert(
                "Ipv6CidrBlock".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_ipam_pool_id {
            properties.insert(
                "Ipv6IpamPoolId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_native {
            properties.insert(
                "Ipv6Native".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_netmask_length {
            properties.insert(
                "Ipv6NetmaskLength".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.map_public_ip_on_launch {
            properties.insert(
                "MapPublicIpOnLaunch".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.outpost_arn {
            properties.insert(
                "OutpostArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.private_dns_name_options_on_launch {
            properties.insert(
                "PrivateDnsNameOptionsOnLaunch".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-subnetcidrblock.html
pub struct SubnetCidrBlock_ {
    pub ipv6_cidr_block: Option<crate::value::ExpString>,
    pub ipv6_ipam_pool_id: Option<crate::value::ExpString>,
    pub ipv6_netmask_length: Option<i32>,
    pub subnet_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_SubnetCidrBlock {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::SubnetCidrBlock"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_SubnetCidrBlock as SubnetCidrBlock;
impl crate::template::ToResource for SubnetCidrBlock_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SubnetCidrBlock"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.ipv6_cidr_block {
            properties.insert(
                "Ipv6CidrBlock".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_ipam_pool_id {
            properties.insert(
                "Ipv6IpamPoolId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_netmask_length {
            properties.insert(
                "Ipv6NetmaskLength".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SubnetId".to_string(),
            crate::value::ToValue::to_value(&self.subnet_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-subnetnetworkaclassociation.html
pub struct SubnetNetworkAclAssociation_ {
    pub network_acl_id: crate::value::ExpString,
    pub subnet_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_SubnetNetworkAclAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::SubnetNetworkAclAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_SubnetNetworkAclAssociation as SubnetNetworkAclAssociation;
impl crate::template::ToResource for SubnetNetworkAclAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "SubnetNetworkAclAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "NetworkAclId".to_string(),
            crate::value::ToValue::to_value(&self.network_acl_id),
        );
        properties.insert(
            "SubnetId".to_string(),
            crate::value::ToValue::to_value(&self.subnet_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-subnetroutetableassociation.html
pub struct SubnetRouteTableAssociation_ {
    pub route_table_id: crate::value::ExpString,
    pub subnet_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_SubnetRouteTableAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::SubnetRouteTableAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_SubnetRouteTableAssociation as SubnetRouteTableAssociation;
impl crate::template::ToResource for SubnetRouteTableAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "SubnetRouteTableAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "RouteTableId".to_string(),
            crate::value::ToValue::to_value(&self.route_table_id),
        );
        properties.insert(
            "SubnetId".to_string(),
            crate::value::ToValue::to_value(&self.subnet_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-trafficmirrorfilter.html
pub struct TrafficMirrorFilter_ {
    pub description: Option<crate::value::ExpString>,
    pub network_services: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TrafficMirrorFilter {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TrafficMirrorFilter"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TrafficMirrorFilter as TrafficMirrorFilter;
impl crate::template::ToResource for TrafficMirrorFilter_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TrafficMirrorFilter"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_services {
            properties.insert(
                "NetworkServices".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-trafficmirrorfilterrule.html
pub struct TrafficMirrorFilterRule_ {
    pub description: Option<crate::value::ExpString>,
    pub destination_cidr_block: crate::value::ExpString,
    pub destination_port_range:
        Option<super::ec2::trafficmirrorfilterrule::TrafficMirrorPortRange_>,
    pub protocol: Option<i32>,
    pub rule_action: crate::value::ExpString,
    pub rule_number: i32,
    pub source_cidr_block: crate::value::ExpString,
    pub source_port_range: Option<super::ec2::trafficmirrorfilterrule::TrafficMirrorPortRange_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub traffic_direction: crate::value::ExpString,
    pub traffic_mirror_filter_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TrafficMirrorFilterRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TrafficMirrorFilterRule"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TrafficMirrorFilterRule as TrafficMirrorFilterRule;
impl crate::template::ToResource for TrafficMirrorFilterRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TrafficMirrorFilterRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DestinationCidrBlock".to_string(),
            crate::value::ToValue::to_value(&self.destination_cidr_block),
        );
        if let Some(ref value) = self.destination_port_range {
            properties.insert(
                "DestinationPortRange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.protocol {
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RuleAction".to_string(),
            crate::value::ToValue::to_value(&self.rule_action),
        );
        properties.insert(
            "RuleNumber".to_string(),
            crate::value::ToValue::to_value(&self.rule_number),
        );
        properties.insert(
            "SourceCidrBlock".to_string(),
            crate::value::ToValue::to_value(&self.source_cidr_block),
        );
        if let Some(ref value) = self.source_port_range {
            properties.insert(
                "SourcePortRange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TrafficDirection".to_string(),
            crate::value::ToValue::to_value(&self.traffic_direction),
        );
        properties.insert(
            "TrafficMirrorFilterId".to_string(),
            crate::value::ToValue::to_value(&self.traffic_mirror_filter_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-trafficmirrorsession.html
pub struct TrafficMirrorSession_ {
    pub description: Option<crate::value::ExpString>,
    pub network_interface_id: crate::value::ExpString,
    pub owner_id: Option<crate::value::ExpString>,
    pub packet_length: Option<i32>,
    pub session_number: i32,
    pub tags: Option<Vec<crate::Tag_>>,
    pub traffic_mirror_filter_id: crate::value::ExpString,
    pub traffic_mirror_target_id: crate::value::ExpString,
    pub virtual_network_id: Option<i32>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TrafficMirrorSession {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TrafficMirrorSession"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TrafficMirrorSession as TrafficMirrorSession;
impl crate::template::ToResource for TrafficMirrorSession_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TrafficMirrorSession"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NetworkInterfaceId".to_string(),
            crate::value::ToValue::to_value(&self.network_interface_id),
        );
        if let Some(ref value) = self.owner_id {
            properties.insert(
                "OwnerId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.packet_length {
            properties.insert(
                "PacketLength".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SessionNumber".to_string(),
            crate::value::ToValue::to_value(&self.session_number),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TrafficMirrorFilterId".to_string(),
            crate::value::ToValue::to_value(&self.traffic_mirror_filter_id),
        );
        properties.insert(
            "TrafficMirrorTargetId".to_string(),
            crate::value::ToValue::to_value(&self.traffic_mirror_target_id),
        );
        if let Some(ref value) = self.virtual_network_id {
            properties.insert(
                "VirtualNetworkId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-trafficmirrortarget.html
pub struct TrafficMirrorTarget_ {
    pub description: Option<crate::value::ExpString>,
    pub gateway_load_balancer_endpoint_id: Option<crate::value::ExpString>,
    pub network_interface_id: Option<crate::value::ExpString>,
    pub network_load_balancer_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TrafficMirrorTarget {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TrafficMirrorTarget"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TrafficMirrorTarget as TrafficMirrorTarget;
impl crate::template::ToResource for TrafficMirrorTarget_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TrafficMirrorTarget"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.gateway_load_balancer_endpoint_id {
            properties.insert(
                "GatewayLoadBalancerEndpointId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_interface_id {
            properties.insert(
                "NetworkInterfaceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_load_balancer_arn {
            properties.insert(
                "NetworkLoadBalancerArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-transitgateway.html
pub struct TransitGateway_ {
    pub amazon_side_asn: Option<i64>,
    pub association_default_route_table_id: Option<crate::value::ExpString>,
    pub auto_accept_shared_attachments: Option<crate::value::ExpString>,
    pub default_route_table_association: Option<crate::value::ExpString>,
    pub default_route_table_propagation: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub dns_support: Option<crate::value::ExpString>,
    pub encryption_support: Option<crate::value::ExpString>,
    pub multicast_support: Option<crate::value::ExpString>,
    pub propagation_default_route_table_id: Option<crate::value::ExpString>,
    pub security_group_referencing_support: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transit_gateway_cidr_blocks: Option<Vec<crate::value::ExpString>>,
    pub vpn_ecmp_support: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TransitGateway {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TransitGateway"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TransitGateway as TransitGateway;
impl crate::template::ToResource for TransitGateway_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TransitGateway"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.amazon_side_asn {
            properties.insert(
                "AmazonSideAsn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.association_default_route_table_id {
            properties.insert(
                "AssociationDefaultRouteTableId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_accept_shared_attachments {
            properties.insert(
                "AutoAcceptSharedAttachments".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_route_table_association {
            properties.insert(
                "DefaultRouteTableAssociation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_route_table_propagation {
            properties.insert(
                "DefaultRouteTablePropagation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dns_support {
            properties.insert(
                "DnsSupport".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_support {
            properties.insert(
                "EncryptionSupport".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.multicast_support {
            properties.insert(
                "MulticastSupport".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.propagation_default_route_table_id {
            properties.insert(
                "PropagationDefaultRouteTableId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_referencing_support {
            properties.insert(
                "SecurityGroupReferencingSupport".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.transit_gateway_cidr_blocks {
            properties.insert(
                "TransitGatewayCidrBlocks".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpn_ecmp_support {
            properties.insert(
                "VpnEcmpSupport".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-transitgatewayattachment.html
pub struct TransitGatewayAttachment_ {
    pub options: Option<super::ec2::transitgatewayattachment::Options_>,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transit_gateway_id: crate::value::ExpString,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TransitGatewayAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TransitGatewayAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TransitGatewayAttachment as TransitGatewayAttachment;
impl crate::template::ToResource for TransitGatewayAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TransitGatewayAttachment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.options {
            properties.insert(
                "Options".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SubnetIds".to_string(),
            crate::value::ToValue::to_value(&self.subnet_ids),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TransitGatewayId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_id),
        );
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-transitgatewayconnect.html
pub struct TransitGatewayConnect_ {
    pub options: super::ec2::transitgatewayconnect::TransitGatewayConnectOptions_,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transport_transit_gateway_attachment_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TransitGatewayConnect {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TransitGatewayConnect"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TransitGatewayConnect as TransitGatewayConnect;
impl crate::template::ToResource for TransitGatewayConnect_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TransitGatewayConnect"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Options".to_string(),
            crate::value::ToValue::to_value(&self.options),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TransportTransitGatewayAttachmentId".to_string(),
            crate::value::ToValue::to_value(&self.transport_transit_gateway_attachment_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-transitgatewayconnectpeer.html
pub struct TransitGatewayConnectPeer_ {
    pub connect_peer_configuration:
        super::ec2::transitgatewayconnectpeer::TransitGatewayConnectPeerConfiguration_,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transit_gateway_attachment_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TransitGatewayConnectPeer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TransitGatewayConnectPeer"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TransitGatewayConnectPeer as TransitGatewayConnectPeer;
impl crate::template::ToResource for TransitGatewayConnectPeer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TransitGatewayConnectPeer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConnectPeerConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.connect_peer_configuration),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TransitGatewayAttachmentId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_attachment_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-transitgatewaymeteringpolicy.html
pub struct TransitGatewayMeteringPolicy_ {
    pub middlebox_attachment_ids: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transit_gateway_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TransitGatewayMeteringPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TransitGatewayMeteringPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TransitGatewayMeteringPolicy as TransitGatewayMeteringPolicy;
impl crate::template::ToResource for TransitGatewayMeteringPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "TransitGatewayMeteringPolicy",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.middlebox_attachment_ids {
            properties.insert(
                "MiddleboxAttachmentIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TransitGatewayId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-transitgatewaymeteringpolicyentry.html
pub struct TransitGatewayMeteringPolicyEntry_ {
    pub destination_cidr_block: Option<crate::value::ExpString>,
    pub destination_port_range: Option<crate::value::ExpString>,
    pub destination_transit_gateway_attachment_id: Option<crate::value::ExpString>,
    pub destination_transit_gateway_attachment_type: Option<crate::value::ExpString>,
    pub metered_account: crate::value::ExpString,
    pub policy_rule_number: i32,
    pub protocol: Option<crate::value::ExpString>,
    pub source_cidr_block: Option<crate::value::ExpString>,
    pub source_port_range: Option<crate::value::ExpString>,
    pub source_transit_gateway_attachment_id: Option<crate::value::ExpString>,
    pub source_transit_gateway_attachment_type: Option<crate::value::ExpString>,
    pub transit_gateway_metering_policy_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TransitGatewayMeteringPolicyEntry {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TransitGatewayMeteringPolicyEntry"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TransitGatewayMeteringPolicyEntry as TransitGatewayMeteringPolicyEntry;
impl crate::template::ToResource for TransitGatewayMeteringPolicyEntry_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "TransitGatewayMeteringPolicyEntry",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.destination_cidr_block {
            properties.insert(
                "DestinationCidrBlock".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destination_port_range {
            properties.insert(
                "DestinationPortRange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destination_transit_gateway_attachment_id {
            properties.insert(
                "DestinationTransitGatewayAttachmentId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destination_transit_gateway_attachment_type {
            properties.insert(
                "DestinationTransitGatewayAttachmentType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MeteredAccount".to_string(),
            crate::value::ToValue::to_value(&self.metered_account),
        );
        properties.insert(
            "PolicyRuleNumber".to_string(),
            crate::value::ToValue::to_value(&self.policy_rule_number),
        );
        if let Some(ref value) = self.protocol {
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_cidr_block {
            properties.insert(
                "SourceCidrBlock".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_port_range {
            properties.insert(
                "SourcePortRange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_transit_gateway_attachment_id {
            properties.insert(
                "SourceTransitGatewayAttachmentId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_transit_gateway_attachment_type {
            properties.insert(
                "SourceTransitGatewayAttachmentType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TransitGatewayMeteringPolicyId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_metering_policy_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-transitgatewaymulticastdomain.html
pub struct TransitGatewayMulticastDomain_ {
    pub options: Option<super::ec2::transitgatewaymulticastdomain::Options_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transit_gateway_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TransitGatewayMulticastDomain {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TransitGatewayMulticastDomain"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TransitGatewayMulticastDomain as TransitGatewayMulticastDomain;
impl crate::template::ToResource for TransitGatewayMulticastDomain_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "TransitGatewayMulticastDomain",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.options {
            properties.insert(
                "Options".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TransitGatewayId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-transitgatewaymulticastdomainassociation.html
pub struct TransitGatewayMulticastDomainAssociation_ {
    pub subnet_id: crate::value::ExpString,
    pub transit_gateway_attachment_id: crate::value::ExpString,
    pub transit_gateway_multicast_domain_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TransitGatewayMulticastDomainAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TransitGatewayMulticastDomainAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TransitGatewayMulticastDomainAssociation as TransitGatewayMulticastDomainAssociation;
impl crate::template::ToResource for TransitGatewayMulticastDomainAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "TransitGatewayMulticastDomainAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "SubnetId".to_string(),
            crate::value::ToValue::to_value(&self.subnet_id),
        );
        properties.insert(
            "TransitGatewayAttachmentId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_attachment_id),
        );
        properties.insert(
            "TransitGatewayMulticastDomainId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_multicast_domain_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-transitgatewaymulticastgroupmember.html
pub struct TransitGatewayMulticastGroupMember_ {
    pub group_ip_address: crate::value::ExpString,
    pub network_interface_id: crate::value::ExpString,
    pub transit_gateway_multicast_domain_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TransitGatewayMulticastGroupMember {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TransitGatewayMulticastGroupMember"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TransitGatewayMulticastGroupMember as TransitGatewayMulticastGroupMember;
impl crate::template::ToResource for TransitGatewayMulticastGroupMember_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "TransitGatewayMulticastGroupMember",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "GroupIpAddress".to_string(),
            crate::value::ToValue::to_value(&self.group_ip_address),
        );
        properties.insert(
            "NetworkInterfaceId".to_string(),
            crate::value::ToValue::to_value(&self.network_interface_id),
        );
        properties.insert(
            "TransitGatewayMulticastDomainId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_multicast_domain_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-transitgatewaymulticastgroupsource.html
pub struct TransitGatewayMulticastGroupSource_ {
    pub group_ip_address: crate::value::ExpString,
    pub network_interface_id: crate::value::ExpString,
    pub transit_gateway_multicast_domain_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TransitGatewayMulticastGroupSource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TransitGatewayMulticastGroupSource"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TransitGatewayMulticastGroupSource as TransitGatewayMulticastGroupSource;
impl crate::template::ToResource for TransitGatewayMulticastGroupSource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "TransitGatewayMulticastGroupSource",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "GroupIpAddress".to_string(),
            crate::value::ToValue::to_value(&self.group_ip_address),
        );
        properties.insert(
            "NetworkInterfaceId".to_string(),
            crate::value::ToValue::to_value(&self.network_interface_id),
        );
        properties.insert(
            "TransitGatewayMulticastDomainId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_multicast_domain_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-transitgatewaypeeringattachment.html
pub struct TransitGatewayPeeringAttachment_ {
    pub peer_account_id: crate::value::ExpString,
    pub peer_region: crate::value::ExpString,
    pub peer_transit_gateway_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transit_gateway_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TransitGatewayPeeringAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TransitGatewayPeeringAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TransitGatewayPeeringAttachment as TransitGatewayPeeringAttachment;
impl crate::template::ToResource for TransitGatewayPeeringAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "TransitGatewayPeeringAttachment",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PeerAccountId".to_string(),
            crate::value::ToValue::to_value(&self.peer_account_id),
        );
        properties.insert(
            "PeerRegion".to_string(),
            crate::value::ToValue::to_value(&self.peer_region),
        );
        properties.insert(
            "PeerTransitGatewayId".to_string(),
            crate::value::ToValue::to_value(&self.peer_transit_gateway_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TransitGatewayId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-transitgatewayroute.html
pub struct TransitGatewayRoute_ {
    pub blackhole: Option<crate::value::ExpBool>,
    pub destination_cidr_block: crate::value::ExpString,
    pub transit_gateway_attachment_id: Option<crate::value::ExpString>,
    pub transit_gateway_route_table_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TransitGatewayRoute {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TransitGatewayRoute"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TransitGatewayRoute as TransitGatewayRoute;
impl crate::template::ToResource for TransitGatewayRoute_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TransitGatewayRoute"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.blackhole {
            properties.insert(
                "Blackhole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DestinationCidrBlock".to_string(),
            crate::value::ToValue::to_value(&self.destination_cidr_block),
        );
        if let Some(ref value) = self.transit_gateway_attachment_id {
            properties.insert(
                "TransitGatewayAttachmentId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TransitGatewayRouteTableId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_route_table_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-transitgatewayroutetable.html
pub struct TransitGatewayRouteTable_ {
    pub tags: Option<Vec<crate::Tag_>>,
    pub transit_gateway_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TransitGatewayRouteTable {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TransitGatewayRouteTable"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TransitGatewayRouteTable as TransitGatewayRouteTable;
impl crate::template::ToResource for TransitGatewayRouteTable_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TransitGatewayRouteTable"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TransitGatewayId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-transitgatewayroutetableassociation.html
pub struct TransitGatewayRouteTableAssociation_ {
    pub transit_gateway_attachment_id: crate::value::ExpString,
    pub transit_gateway_route_table_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TransitGatewayRouteTableAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TransitGatewayRouteTableAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TransitGatewayRouteTableAssociation as TransitGatewayRouteTableAssociation;
impl crate::template::ToResource for TransitGatewayRouteTableAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "TransitGatewayRouteTableAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "TransitGatewayAttachmentId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_attachment_id),
        );
        properties.insert(
            "TransitGatewayRouteTableId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_route_table_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-transitgatewayroutetablepropagation.html
pub struct TransitGatewayRouteTablePropagation_ {
    pub transit_gateway_attachment_id: crate::value::ExpString,
    pub transit_gateway_route_table_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TransitGatewayRouteTablePropagation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TransitGatewayRouteTablePropagation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TransitGatewayRouteTablePropagation as TransitGatewayRouteTablePropagation;
impl crate::template::ToResource for TransitGatewayRouteTablePropagation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "TransitGatewayRouteTablePropagation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "TransitGatewayAttachmentId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_attachment_id),
        );
        properties.insert(
            "TransitGatewayRouteTableId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_route_table_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-transitgatewayvpcattachment.html
pub struct TransitGatewayVpcAttachment_ {
    pub add_subnet_ids: Option<Vec<crate::value::ExpString>>,
    pub options: Option<super::ec2::transitgatewayvpcattachment::Options_>,
    pub remove_subnet_ids: Option<Vec<crate::value::ExpString>>,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transit_gateway_id: crate::value::ExpString,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_TransitGatewayVpcAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::TransitGatewayVpcAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_TransitGatewayVpcAttachment as TransitGatewayVpcAttachment;
impl crate::template::ToResource for TransitGatewayVpcAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "TransitGatewayVpcAttachment",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.add_subnet_ids {
            properties.insert(
                "AddSubnetIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.options {
            properties.insert(
                "Options".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.remove_subnet_ids {
            properties.insert(
                "RemoveSubnetIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SubnetIds".to_string(),
            crate::value::ToValue::to_value(&self.subnet_ids),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TransitGatewayId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_id),
        );
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpc.html
pub struct VPC_ {
    pub cidr_block: Option<crate::value::ExpString>,
    pub enable_dns_hostnames: Option<crate::value::ExpBool>,
    pub enable_dns_support: Option<crate::value::ExpBool>,
    pub instance_tenancy: Option<crate::value::ExpString>,
    pub ipv4_ipam_pool_id: Option<crate::value::ExpString>,
    pub ipv4_netmask_length: Option<i32>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPC {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPC" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_VPC as VPC;
impl crate::template::ToResource for VPC_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VPC"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cidr_block {
            properties.insert(
                "CidrBlock".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_dns_hostnames {
            properties.insert(
                "EnableDnsHostnames".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_dns_support {
            properties.insert(
                "EnableDnsSupport".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_tenancy {
            properties.insert(
                "InstanceTenancy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv4_ipam_pool_id {
            properties.insert(
                "Ipv4IpamPoolId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv4_netmask_length {
            properties.insert(
                "Ipv4NetmaskLength".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpcblockpublicaccessexclusion.html
pub struct VPCBlockPublicAccessExclusion_ {
    pub internet_gateway_exclusion_mode: crate::value::ExpString,
    pub subnet_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPCBlockPublicAccessExclusion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPCBlockPublicAccessExclusion"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VPCBlockPublicAccessExclusion as VPCBlockPublicAccessExclusion;
impl crate::template::ToResource for VPCBlockPublicAccessExclusion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "VPCBlockPublicAccessExclusion",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "InternetGatewayExclusionMode".to_string(),
            crate::value::ToValue::to_value(&self.internet_gateway_exclusion_mode),
        );
        if let Some(ref value) = self.subnet_id {
            properties.insert(
                "SubnetId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_id {
            properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpcblockpublicaccessoptions.html
pub struct VPCBlockPublicAccessOptions_ {
    pub internet_gateway_block_mode: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPCBlockPublicAccessOptions {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPCBlockPublicAccessOptions"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VPCBlockPublicAccessOptions as VPCBlockPublicAccessOptions;
impl crate::template::ToResource for VPCBlockPublicAccessOptions_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "VPCBlockPublicAccessOptions",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "InternetGatewayBlockMode".to_string(),
            crate::value::ToValue::to_value(&self.internet_gateway_block_mode),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpccidrblock.html
pub struct VPCCidrBlock_ {
    pub amazon_provided_ipv6_cidr_block: Option<crate::value::ExpBool>,
    pub cidr_block: Option<crate::value::ExpString>,
    pub ipv4_ipam_pool_id: Option<crate::value::ExpString>,
    pub ipv4_netmask_length: Option<i32>,
    pub ipv6_cidr_block: Option<crate::value::ExpString>,
    pub ipv6_cidr_block_network_border_group: Option<crate::value::ExpString>,
    pub ipv6_ipam_pool_id: Option<crate::value::ExpString>,
    pub ipv6_netmask_length: Option<i32>,
    pub ipv6_pool: Option<crate::value::ExpString>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPCCidrBlock {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPCCidrBlock"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VPCCidrBlock as VPCCidrBlock;
impl crate::template::ToResource for VPCCidrBlock_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VPCCidrBlock"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.amazon_provided_ipv6_cidr_block {
            properties.insert(
                "AmazonProvidedIpv6CidrBlock".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cidr_block {
            properties.insert(
                "CidrBlock".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv4_ipam_pool_id {
            properties.insert(
                "Ipv4IpamPoolId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv4_netmask_length {
            properties.insert(
                "Ipv4NetmaskLength".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_cidr_block {
            properties.insert(
                "Ipv6CidrBlock".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_cidr_block_network_border_group {
            properties.insert(
                "Ipv6CidrBlockNetworkBorderGroup".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_ipam_pool_id {
            properties.insert(
                "Ipv6IpamPoolId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_netmask_length {
            properties.insert(
                "Ipv6NetmaskLength".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_pool {
            properties.insert(
                "Ipv6Pool".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpcdhcpoptionsassociation.html
pub struct VPCDHCPOptionsAssociation_ {
    pub dhcp_options_id: crate::value::ExpString,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPCDHCPOptionsAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPCDHCPOptionsAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VPCDHCPOptionsAssociation as VPCDHCPOptionsAssociation;
impl crate::template::ToResource for VPCDHCPOptionsAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VPCDHCPOptionsAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DhcpOptionsId".to_string(),
            crate::value::ToValue::to_value(&self.dhcp_options_id),
        );
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpcencryptioncontrol.html
pub struct VPCEncryptionControl_ {
    pub egress_only_internet_gateway_exclusion_input: Option<crate::value::ExpString>,
    pub elastic_file_system_exclusion_input: Option<crate::value::ExpString>,
    pub internet_gateway_exclusion_input: Option<crate::value::ExpString>,
    pub lambda_exclusion_input: Option<crate::value::ExpString>,
    pub mode: Option<crate::value::ExpString>,
    pub nat_gateway_exclusion_input: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub virtual_private_gateway_exclusion_input: Option<crate::value::ExpString>,
    pub vpc_id: Option<crate::value::ExpString>,
    pub vpc_lattice_exclusion_input: Option<crate::value::ExpString>,
    pub vpc_peering_exclusion_input: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPCEncryptionControl {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPCEncryptionControl"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VPCEncryptionControl as VPCEncryptionControl;
impl crate::template::ToResource for VPCEncryptionControl_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VPCEncryptionControl"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.egress_only_internet_gateway_exclusion_input {
            properties.insert(
                "EgressOnlyInternetGatewayExclusionInput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.elastic_file_system_exclusion_input {
            properties.insert(
                "ElasticFileSystemExclusionInput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.internet_gateway_exclusion_input {
            properties.insert(
                "InternetGatewayExclusionInput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lambda_exclusion_input {
            properties.insert(
                "LambdaExclusionInput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.mode {
            properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.nat_gateway_exclusion_input {
            properties.insert(
                "NatGatewayExclusionInput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.virtual_private_gateway_exclusion_input {
            properties.insert(
                "VirtualPrivateGatewayExclusionInput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_id {
            properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_lattice_exclusion_input {
            properties.insert(
                "VpcLatticeExclusionInput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_peering_exclusion_input {
            properties.insert(
                "VpcPeeringExclusionInput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpcendpoint.html
pub struct VPCEndpoint_ {
    pub dns_options: Option<super::ec2::vpcendpoint::DnsOptionsSpecification_>,
    pub ip_address_type: Option<crate::value::ExpString>,
    pub policy_document: Option<serde_json::Value>,
    pub private_dns_enabled: Option<crate::value::ExpBool>,
    pub resource_configuration_arn: Option<crate::value::ExpString>,
    pub route_table_ids: Option<Vec<crate::value::ExpString>>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub service_name: Option<crate::value::ExpString>,
    pub service_network_arn: Option<crate::value::ExpString>,
    pub service_region: Option<crate::value::ExpString>,
    pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_endpoint_type: Option<crate::value::ExpString>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPCEndpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPCEndpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VPCEndpoint as VPCEndpoint;
impl crate::template::ToResource for VPCEndpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VPCEndpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.dns_options {
            properties.insert(
                "DnsOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ip_address_type {
            properties.insert(
                "IpAddressType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policy_document {
            properties.insert(
                "PolicyDocument".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.private_dns_enabled {
            properties.insert(
                "PrivateDnsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_configuration_arn {
            properties.insert(
                "ResourceConfigurationArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.route_table_ids {
            properties.insert(
                "RouteTableIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_name {
            properties.insert(
                "ServiceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_network_arn {
            properties.insert(
                "ServiceNetworkArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_region {
            properties.insert(
                "ServiceRegion".to_string(),
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
        if let Some(ref value) = self.vpc_endpoint_type {
            properties.insert(
                "VpcEndpointType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpcendpointconnectionnotification.html
pub struct VPCEndpointConnectionNotification_ {
    pub connection_events: Vec<crate::value::ExpString>,
    pub connection_notification_arn: crate::value::ExpString,
    pub service_id: Option<crate::value::ExpString>,
    pub vpc_endpoint_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPCEndpointConnectionNotification {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPCEndpointConnectionNotification"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VPCEndpointConnectionNotification as VPCEndpointConnectionNotification;
impl crate::template::ToResource for VPCEndpointConnectionNotification_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "VPCEndpointConnectionNotification",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConnectionEvents".to_string(),
            crate::value::ToValue::to_value(&self.connection_events),
        );
        properties.insert(
            "ConnectionNotificationArn".to_string(),
            crate::value::ToValue::to_value(&self.connection_notification_arn),
        );
        if let Some(ref value) = self.service_id {
            properties.insert(
                "ServiceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_endpoint_id {
            properties.insert(
                "VPCEndpointId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpcendpointservice.html
pub struct VPCEndpointService_ {
    pub acceptance_required: Option<crate::value::ExpBool>,
    pub contributor_insights_enabled: Option<crate::value::ExpBool>,
    pub gateway_load_balancer_arns: Option<Vec<crate::value::ExpString>>,
    pub network_load_balancer_arns: Option<Vec<crate::value::ExpString>>,
    pub payer_responsibility: Option<crate::value::ExpString>,
    pub supported_ip_address_types: Option<Vec<crate::value::ExpString>>,
    pub supported_regions: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPCEndpointService {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPCEndpointService"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VPCEndpointService as VPCEndpointService;
impl crate::template::ToResource for VPCEndpointService_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VPCEndpointService"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.acceptance_required {
            properties.insert(
                "AcceptanceRequired".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.contributor_insights_enabled {
            properties.insert(
                "ContributorInsightsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.gateway_load_balancer_arns {
            properties.insert(
                "GatewayLoadBalancerArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_load_balancer_arns {
            properties.insert(
                "NetworkLoadBalancerArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.payer_responsibility {
            properties.insert(
                "PayerResponsibility".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.supported_ip_address_types {
            properties.insert(
                "SupportedIpAddressTypes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.supported_regions {
            properties.insert(
                "SupportedRegions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpcendpointservicepermissions.html
pub struct VPCEndpointServicePermissions_ {
    pub allowed_principals: Option<Vec<crate::value::ExpString>>,
    pub service_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPCEndpointServicePermissions {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPCEndpointServicePermissions"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VPCEndpointServicePermissions as VPCEndpointServicePermissions;
impl crate::template::ToResource for VPCEndpointServicePermissions_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "VPCEndpointServicePermissions",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allowed_principals {
            properties.insert(
                "AllowedPrincipals".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ServiceId".to_string(),
            crate::value::ToValue::to_value(&self.service_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpcgatewayattachment.html
pub struct VPCGatewayAttachment_ {
    pub internet_gateway_id: Option<crate::value::ExpString>,
    pub vpc_id: crate::value::ExpString,
    pub vpn_gateway_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPCGatewayAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPCGatewayAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VPCGatewayAttachment as VPCGatewayAttachment;
impl crate::template::ToResource for VPCGatewayAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VPCGatewayAttachment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.internet_gateway_id {
            properties.insert(
                "InternetGatewayId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        if let Some(ref value) = self.vpn_gateway_id {
            properties.insert(
                "VpnGatewayId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpcpeeringconnection.html
pub struct VPCPeeringConnection_ {
    pub peer_owner_id: Option<crate::value::ExpString>,
    pub peer_region: Option<crate::value::ExpString>,
    pub peer_role_arn: Option<crate::value::ExpString>,
    pub peer_vpc_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPCPeeringConnection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPCPeeringConnection"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VPCPeeringConnection as VPCPeeringConnection;
impl crate::template::ToResource for VPCPeeringConnection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VPCPeeringConnection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.peer_owner_id {
            properties.insert(
                "PeerOwnerId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.peer_region {
            properties.insert(
                "PeerRegion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.peer_role_arn {
            properties.insert(
                "PeerRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PeerVpcId".to_string(),
            crate::value::ToValue::to_value(&self.peer_vpc_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpnconcentrator.html
pub struct VPNConcentrator_ {
    pub tags: Option<Vec<crate::Tag_>>,
    pub transit_gateway_id: crate::value::ExpString,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPNConcentrator {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPNConcentrator"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VPNConcentrator as VPNConcentrator;
impl crate::template::ToResource for VPNConcentrator_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VPNConcentrator"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TransitGatewayId".to_string(),
            crate::value::ToValue::to_value(&self.transit_gateway_id),
        );
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpnconnection.html
pub struct VPNConnection_ {
    pub customer_gateway_id: crate::value::ExpString,
    pub enable_acceleration: Option<crate::value::ExpBool>,
    pub local_ipv4_network_cidr: Option<crate::value::ExpString>,
    pub local_ipv6_network_cidr: Option<crate::value::ExpString>,
    pub outside_ip_address_type: Option<crate::value::ExpString>,
    pub pre_shared_key_storage: Option<crate::value::ExpString>,
    pub remote_ipv4_network_cidr: Option<crate::value::ExpString>,
    pub remote_ipv6_network_cidr: Option<crate::value::ExpString>,
    pub static_routes_only: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transit_gateway_id: Option<crate::value::ExpString>,
    pub transport_transit_gateway_attachment_id: Option<crate::value::ExpString>,
    pub tunnel_bandwidth: Option<crate::value::ExpString>,
    pub tunnel_inside_ip_version: Option<crate::value::ExpString>,
    pub r#type: crate::value::ExpString,
    pub vpn_concentrator_id: Option<crate::value::ExpString>,
    pub vpn_gateway_id: Option<crate::value::ExpString>,
    pub vpn_tunnel_options_specifications:
        Option<Vec<super::ec2::vpnconnection::VpnTunnelOptionsSpecification_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPNConnection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPNConnection"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VPNConnection as VPNConnection;
impl crate::template::ToResource for VPNConnection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VPNConnection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CustomerGatewayId".to_string(),
            crate::value::ToValue::to_value(&self.customer_gateway_id),
        );
        if let Some(ref value) = self.enable_acceleration {
            properties.insert(
                "EnableAcceleration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.local_ipv4_network_cidr {
            properties.insert(
                "LocalIpv4NetworkCidr".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.local_ipv6_network_cidr {
            properties.insert(
                "LocalIpv6NetworkCidr".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.outside_ip_address_type {
            properties.insert(
                "OutsideIpAddressType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.pre_shared_key_storage {
            properties.insert(
                "PreSharedKeyStorage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.remote_ipv4_network_cidr {
            properties.insert(
                "RemoteIpv4NetworkCidr".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.remote_ipv6_network_cidr {
            properties.insert(
                "RemoteIpv6NetworkCidr".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.static_routes_only {
            properties.insert(
                "StaticRoutesOnly".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.transit_gateway_id {
            properties.insert(
                "TransitGatewayId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.transport_transit_gateway_attachment_id {
            properties.insert(
                "TransportTransitGatewayAttachmentId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tunnel_bandwidth {
            properties.insert(
                "TunnelBandwidth".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tunnel_inside_ip_version {
            properties.insert(
                "TunnelInsideIpVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        if let Some(ref value) = self.vpn_concentrator_id {
            properties.insert(
                "VpnConcentratorId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpn_gateway_id {
            properties.insert(
                "VpnGatewayId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpn_tunnel_options_specifications {
            properties.insert(
                "VpnTunnelOptionsSpecifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpnconnectionroute.html
pub struct VPNConnectionRoute_ {
    pub destination_cidr_block: crate::value::ExpString,
    pub vpn_connection_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPNConnectionRoute {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPNConnectionRoute"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VPNConnectionRoute as VPNConnectionRoute;
impl crate::template::ToResource for VPNConnectionRoute_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VPNConnectionRoute"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DestinationCidrBlock".to_string(),
            crate::value::ToValue::to_value(&self.destination_cidr_block),
        );
        properties.insert(
            "VpnConnectionId".to_string(),
            crate::value::ToValue::to_value(&self.vpn_connection_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpngateway.html
pub struct VPNGateway_ {
    pub amazon_side_asn: Option<i64>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPNGateway {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPNGateway" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_VPNGateway as VPNGateway;
impl crate::template::ToResource for VPNGateway_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VPNGateway"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.amazon_side_asn {
            properties.insert(
                "AmazonSideAsn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-vpngatewayroutepropagation.html
pub struct VPNGatewayRoutePropagation_ {
    pub route_table_ids: Vec<crate::value::ExpString>,
    pub vpn_gateway_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VPNGatewayRoutePropagation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VPNGatewayRoutePropagation"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VPNGatewayRoutePropagation as VPNGatewayRoutePropagation;
impl crate::template::ToResource for VPNGatewayRoutePropagation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "VPNGatewayRoutePropagation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "RouteTableIds".to_string(),
            crate::value::ToValue::to_value(&self.route_table_ids),
        );
        properties.insert(
            "VpnGatewayId".to_string(),
            crate::value::ToValue::to_value(&self.vpn_gateway_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-verifiedaccessendpoint.html
pub struct VerifiedAccessEndpoint_ {
    pub application_domain: Option<crate::value::ExpString>,
    pub attachment_type: crate::value::ExpString,
    pub cidr_options: Option<super::ec2::verifiedaccessendpoint::CidrOptions_>,
    pub description: Option<crate::value::ExpString>,
    pub domain_certificate_arn: Option<crate::value::ExpString>,
    pub endpoint_domain_prefix: Option<crate::value::ExpString>,
    pub endpoint_type: crate::value::ExpString,
    pub load_balancer_options: Option<super::ec2::verifiedaccessendpoint::LoadBalancerOptions_>,
    pub network_interface_options:
        Option<super::ec2::verifiedaccessendpoint::NetworkInterfaceOptions_>,
    pub policy_document: Option<crate::value::ExpString>,
    pub policy_enabled: Option<crate::value::ExpBool>,
    pub rds_options: Option<super::ec2::verifiedaccessendpoint::RdsOptions_>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub sse_specification: Option<super::ec2::verifiedaccessendpoint::SseSpecification_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub verified_access_group_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VerifiedAccessEndpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VerifiedAccessEndpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VerifiedAccessEndpoint as VerifiedAccessEndpoint;
impl crate::template::ToResource for VerifiedAccessEndpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VerifiedAccessEndpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.application_domain {
            properties.insert(
                "ApplicationDomain".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AttachmentType".to_string(),
            crate::value::ToValue::to_value(&self.attachment_type),
        );
        if let Some(ref value) = self.cidr_options {
            properties.insert(
                "CidrOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_certificate_arn {
            properties.insert(
                "DomainCertificateArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint_domain_prefix {
            properties.insert(
                "EndpointDomainPrefix".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EndpointType".to_string(),
            crate::value::ToValue::to_value(&self.endpoint_type),
        );
        if let Some(ref value) = self.load_balancer_options {
            properties.insert(
                "LoadBalancerOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_interface_options {
            properties.insert(
                "NetworkInterfaceOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policy_document {
            properties.insert(
                "PolicyDocument".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policy_enabled {
            properties.insert(
                "PolicyEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.rds_options {
            properties.insert(
                "RdsOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sse_specification {
            properties.insert(
                "SseSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VerifiedAccessGroupId".to_string(),
            crate::value::ToValue::to_value(&self.verified_access_group_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-verifiedaccessgroup.html
pub struct VerifiedAccessGroup_ {
    pub description: Option<crate::value::ExpString>,
    pub policy_document: Option<crate::value::ExpString>,
    pub policy_enabled: Option<crate::value::ExpBool>,
    pub sse_specification: Option<super::ec2::verifiedaccessgroup::SseSpecification_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub verified_access_instance_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VerifiedAccessGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VerifiedAccessGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VerifiedAccessGroup as VerifiedAccessGroup;
impl crate::template::ToResource for VerifiedAccessGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VerifiedAccessGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policy_document {
            properties.insert(
                "PolicyDocument".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policy_enabled {
            properties.insert(
                "PolicyEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sse_specification {
            properties.insert(
                "SseSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VerifiedAccessInstanceId".to_string(),
            crate::value::ToValue::to_value(&self.verified_access_instance_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-verifiedaccessinstance.html
pub struct VerifiedAccessInstance_ {
    pub cidr_endpoints_custom_sub_domain: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub fips_enabled: Option<crate::value::ExpBool>,
    pub logging_configurations: Option<super::ec2::verifiedaccessinstance::VerifiedAccessLogs_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub verified_access_trust_provider_ids: Option<Vec<crate::value::ExpString>>,
    pub verified_access_trust_providers:
        Option<Vec<super::ec2::verifiedaccessinstance::VerifiedAccessTrustProvider_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VerifiedAccessInstance {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VerifiedAccessInstance"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VerifiedAccessInstance as VerifiedAccessInstance;
impl crate::template::ToResource for VerifiedAccessInstance_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VerifiedAccessInstance"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cidr_endpoints_custom_sub_domain {
            properties.insert(
                "CidrEndpointsCustomSubDomain".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.fips_enabled {
            properties.insert(
                "FipsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logging_configurations {
            properties.insert(
                "LoggingConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.verified_access_trust_provider_ids {
            properties.insert(
                "VerifiedAccessTrustProviderIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.verified_access_trust_providers {
            properties.insert(
                "VerifiedAccessTrustProviders".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-verifiedaccesstrustprovider.html
pub struct VerifiedAccessTrustProvider_ {
    pub description: Option<crate::value::ExpString>,
    pub device_options: Option<super::ec2::verifiedaccesstrustprovider::DeviceOptions_>,
    pub device_trust_provider_type: Option<crate::value::ExpString>,
    pub native_application_oidc_options:
        Option<super::ec2::verifiedaccesstrustprovider::NativeApplicationOidcOptions_>,
    pub oidc_options: Option<super::ec2::verifiedaccesstrustprovider::OidcOptions_>,
    pub policy_reference_name: crate::value::ExpString,
    pub sse_specification: Option<super::ec2::verifiedaccesstrustprovider::SseSpecification_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub trust_provider_type: crate::value::ExpString,
    pub user_trust_provider_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VerifiedAccessTrustProvider {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VerifiedAccessTrustProvider"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VerifiedAccessTrustProvider as VerifiedAccessTrustProvider;
impl crate::template::ToResource for VerifiedAccessTrustProvider_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "VerifiedAccessTrustProvider",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.device_options {
            properties.insert(
                "DeviceOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.device_trust_provider_type {
            properties.insert(
                "DeviceTrustProviderType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.native_application_oidc_options {
            properties.insert(
                "NativeApplicationOidcOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.oidc_options {
            properties.insert(
                "OidcOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PolicyReferenceName".to_string(),
            crate::value::ToValue::to_value(&self.policy_reference_name),
        );
        if let Some(ref value) = self.sse_specification {
            properties.insert(
                "SseSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TrustProviderType".to_string(),
            crate::value::ToValue::to_value(&self.trust_provider_type),
        );
        if let Some(ref value) = self.user_trust_provider_type {
            properties.insert(
                "UserTrustProviderType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-volume.html
pub struct Volume_ {
    pub auto_enable_io: Option<crate::value::ExpBool>,
    pub availability_zone: Option<crate::value::ExpString>,
    pub availability_zone_id: Option<crate::value::ExpString>,
    pub encrypted: Option<crate::value::ExpBool>,
    pub iops: Option<i32>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub multi_attach_enabled: Option<crate::value::ExpBool>,
    pub outpost_arn: Option<crate::value::ExpString>,
    pub size: Option<i32>,
    pub snapshot_id: Option<crate::value::ExpString>,
    pub source_volume_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub throughput: Option<i32>,
    pub volume_initialization_rate: Option<i32>,
    pub volume_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_Volume {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::Volume" $($field
        $value)*)
    };
}
pub use crate::__aws_ec2_Volume as Volume;
impl crate::template::ToResource for Volume_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Volume"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_enable_io {
            properties.insert(
                "AutoEnableIO".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
        if let Some(ref value) = self.multi_attach_enabled {
            properties.insert(
                "MultiAttachEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.outpost_arn {
            properties.insert(
                "OutpostArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.size {
            properties.insert("Size".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.snapshot_id {
            properties.insert(
                "SnapshotId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_volume_id {
            properties.insert(
                "SourceVolumeId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.throughput {
            properties.insert(
                "Throughput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.volume_initialization_rate {
            properties.insert(
                "VolumeInitializationRate".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ec2-volumeattachment.html
pub struct VolumeAttachment_ {
    pub device: Option<crate::value::ExpString>,
    pub ebs_card_index: Option<i32>,
    pub instance_id: crate::value::ExpString,
    pub volume_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ec2_VolumeAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EC2::VolumeAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_ec2_VolumeAttachment as VolumeAttachment;
impl crate::template::ToResource for VolumeAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EC2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VolumeAttachment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.device {
            properties.insert("Device".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.ebs_card_index {
            properties.insert(
                "EbsCardIndex".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceId".to_string(),
            crate::value::ToValue::to_value(&self.instance_id),
        );
        properties.insert(
            "VolumeId".to_string(),
            crate::value::ToValue::to_value(&self.volume_id),
        );
        properties
    }
}
