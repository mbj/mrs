pub mod fleet {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-acceleratorcapabilities.html
    pub struct AcceleratorCapabilities_ {
        pub count: Option<Box<AcceleratorCountRange_>>,
        pub selections: Vec<AcceleratorSelection_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_AcceleratorCapabilities {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.AcceleratorCapabilities"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_AcceleratorCapabilities as AcceleratorCapabilities;
    impl crate::value::ToValue for AcceleratorCapabilities_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.count {
                properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Selections".to_string(),
                crate::value::ToValue::to_value(&self.selections),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-acceleratorcountrange.html
    pub struct AcceleratorCountRange_ {
        pub max: Option<i64>,
        pub min: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_AcceleratorCountRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.AcceleratorCountRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_AcceleratorCountRange as AcceleratorCountRange;
    impl crate::value::ToValue for AcceleratorCountRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Min".to_string(),
                crate::value::ToValue::to_value(&self.min),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-acceleratorselection.html
    pub struct AcceleratorSelection_ {
        pub name: crate::value::ExpString,
        pub runtime: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_AcceleratorSelection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.AcceleratorSelection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_AcceleratorSelection as AcceleratorSelection;
    impl crate::value::ToValue for AcceleratorSelection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.runtime {
                properties.insert(
                    "Runtime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-acceleratortotalmemorymibrange.html
    pub struct AcceleratorTotalMemoryMiBRange_ {
        pub max: Option<i64>,
        pub min: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_AcceleratorTotalMemoryMiBRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.AcceleratorTotalMemoryMiBRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_AcceleratorTotalMemoryMiBRange as AcceleratorTotalMemoryMiBRange;
    impl crate::value::ToValue for AcceleratorTotalMemoryMiBRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Min".to_string(),
                crate::value::ToValue::to_value(&self.min),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-customermanagedfleetconfiguration.html
    pub struct CustomerManagedFleetConfiguration_ {
        pub mode: crate::value::ExpString,
        pub storage_profile_id: Option<crate::value::ExpString>,
        pub tag_propagation_mode: Option<crate::value::ExpString>,
        pub worker_capabilities: Box<CustomerManagedWorkerCapabilities_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_CustomerManagedFleetConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.CustomerManagedFleetConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_CustomerManagedFleetConfiguration as CustomerManagedFleetConfiguration;
    impl crate::value::ToValue for CustomerManagedFleetConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Mode".to_string(),
                crate::value::ToValue::to_value(&self.mode),
            );
            if let Some(ref value) = self.storage_profile_id {
                properties.insert(
                    "StorageProfileId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_propagation_mode {
                properties.insert(
                    "TagPropagationMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "WorkerCapabilities".to_string(),
                crate::value::ToValue::to_value(&self.worker_capabilities),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-customermanagedworkercapabilities.html
    pub struct CustomerManagedWorkerCapabilities_ {
        pub accelerator_count: Option<Box<AcceleratorCountRange_>>,
        pub accelerator_total_memory_mi_b: Option<Box<AcceleratorTotalMemoryMiBRange_>>,
        pub accelerator_types: Option<Vec<crate::value::ExpString>>,
        pub cpu_architecture_type: crate::value::ExpString,
        pub custom_amounts: Option<Vec<FleetAmountCapability_>>,
        pub custom_attributes: Option<Vec<FleetAttributeCapability_>>,
        pub memory_mi_b: Box<MemoryMiBRange_>,
        pub os_family: crate::value::ExpString,
        pub v_cpu_count: Box<VCpuCountRange_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_CustomerManagedWorkerCapabilities {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.CustomerManagedWorkerCapabilities"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_CustomerManagedWorkerCapabilities as CustomerManagedWorkerCapabilities;
    impl crate::value::ToValue for CustomerManagedWorkerCapabilities_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.accelerator_count {
                properties.insert(
                    "AcceleratorCount".to_string(),
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
            properties.insert(
                "CpuArchitectureType".to_string(),
                crate::value::ToValue::to_value(&self.cpu_architecture_type),
            );
            if let Some(ref value) = self.custom_amounts {
                properties.insert(
                    "CustomAmounts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_attributes {
                properties.insert(
                    "CustomAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MemoryMiB".to_string(),
                crate::value::ToValue::to_value(&self.memory_mi_b),
            );
            properties.insert(
                "OsFamily".to_string(),
                crate::value::ToValue::to_value(&self.os_family),
            );
            properties.insert(
                "VCpuCount".to_string(),
                crate::value::ToValue::to_value(&self.v_cpu_count),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-ec2ebsvolume.html
    pub struct Ec2EbsVolume_ {
        pub iops: Option<i64>,
        pub size_gi_b: Option<i64>,
        pub throughput_mi_b: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_Ec2EbsVolume {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.Ec2EbsVolume"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_Ec2EbsVolume as Ec2EbsVolume;
    impl crate::value::ToValue for Ec2EbsVolume_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.iops {
                properties.insert("Iops".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.size_gi_b {
                properties.insert(
                    "SizeGiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.throughput_mi_b {
                properties.insert(
                    "ThroughputMiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-fleetamountcapability.html
    pub struct FleetAmountCapability_ {
        pub max: Option<f64>,
        pub min: f64,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_FleetAmountCapability {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.FleetAmountCapability"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_FleetAmountCapability as FleetAmountCapability;
    impl crate::value::ToValue for FleetAmountCapability_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Min".to_string(),
                crate::value::ToValue::to_value(&self.min),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-fleetattributecapability.html
    pub struct FleetAttributeCapability_ {
        pub name: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_FleetAttributeCapability {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.FleetAttributeCapability"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_FleetAttributeCapability as FleetAttributeCapability;
    impl crate::value::ToValue for FleetAttributeCapability_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-fleetcapabilities.html
    pub struct FleetCapabilities_ {
        pub amounts: Option<Vec<FleetAmountCapability_>>,
        pub attributes: Option<Vec<FleetAttributeCapability_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_FleetCapabilities {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.FleetCapabilities"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_FleetCapabilities as FleetCapabilities;
    impl crate::value::ToValue for FleetCapabilities_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.amounts {
                properties.insert(
                    "Amounts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.attributes {
                properties.insert(
                    "Attributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-fleetconfiguration.html
    pub struct FleetConfiguration_ {
        pub customer_managed: Option<Box<CustomerManagedFleetConfiguration_>>,
        pub service_managed_ec2: Option<Box<ServiceManagedEc2FleetConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_FleetConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.FleetConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_FleetConfiguration as FleetConfiguration;
    impl crate::value::ToValue for FleetConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customer_managed {
                properties.insert(
                    "CustomerManaged".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_managed_ec2 {
                properties.insert(
                    "ServiceManagedEc2".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-hostconfiguration.html
    pub struct HostConfiguration_ {
        pub script_body: crate::value::ExpString,
        pub script_timeout_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_HostConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.HostConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_HostConfiguration as HostConfiguration;
    impl crate::value::ToValue for HostConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ScriptBody".to_string(),
                crate::value::ToValue::to_value(&self.script_body),
            );
            if let Some(ref value) = self.script_timeout_seconds {
                properties.insert(
                    "ScriptTimeoutSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-memorymibrange.html
    pub struct MemoryMiBRange_ {
        pub max: Option<i64>,
        pub min: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_MemoryMiBRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.MemoryMiBRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_MemoryMiBRange as MemoryMiBRange;
    impl crate::value::ToValue for MemoryMiBRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Min".to_string(),
                crate::value::ToValue::to_value(&self.min),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-servicemanagedec2fleetconfiguration.html
    pub struct ServiceManagedEc2FleetConfiguration_ {
        pub instance_capabilities: Box<ServiceManagedEc2InstanceCapabilities_>,
        pub instance_market_options: Box<ServiceManagedEc2InstanceMarketOptions_>,
        pub storage_profile_id: Option<crate::value::ExpString>,
        pub vpc_configuration: Option<Box<VpcConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_ServiceManagedEc2FleetConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.ServiceManagedEc2FleetConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_ServiceManagedEc2FleetConfiguration as ServiceManagedEc2FleetConfiguration;
    impl crate::value::ToValue for ServiceManagedEc2FleetConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceCapabilities".to_string(),
                crate::value::ToValue::to_value(&self.instance_capabilities),
            );
            properties.insert(
                "InstanceMarketOptions".to_string(),
                crate::value::ToValue::to_value(&self.instance_market_options),
            );
            if let Some(ref value) = self.storage_profile_id {
                properties.insert(
                    "StorageProfileId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_configuration {
                properties.insert(
                    "VpcConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-servicemanagedec2instancecapabilities.html
    pub struct ServiceManagedEc2InstanceCapabilities_ {
        pub accelerator_capabilities: Option<Box<AcceleratorCapabilities_>>,
        pub allowed_instance_types: Option<Vec<crate::value::ExpString>>,
        pub cpu_architecture_type: crate::value::ExpString,
        pub custom_amounts: Option<Vec<FleetAmountCapability_>>,
        pub custom_attributes: Option<Vec<FleetAttributeCapability_>>,
        pub excluded_instance_types: Option<Vec<crate::value::ExpString>>,
        pub memory_mi_b: Box<MemoryMiBRange_>,
        pub os_family: crate::value::ExpString,
        pub root_ebs_volume: Option<Box<Ec2EbsVolume_>>,
        pub v_cpu_count: Box<VCpuCountRange_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_ServiceManagedEc2InstanceCapabilities {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.ServiceManagedEc2InstanceCapabilities"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_ServiceManagedEc2InstanceCapabilities as ServiceManagedEc2InstanceCapabilities;
    impl crate::value::ToValue for ServiceManagedEc2InstanceCapabilities_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.accelerator_capabilities {
                properties.insert(
                    "AcceleratorCapabilities".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allowed_instance_types {
                properties.insert(
                    "AllowedInstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "CpuArchitectureType".to_string(),
                crate::value::ToValue::to_value(&self.cpu_architecture_type),
            );
            if let Some(ref value) = self.custom_amounts {
                properties.insert(
                    "CustomAmounts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_attributes {
                properties.insert(
                    "CustomAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.excluded_instance_types {
                properties.insert(
                    "ExcludedInstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MemoryMiB".to_string(),
                crate::value::ToValue::to_value(&self.memory_mi_b),
            );
            properties.insert(
                "OsFamily".to_string(),
                crate::value::ToValue::to_value(&self.os_family),
            );
            if let Some(ref value) = self.root_ebs_volume {
                properties.insert(
                    "RootEbsVolume".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VCpuCount".to_string(),
                crate::value::ToValue::to_value(&self.v_cpu_count),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-servicemanagedec2instancemarketoptions.html
    pub struct ServiceManagedEc2InstanceMarketOptions_ {
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_ServiceManagedEc2InstanceMarketOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.ServiceManagedEc2InstanceMarketOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_ServiceManagedEc2InstanceMarketOptions as ServiceManagedEc2InstanceMarketOptions;
    impl crate::value::ToValue for ServiceManagedEc2InstanceMarketOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-vcpucountrange.html
    pub struct VCpuCountRange_ {
        pub max: Option<i64>,
        pub min: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_VCpuCountRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.VCpuCountRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_VCpuCountRange as VCpuCountRange;
    impl crate::value::ToValue for VCpuCountRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Min".to_string(),
                crate::value::ToValue::to_value(&self.min),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-fleet-vpcconfiguration.html
    pub struct VpcConfiguration_ {
        pub resource_configuration_arns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Fleet_VpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Fleet.VpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Fleet_VpcConfiguration as VpcConfiguration;
    impl crate::value::ToValue for VpcConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.resource_configuration_arns {
                properties.insert(
                    "ResourceConfigurationArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod queue {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-queue-jobattachmentsettings.html
    pub struct JobAttachmentSettings_ {
        pub root_prefix: crate::value::ExpString,
        pub s3_bucket_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Queue_JobAttachmentSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Queue.JobAttachmentSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Queue_JobAttachmentSettings as JobAttachmentSettings;
    impl crate::value::ToValue for JobAttachmentSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RootPrefix".to_string(),
                crate::value::ToValue::to_value(&self.root_prefix),
            );
            properties.insert(
                "S3BucketName".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-queue-jobrunasuser.html
    pub struct JobRunAsUser_ {
        pub posix: Option<Box<PosixUser_>>,
        pub run_as: crate::value::ExpString,
        pub windows: Option<Box<WindowsUser_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Queue_JobRunAsUser {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Queue.JobRunAsUser"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Queue_JobRunAsUser as JobRunAsUser;
    impl crate::value::ToValue for JobRunAsUser_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.posix {
                properties.insert("Posix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "RunAs".to_string(),
                crate::value::ToValue::to_value(&self.run_as),
            );
            if let Some(ref value) = self.windows {
                properties.insert(
                    "Windows".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-queue-posixuser.html
    pub struct PosixUser_ {
        pub group: crate::value::ExpString,
        pub user: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Queue_PosixUser {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Queue.PosixUser"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Queue_PosixUser as PosixUser;
    impl crate::value::ToValue for PosixUser_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Group".to_string(),
                crate::value::ToValue::to_value(&self.group),
            );
            properties.insert(
                "User".to_string(),
                crate::value::ToValue::to_value(&self.user),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-queue-windowsuser.html
    pub struct WindowsUser_ {
        pub password_arn: crate::value::ExpString,
        pub user: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_Queue_WindowsUser {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::Queue.WindowsUser"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_Queue_WindowsUser as WindowsUser;
    impl crate::value::ToValue for WindowsUser_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PasswordArn".to_string(),
                crate::value::ToValue::to_value(&self.password_arn),
            );
            properties.insert(
                "User".to_string(),
                crate::value::ToValue::to_value(&self.user),
            );
            properties.into()
        }
    }
}
pub mod storageprofile {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-deadline-storageprofile-filesystemlocation.html
    pub struct FileSystemLocation_ {
        pub name: crate::value::ExpString,
        pub path: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_deadline_StorageProfile_FileSystemLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Deadline::StorageProfile.FileSystemLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_deadline_StorageProfile_FileSystemLocation as FileSystemLocation;
    impl crate::value::ToValue for FileSystemLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Path".to_string(),
                crate::value::ToValue::to_value(&self.path),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-deadline-farm.html
pub struct Farm_ {
    pub description: Option<crate::value::ExpString>,
    pub display_name: crate::value::ExpString,
    pub kms_key_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_deadline_Farm {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Deadline::Farm" $($field
        $value)*)
    };
}
pub use crate::__aws_deadline_Farm as Farm;
impl crate::template::ToResource for Farm_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Deadline"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Farm"),
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
            "DisplayName".to_string(),
            crate::value::ToValue::to_value(&self.display_name),
        );
        if let Some(ref value) = self.kms_key_arn {
            properties.insert(
                "KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-deadline-fleet.html
pub struct Fleet_ {
    pub configuration: super::deadline::fleet::FleetConfiguration_,
    pub description: Option<crate::value::ExpString>,
    pub display_name: crate::value::ExpString,
    pub farm_id: crate::value::ExpString,
    pub host_configuration: Option<super::deadline::fleet::HostConfiguration_>,
    pub max_worker_count: i64,
    pub min_worker_count: Option<i64>,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_deadline_Fleet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Deadline::Fleet" $($field
        $value)*)
    };
}
pub use crate::__aws_deadline_Fleet as Fleet;
impl crate::template::ToResource for Fleet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Deadline"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Fleet"),
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
            "DisplayName".to_string(),
            crate::value::ToValue::to_value(&self.display_name),
        );
        properties.insert(
            "FarmId".to_string(),
            crate::value::ToValue::to_value(&self.farm_id),
        );
        if let Some(ref value) = self.host_configuration {
            properties.insert(
                "HostConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MaxWorkerCount".to_string(),
            crate::value::ToValue::to_value(&self.max_worker_count),
        );
        if let Some(ref value) = self.min_worker_count {
            properties.insert(
                "MinWorkerCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-deadline-licenseendpoint.html
pub struct LicenseEndpoint_ {
    pub security_group_ids: Vec<crate::value::ExpString>,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_deadline_LicenseEndpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Deadline::LicenseEndpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_deadline_LicenseEndpoint as LicenseEndpoint;
impl crate::template::ToResource for LicenseEndpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Deadline"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LicenseEndpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "SecurityGroupIds".to_string(),
            crate::value::ToValue::to_value(&self.security_group_ids),
        );
        properties.insert(
            "SubnetIds".to_string(),
            crate::value::ToValue::to_value(&self.subnet_ids),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-deadline-limit.html
pub struct Limit_ {
    pub amount_requirement_name: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub display_name: crate::value::ExpString,
    pub farm_id: crate::value::ExpString,
    pub max_count: i64,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_deadline_Limit {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Deadline::Limit" $($field
        $value)*)
    };
}
pub use crate::__aws_deadline_Limit as Limit;
impl crate::template::ToResource for Limit_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Deadline"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Limit"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AmountRequirementName".to_string(),
            crate::value::ToValue::to_value(&self.amount_requirement_name),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DisplayName".to_string(),
            crate::value::ToValue::to_value(&self.display_name),
        );
        properties.insert(
            "FarmId".to_string(),
            crate::value::ToValue::to_value(&self.farm_id),
        );
        properties.insert(
            "MaxCount".to_string(),
            crate::value::ToValue::to_value(&self.max_count),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-deadline-meteredproduct.html
pub struct MeteredProduct_ {
    pub license_endpoint_id: Option<crate::value::ExpString>,
    pub product_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_deadline_MeteredProduct {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Deadline::MeteredProduct"
        $($field $value)*)
    };
}
pub use crate::__aws_deadline_MeteredProduct as MeteredProduct;
impl crate::template::ToResource for MeteredProduct_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Deadline"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MeteredProduct"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.license_endpoint_id {
            properties.insert(
                "LicenseEndpointId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.product_id {
            properties.insert(
                "ProductId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-deadline-monitor.html
pub struct Monitor_ {
    pub display_name: crate::value::ExpString,
    pub identity_center_instance_arn: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub subdomain: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_deadline_Monitor {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Deadline::Monitor"
        $($field $value)*)
    };
}
pub use crate::__aws_deadline_Monitor as Monitor;
impl crate::template::ToResource for Monitor_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Deadline"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Monitor"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DisplayName".to_string(),
            crate::value::ToValue::to_value(&self.display_name),
        );
        properties.insert(
            "IdentityCenterInstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.identity_center_instance_arn),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties.insert(
            "Subdomain".to_string(),
            crate::value::ToValue::to_value(&self.subdomain),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-deadline-queue.html
pub struct Queue_ {
    pub allowed_storage_profile_ids: Option<Vec<crate::value::ExpString>>,
    pub default_budget_action: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub display_name: crate::value::ExpString,
    pub farm_id: crate::value::ExpString,
    pub job_attachment_settings: Option<super::deadline::queue::JobAttachmentSettings_>,
    pub job_run_as_user: Option<super::deadline::queue::JobRunAsUser_>,
    pub required_file_system_location_names: Option<Vec<crate::value::ExpString>>,
    pub role_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_deadline_Queue {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Deadline::Queue" $($field
        $value)*)
    };
}
pub use crate::__aws_deadline_Queue as Queue;
impl crate::template::ToResource for Queue_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Deadline"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Queue"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allowed_storage_profile_ids {
            properties.insert(
                "AllowedStorageProfileIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_budget_action {
            properties.insert(
                "DefaultBudgetAction".to_string(),
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
            "DisplayName".to_string(),
            crate::value::ToValue::to_value(&self.display_name),
        );
        properties.insert(
            "FarmId".to_string(),
            crate::value::ToValue::to_value(&self.farm_id),
        );
        if let Some(ref value) = self.job_attachment_settings {
            properties.insert(
                "JobAttachmentSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_run_as_user {
            properties.insert(
                "JobRunAsUser".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.required_file_system_location_names {
            properties.insert(
                "RequiredFileSystemLocationNames".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-deadline-queueenvironment.html
pub struct QueueEnvironment_ {
    pub farm_id: crate::value::ExpString,
    pub priority: i64,
    pub queue_id: crate::value::ExpString,
    pub template: crate::value::ExpString,
    pub template_type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_deadline_QueueEnvironment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Deadline::QueueEnvironment"
        $($field $value)*)
    };
}
pub use crate::__aws_deadline_QueueEnvironment as QueueEnvironment;
impl crate::template::ToResource for QueueEnvironment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Deadline"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("QueueEnvironment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "FarmId".to_string(),
            crate::value::ToValue::to_value(&self.farm_id),
        );
        properties.insert(
            "Priority".to_string(),
            crate::value::ToValue::to_value(&self.priority),
        );
        properties.insert(
            "QueueId".to_string(),
            crate::value::ToValue::to_value(&self.queue_id),
        );
        properties.insert(
            "Template".to_string(),
            crate::value::ToValue::to_value(&self.template),
        );
        properties.insert(
            "TemplateType".to_string(),
            crate::value::ToValue::to_value(&self.template_type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-deadline-queuefleetassociation.html
pub struct QueueFleetAssociation_ {
    pub farm_id: crate::value::ExpString,
    pub fleet_id: crate::value::ExpString,
    pub queue_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_deadline_QueueFleetAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Deadline::QueueFleetAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_deadline_QueueFleetAssociation as QueueFleetAssociation;
impl crate::template::ToResource for QueueFleetAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Deadline"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("QueueFleetAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "FarmId".to_string(),
            crate::value::ToValue::to_value(&self.farm_id),
        );
        properties.insert(
            "FleetId".to_string(),
            crate::value::ToValue::to_value(&self.fleet_id),
        );
        properties.insert(
            "QueueId".to_string(),
            crate::value::ToValue::to_value(&self.queue_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-deadline-queuelimitassociation.html
pub struct QueueLimitAssociation_ {
    pub farm_id: crate::value::ExpString,
    pub limit_id: crate::value::ExpString,
    pub queue_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_deadline_QueueLimitAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Deadline::QueueLimitAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_deadline_QueueLimitAssociation as QueueLimitAssociation;
impl crate::template::ToResource for QueueLimitAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Deadline"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("QueueLimitAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "FarmId".to_string(),
            crate::value::ToValue::to_value(&self.farm_id),
        );
        properties.insert(
            "LimitId".to_string(),
            crate::value::ToValue::to_value(&self.limit_id),
        );
        properties.insert(
            "QueueId".to_string(),
            crate::value::ToValue::to_value(&self.queue_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-deadline-storageprofile.html
pub struct StorageProfile_ {
    pub display_name: crate::value::ExpString,
    pub farm_id: crate::value::ExpString,
    pub file_system_locations: Option<Vec<super::deadline::storageprofile::FileSystemLocation_>>,
    pub os_family: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_deadline_StorageProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Deadline::StorageProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_deadline_StorageProfile as StorageProfile;
impl crate::template::ToResource for StorageProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Deadline"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StorageProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DisplayName".to_string(),
            crate::value::ToValue::to_value(&self.display_name),
        );
        properties.insert(
            "FarmId".to_string(),
            crate::value::ToValue::to_value(&self.farm_id),
        );
        if let Some(ref value) = self.file_system_locations {
            properties.insert(
                "FileSystemLocations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "OsFamily".to_string(),
            crate::value::ToValue::to_value(&self.os_family),
        );
        properties
    }
}
