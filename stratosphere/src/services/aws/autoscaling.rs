pub mod autoscalinggroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-acceleratorcountrequest.html
    pub struct AcceleratorCountRequest_ {
        pub max: Option<i64>,
        pub min: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_AcceleratorCountRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.AcceleratorCountRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_AcceleratorCountRequest as AcceleratorCountRequest;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-acceleratortotalmemorymibrequest.html
    pub struct AcceleratorTotalMemoryMiBRequest_ {
        pub max: Option<i64>,
        pub min: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_AcceleratorTotalMemoryMiBRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.AcceleratorTotalMemoryMiBRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_AcceleratorTotalMemoryMiBRequest as AcceleratorTotalMemoryMiBRequest;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-availabilityzonedistribution.html
    pub struct AvailabilityZoneDistribution_ {
        pub capacity_distribution_strategy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_AvailabilityZoneDistribution {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.AvailabilityZoneDistribution"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_AvailabilityZoneDistribution as AvailabilityZoneDistribution;
    impl crate::value::ToValue for AvailabilityZoneDistribution_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity_distribution_strategy {
                properties.insert(
                    "CapacityDistributionStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-availabilityzoneimpairmentpolicy.html
    pub struct AvailabilityZoneImpairmentPolicy_ {
        pub impaired_zone_health_check_behavior: crate::value::ExpString,
        pub zonal_shift_enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_AvailabilityZoneImpairmentPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.AvailabilityZoneImpairmentPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_AvailabilityZoneImpairmentPolicy as AvailabilityZoneImpairmentPolicy;
    impl crate::value::ToValue for AvailabilityZoneImpairmentPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ImpairedZoneHealthCheckBehavior".to_string(),
                crate::value::ToValue::to_value(&self.impaired_zone_health_check_behavior),
            );
            properties.insert(
                "ZonalShiftEnabled".to_string(),
                crate::value::ToValue::to_value(&self.zonal_shift_enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-baselineebsbandwidthmbpsrequest.html
    pub struct BaselineEbsBandwidthMbpsRequest_ {
        pub max: Option<i64>,
        pub min: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_BaselineEbsBandwidthMbpsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.BaselineEbsBandwidthMbpsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_BaselineEbsBandwidthMbpsRequest as BaselineEbsBandwidthMbpsRequest;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-baselineperformancefactorsrequest.html
    pub struct BaselinePerformanceFactorsRequest_ {
        pub cpu: Option<Box<CpuPerformanceFactorRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_BaselinePerformanceFactorsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.BaselinePerformanceFactorsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_BaselinePerformanceFactorsRequest as BaselinePerformanceFactorsRequest;
    impl crate::value::ToValue for BaselinePerformanceFactorsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cpu {
                properties.insert("Cpu".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-capacityreservationspecification.html
    pub struct CapacityReservationSpecification_ {
        pub capacity_reservation_preference: crate::value::ExpString,
        pub capacity_reservation_target: Option<Box<CapacityReservationTarget_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_CapacityReservationSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.CapacityReservationSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_CapacityReservationSpecification as CapacityReservationSpecification;
    impl crate::value::ToValue for CapacityReservationSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CapacityReservationPreference".to_string(),
                crate::value::ToValue::to_value(&self.capacity_reservation_preference),
            );
            if let Some(ref value) = self.capacity_reservation_target {
                properties.insert(
                    "CapacityReservationTarget".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-capacityreservationtarget.html
    pub struct CapacityReservationTarget_ {
        pub capacity_reservation_ids: Option<Vec<crate::value::ExpString>>,
        pub capacity_reservation_resource_group_arns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_CapacityReservationTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.CapacityReservationTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_CapacityReservationTarget as CapacityReservationTarget;
    impl crate::value::ToValue for CapacityReservationTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity_reservation_ids {
                properties.insert(
                    "CapacityReservationIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.capacity_reservation_resource_group_arns {
                properties.insert(
                    "CapacityReservationResourceGroupArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-cpuperformancefactorrequest.html
    pub struct CpuPerformanceFactorRequest_ {
        pub references: Option<Vec<PerformanceFactorReferenceRequest_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_CpuPerformanceFactorRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.CpuPerformanceFactorRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_CpuPerformanceFactorRequest as CpuPerformanceFactorRequest;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-instancemaintenancepolicy.html
    pub struct InstanceMaintenancePolicy_ {
        pub max_healthy_percentage: Option<i64>,
        pub min_healthy_percentage: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_InstanceMaintenancePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.InstanceMaintenancePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_InstanceMaintenancePolicy as InstanceMaintenancePolicy;
    impl crate::value::ToValue for InstanceMaintenancePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_healthy_percentage {
                properties.insert(
                    "MaxHealthyPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_healthy_percentage {
                properties.insert(
                    "MinHealthyPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-instancerequirements.html
    pub struct InstanceRequirements_ {
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
        pub max_spot_price_as_percentage_of_optimal_on_demand_price: Option<i64>,
        pub memory_gi_b_per_v_cpu: Option<Box<MemoryGiBPerVCpuRequest_>>,
        pub memory_mi_b: Box<MemoryMiBRequest_>,
        pub network_bandwidth_gbps: Option<Box<NetworkBandwidthGbpsRequest_>>,
        pub network_interface_count: Option<Box<NetworkInterfaceCountRequest_>>,
        pub on_demand_max_price_percentage_over_lowest_price: Option<i64>,
        pub require_hibernate_support: Option<crate::value::ExpBool>,
        pub spot_max_price_percentage_over_lowest_price: Option<i64>,
        pub total_local_storage_gb: Option<Box<TotalLocalStorageGBRequest_>>,
        pub v_cpu_count: Box<VCpuCountRequest_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_InstanceRequirements {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.InstanceRequirements"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_InstanceRequirements as InstanceRequirements;
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
            properties.insert(
                "MemoryMiB".to_string(),
                crate::value::ToValue::to_value(&self.memory_mi_b),
            );
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
            properties.insert(
                "VCpuCount".to_string(),
                crate::value::ToValue::to_value(&self.v_cpu_count),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-instancesdistribution.html
    pub struct InstancesDistribution_ {
        pub on_demand_allocation_strategy: Option<crate::value::ExpString>,
        pub on_demand_base_capacity: Option<i64>,
        pub on_demand_percentage_above_base_capacity: Option<i64>,
        pub spot_allocation_strategy: Option<crate::value::ExpString>,
        pub spot_instance_pools: Option<i64>,
        pub spot_max_price: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_InstancesDistribution {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.InstancesDistribution"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_InstancesDistribution as InstancesDistribution;
    impl crate::value::ToValue for InstancesDistribution_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.on_demand_allocation_strategy {
                properties.insert(
                    "OnDemandAllocationStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_demand_base_capacity {
                properties.insert(
                    "OnDemandBaseCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_demand_percentage_above_base_capacity {
                properties.insert(
                    "OnDemandPercentageAboveBaseCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_allocation_strategy {
                properties.insert(
                    "SpotAllocationStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_instance_pools {
                properties.insert(
                    "SpotInstancePools".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_max_price {
                properties.insert(
                    "SpotMaxPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-launchtemplate.html
    pub struct LaunchTemplate_ {
        pub launch_template_specification: Box<LaunchTemplateSpecification_>,
        pub overrides: Option<Vec<LaunchTemplateOverrides_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_LaunchTemplate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.LaunchTemplate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_LaunchTemplate as LaunchTemplate;
    impl crate::value::ToValue for LaunchTemplate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LaunchTemplateSpecification".to_string(),
                crate::value::ToValue::to_value(&self.launch_template_specification),
            );
            if let Some(ref value) = self.overrides {
                properties.insert(
                    "Overrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-launchtemplateoverrides.html
    pub struct LaunchTemplateOverrides_ {
        pub instance_requirements: Option<Box<InstanceRequirements_>>,
        pub instance_type: Option<crate::value::ExpString>,
        pub launch_template_specification: Option<Box<LaunchTemplateSpecification_>>,
        pub weighted_capacity: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_LaunchTemplateOverrides {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.LaunchTemplateOverrides"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_LaunchTemplateOverrides as LaunchTemplateOverrides;
    impl crate::value::ToValue for LaunchTemplateOverrides_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
            if let Some(ref value) = self.launch_template_specification {
                properties.insert(
                    "LaunchTemplateSpecification".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-launchtemplatespecification.html
    pub struct LaunchTemplateSpecification_ {
        pub launch_template_id: Option<crate::value::ExpString>,
        pub launch_template_name: Option<crate::value::ExpString>,
        pub version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_LaunchTemplateSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.LaunchTemplateSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_LaunchTemplateSpecification as LaunchTemplateSpecification;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-lifecyclehookspecification.html
    pub struct LifecycleHookSpecification_ {
        pub default_result: Option<crate::value::ExpString>,
        pub heartbeat_timeout: Option<i64>,
        pub lifecycle_hook_name: crate::value::ExpString,
        pub lifecycle_transition: crate::value::ExpString,
        pub notification_metadata: Option<crate::value::ExpString>,
        pub notification_target_arn: Option<crate::value::ExpString>,
        pub role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_LifecycleHookSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.LifecycleHookSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_LifecycleHookSpecification as LifecycleHookSpecification;
    impl crate::value::ToValue for LifecycleHookSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_result {
                properties.insert(
                    "DefaultResult".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.heartbeat_timeout {
                properties.insert(
                    "HeartbeatTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LifecycleHookName".to_string(),
                crate::value::ToValue::to_value(&self.lifecycle_hook_name),
            );
            properties.insert(
                "LifecycleTransition".to_string(),
                crate::value::ToValue::to_value(&self.lifecycle_transition),
            );
            if let Some(ref value) = self.notification_metadata {
                properties.insert(
                    "NotificationMetadata".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.notification_target_arn {
                properties.insert(
                    "NotificationTargetARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-memorygibpervcpurequest.html
    pub struct MemoryGiBPerVCpuRequest_ {
        pub max: Option<f64>,
        pub min: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_MemoryGiBPerVCpuRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.MemoryGiBPerVCpuRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_MemoryGiBPerVCpuRequest as MemoryGiBPerVCpuRequest;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-memorymibrequest.html
    pub struct MemoryMiBRequest_ {
        pub max: Option<i64>,
        pub min: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_MemoryMiBRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.MemoryMiBRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_MemoryMiBRequest as MemoryMiBRequest;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-metricscollection.html
    pub struct MetricsCollection_ {
        pub granularity: crate::value::ExpString,
        pub metrics: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_MetricsCollection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.MetricsCollection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_MetricsCollection as MetricsCollection;
    impl crate::value::ToValue for MetricsCollection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Granularity".to_string(),
                crate::value::ToValue::to_value(&self.granularity),
            );
            if let Some(ref value) = self.metrics {
                properties.insert(
                    "Metrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-mixedinstancespolicy.html
    pub struct MixedInstancesPolicy_ {
        pub instances_distribution: Option<Box<InstancesDistribution_>>,
        pub launch_template: Box<LaunchTemplate_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_MixedInstancesPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.MixedInstancesPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_MixedInstancesPolicy as MixedInstancesPolicy;
    impl crate::value::ToValue for MixedInstancesPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instances_distribution {
                properties.insert(
                    "InstancesDistribution".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LaunchTemplate".to_string(),
                crate::value::ToValue::to_value(&self.launch_template),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-networkbandwidthgbpsrequest.html
    pub struct NetworkBandwidthGbpsRequest_ {
        pub max: Option<f64>,
        pub min: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_NetworkBandwidthGbpsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.NetworkBandwidthGbpsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_NetworkBandwidthGbpsRequest as NetworkBandwidthGbpsRequest;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-networkinterfacecountrequest.html
    pub struct NetworkInterfaceCountRequest_ {
        pub max: Option<i64>,
        pub min: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_NetworkInterfaceCountRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.NetworkInterfaceCountRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_NetworkInterfaceCountRequest as NetworkInterfaceCountRequest;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-notificationconfiguration.html
    pub struct NotificationConfiguration_ {
        pub notification_types: Option<Vec<crate::value::ExpString>>,
        pub topic_arn: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_NotificationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.NotificationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_NotificationConfiguration as NotificationConfiguration;
    impl crate::value::ToValue for NotificationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.notification_types {
                properties.insert(
                    "NotificationTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TopicARN".to_string(),
                crate::value::ToValue::to_value(&self.topic_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-performancefactorreferencerequest.html
    pub struct PerformanceFactorReferenceRequest_ {
        pub instance_family: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_PerformanceFactorReferenceRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.PerformanceFactorReferenceRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_PerformanceFactorReferenceRequest as PerformanceFactorReferenceRequest;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-tagproperty.html
    pub struct TagProperty_ {
        pub key: crate::value::ExpString,
        pub propagate_at_launch: crate::value::ExpBool,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_TagProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.TagProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_TagProperty as TagProperty;
    impl crate::value::ToValue for TagProperty_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "PropagateAtLaunch".to_string(),
                crate::value::ToValue::to_value(&self.propagate_at_launch),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-totallocalstoragegbrequest.html
    pub struct TotalLocalStorageGBRequest_ {
        pub max: Option<f64>,
        pub min: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_TotalLocalStorageGBRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.TotalLocalStorageGBRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_TotalLocalStorageGBRequest as TotalLocalStorageGBRequest;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-trafficsourceidentifier.html
    pub struct TrafficSourceIdentifier_ {
        pub identifier: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_TrafficSourceIdentifier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.TrafficSourceIdentifier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_TrafficSourceIdentifier as TrafficSourceIdentifier;
    impl crate::value::ToValue for TrafficSourceIdentifier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Identifier".to_string(),
                crate::value::ToValue::to_value(&self.identifier),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-autoscalinggroup-vcpucountrequest.html
    pub struct VCpuCountRequest_ {
        pub max: Option<i64>,
        pub min: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_AutoScalingGroup_VCpuCountRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::AutoScalingGroup.VCpuCountRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_AutoScalingGroup_VCpuCountRequest as VCpuCountRequest;
    impl crate::value::ToValue for VCpuCountRequest_ {
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
pub mod launchconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-launchconfiguration-blockdevice.html
    pub struct BlockDevice_ {
        pub delete_on_termination: Option<crate::value::ExpBool>,
        pub encrypted: Option<crate::value::ExpBool>,
        pub iops: Option<i64>,
        pub snapshot_id: Option<crate::value::ExpString>,
        pub throughput: Option<i64>,
        pub volume_size: Option<i64>,
        pub volume_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_LaunchConfiguration_BlockDevice {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::LaunchConfiguration.BlockDevice"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_LaunchConfiguration_BlockDevice as BlockDevice;
    impl crate::value::ToValue for BlockDevice_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-launchconfiguration-blockdevicemapping.html
    pub struct BlockDeviceMapping_ {
        pub device_name: crate::value::ExpString,
        pub ebs: Option<Box<BlockDevice_>>,
        pub no_device: Option<crate::value::ExpBool>,
        pub virtual_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_LaunchConfiguration_BlockDeviceMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::LaunchConfiguration.BlockDeviceMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_LaunchConfiguration_BlockDeviceMapping as BlockDeviceMapping;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-launchconfiguration-metadataoptions.html
    pub struct MetadataOptions_ {
        pub http_endpoint: Option<crate::value::ExpString>,
        pub http_put_response_hop_limit: Option<i64>,
        pub http_tokens: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_LaunchConfiguration_MetadataOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::LaunchConfiguration.MetadataOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_LaunchConfiguration_MetadataOptions as MetadataOptions;
    impl crate::value::ToValue for MetadataOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.http_endpoint {
                properties.insert(
                    "HttpEndpoint".to_string(),
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
            properties.into()
        }
    }
}
pub mod scalingpolicy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-customizedmetricspecification.html
    pub struct CustomizedMetricSpecification_ {
        pub dimensions: Option<Vec<MetricDimension_>>,
        pub metric_name: Option<crate::value::ExpString>,
        pub metrics: Option<Vec<TargetTrackingMetricDataQuery_>>,
        pub namespace: Option<crate::value::ExpString>,
        pub period: Option<i64>,
        pub statistic: Option<crate::value::ExpString>,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_CustomizedMetricSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.CustomizedMetricSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_CustomizedMetricSpecification as CustomizedMetricSpecification;
    impl crate::value::ToValue for CustomizedMetricSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimensions {
                properties.insert(
                    "Dimensions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_name {
                properties.insert(
                    "MetricName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metrics {
                properties.insert(
                    "Metrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.namespace {
                properties.insert(
                    "Namespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.period {
                properties.insert("Period".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.statistic {
                properties.insert(
                    "Statistic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-metric.html
    pub struct Metric_ {
        pub dimensions: Option<Vec<MetricDimension_>>,
        pub metric_name: crate::value::ExpString,
        pub namespace: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_Metric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.Metric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_Metric as Metric;
    impl crate::value::ToValue for Metric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimensions {
                properties.insert(
                    "Dimensions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MetricName".to_string(),
                crate::value::ToValue::to_value(&self.metric_name),
            );
            properties.insert(
                "Namespace".to_string(),
                crate::value::ToValue::to_value(&self.namespace),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-metricdataquery.html
    pub struct MetricDataQuery_ {
        pub expression: Option<crate::value::ExpString>,
        pub id: crate::value::ExpString,
        pub label: Option<crate::value::ExpString>,
        pub metric_stat: Option<Box<MetricStat_>>,
        pub return_data: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_MetricDataQuery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.MetricDataQuery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_MetricDataQuery as MetricDataQuery;
    impl crate::value::ToValue for MetricDataQuery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.expression {
                properties.insert(
                    "Expression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.label {
                properties.insert("Label".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.metric_stat {
                properties.insert(
                    "MetricStat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.return_data {
                properties.insert(
                    "ReturnData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-metricdimension.html
    pub struct MetricDimension_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_MetricDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.MetricDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_MetricDimension as MetricDimension;
    impl crate::value::ToValue for MetricDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-metricstat.html
    pub struct MetricStat_ {
        pub metric: Box<Metric_>,
        pub stat: crate::value::ExpString,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_MetricStat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.MetricStat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_MetricStat as MetricStat;
    impl crate::value::ToValue for MetricStat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Metric".to_string(),
                crate::value::ToValue::to_value(&self.metric),
            );
            properties.insert(
                "Stat".to_string(),
                crate::value::ToValue::to_value(&self.stat),
            );
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-predefinedmetricspecification.html
    pub struct PredefinedMetricSpecification_ {
        pub predefined_metric_type: crate::value::ExpString,
        pub resource_label: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_PredefinedMetricSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.PredefinedMetricSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_PredefinedMetricSpecification as PredefinedMetricSpecification;
    impl crate::value::ToValue for PredefinedMetricSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PredefinedMetricType".to_string(),
                crate::value::ToValue::to_value(&self.predefined_metric_type),
            );
            if let Some(ref value) = self.resource_label {
                properties.insert(
                    "ResourceLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-predictivescalingconfiguration.html
    pub struct PredictiveScalingConfiguration_ {
        pub max_capacity_breach_behavior: Option<crate::value::ExpString>,
        pub max_capacity_buffer: Option<i64>,
        pub metric_specifications: Vec<PredictiveScalingMetricSpecification_>,
        pub mode: Option<crate::value::ExpString>,
        pub scheduling_buffer_time: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_PredictiveScalingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.PredictiveScalingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_PredictiveScalingConfiguration as PredictiveScalingConfiguration;
    impl crate::value::ToValue for PredictiveScalingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_capacity_breach_behavior {
                properties.insert(
                    "MaxCapacityBreachBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_capacity_buffer {
                properties.insert(
                    "MaxCapacityBuffer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MetricSpecifications".to_string(),
                crate::value::ToValue::to_value(&self.metric_specifications),
            );
            if let Some(ref value) = self.mode {
                properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.scheduling_buffer_time {
                properties.insert(
                    "SchedulingBufferTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-predictivescalingcustomizedcapacitymetric.html
    pub struct PredictiveScalingCustomizedCapacityMetric_ {
        pub metric_data_queries: Vec<MetricDataQuery_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_PredictiveScalingCustomizedCapacityMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.PredictiveScalingCustomizedCapacityMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_PredictiveScalingCustomizedCapacityMetric as PredictiveScalingCustomizedCapacityMetric;
    impl crate::value::ToValue for PredictiveScalingCustomizedCapacityMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MetricDataQueries".to_string(),
                crate::value::ToValue::to_value(&self.metric_data_queries),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-predictivescalingcustomizedloadmetric.html
    pub struct PredictiveScalingCustomizedLoadMetric_ {
        pub metric_data_queries: Vec<MetricDataQuery_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_PredictiveScalingCustomizedLoadMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.PredictiveScalingCustomizedLoadMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_PredictiveScalingCustomizedLoadMetric as PredictiveScalingCustomizedLoadMetric;
    impl crate::value::ToValue for PredictiveScalingCustomizedLoadMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MetricDataQueries".to_string(),
                crate::value::ToValue::to_value(&self.metric_data_queries),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-predictivescalingcustomizedscalingmetric.html
    pub struct PredictiveScalingCustomizedScalingMetric_ {
        pub metric_data_queries: Vec<MetricDataQuery_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_PredictiveScalingCustomizedScalingMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.PredictiveScalingCustomizedScalingMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_PredictiveScalingCustomizedScalingMetric as PredictiveScalingCustomizedScalingMetric;
    impl crate::value::ToValue for PredictiveScalingCustomizedScalingMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MetricDataQueries".to_string(),
                crate::value::ToValue::to_value(&self.metric_data_queries),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-predictivescalingmetricspecification.html
    pub struct PredictiveScalingMetricSpecification_ {
        pub customized_capacity_metric_specification:
            Option<Box<PredictiveScalingCustomizedCapacityMetric_>>,
        pub customized_load_metric_specification:
            Option<Box<PredictiveScalingCustomizedLoadMetric_>>,
        pub customized_scaling_metric_specification:
            Option<Box<PredictiveScalingCustomizedScalingMetric_>>,
        pub predefined_load_metric_specification:
            Option<Box<PredictiveScalingPredefinedLoadMetric_>>,
        pub predefined_metric_pair_specification:
            Option<Box<PredictiveScalingPredefinedMetricPair_>>,
        pub predefined_scaling_metric_specification:
            Option<Box<PredictiveScalingPredefinedScalingMetric_>>,
        pub target_value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_PredictiveScalingMetricSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.PredictiveScalingMetricSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_PredictiveScalingMetricSpecification as PredictiveScalingMetricSpecification;
    impl crate::value::ToValue for PredictiveScalingMetricSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customized_capacity_metric_specification {
                properties.insert(
                    "CustomizedCapacityMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.customized_load_metric_specification {
                properties.insert(
                    "CustomizedLoadMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.customized_scaling_metric_specification {
                properties.insert(
                    "CustomizedScalingMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.predefined_load_metric_specification {
                properties.insert(
                    "PredefinedLoadMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.predefined_metric_pair_specification {
                properties.insert(
                    "PredefinedMetricPairSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.predefined_scaling_metric_specification {
                properties.insert(
                    "PredefinedScalingMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetValue".to_string(),
                crate::value::ToValue::to_value(&self.target_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-predictivescalingpredefinedloadmetric.html
    pub struct PredictiveScalingPredefinedLoadMetric_ {
        pub predefined_metric_type: crate::value::ExpString,
        pub resource_label: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_PredictiveScalingPredefinedLoadMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.PredictiveScalingPredefinedLoadMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_PredictiveScalingPredefinedLoadMetric as PredictiveScalingPredefinedLoadMetric;
    impl crate::value::ToValue for PredictiveScalingPredefinedLoadMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PredefinedMetricType".to_string(),
                crate::value::ToValue::to_value(&self.predefined_metric_type),
            );
            if let Some(ref value) = self.resource_label {
                properties.insert(
                    "ResourceLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-predictivescalingpredefinedmetricpair.html
    pub struct PredictiveScalingPredefinedMetricPair_ {
        pub predefined_metric_type: crate::value::ExpString,
        pub resource_label: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_PredictiveScalingPredefinedMetricPair {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.PredictiveScalingPredefinedMetricPair"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_PredictiveScalingPredefinedMetricPair as PredictiveScalingPredefinedMetricPair;
    impl crate::value::ToValue for PredictiveScalingPredefinedMetricPair_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PredefinedMetricType".to_string(),
                crate::value::ToValue::to_value(&self.predefined_metric_type),
            );
            if let Some(ref value) = self.resource_label {
                properties.insert(
                    "ResourceLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-predictivescalingpredefinedscalingmetric.html
    pub struct PredictiveScalingPredefinedScalingMetric_ {
        pub predefined_metric_type: crate::value::ExpString,
        pub resource_label: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_PredictiveScalingPredefinedScalingMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.PredictiveScalingPredefinedScalingMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_PredictiveScalingPredefinedScalingMetric as PredictiveScalingPredefinedScalingMetric;
    impl crate::value::ToValue for PredictiveScalingPredefinedScalingMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PredefinedMetricType".to_string(),
                crate::value::ToValue::to_value(&self.predefined_metric_type),
            );
            if let Some(ref value) = self.resource_label {
                properties.insert(
                    "ResourceLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-stepadjustment.html
    pub struct StepAdjustment_ {
        pub metric_interval_lower_bound: Option<f64>,
        pub metric_interval_upper_bound: Option<f64>,
        pub scaling_adjustment: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_StepAdjustment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.StepAdjustment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_StepAdjustment as StepAdjustment;
    impl crate::value::ToValue for StepAdjustment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.metric_interval_lower_bound {
                properties.insert(
                    "MetricIntervalLowerBound".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_interval_upper_bound {
                properties.insert(
                    "MetricIntervalUpperBound".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ScalingAdjustment".to_string(),
                crate::value::ToValue::to_value(&self.scaling_adjustment),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-targettrackingconfiguration.html
    pub struct TargetTrackingConfiguration_ {
        pub customized_metric_specification: Option<Box<CustomizedMetricSpecification_>>,
        pub disable_scale_in: Option<crate::value::ExpBool>,
        pub predefined_metric_specification: Option<Box<PredefinedMetricSpecification_>>,
        pub target_value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_TargetTrackingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.TargetTrackingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_TargetTrackingConfiguration as TargetTrackingConfiguration;
    impl crate::value::ToValue for TargetTrackingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customized_metric_specification {
                properties.insert(
                    "CustomizedMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.disable_scale_in {
                properties.insert(
                    "DisableScaleIn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.predefined_metric_specification {
                properties.insert(
                    "PredefinedMetricSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetValue".to_string(),
                crate::value::ToValue::to_value(&self.target_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-targettrackingmetricdataquery.html
    pub struct TargetTrackingMetricDataQuery_ {
        pub expression: Option<crate::value::ExpString>,
        pub id: crate::value::ExpString,
        pub label: Option<crate::value::ExpString>,
        pub metric_stat: Option<Box<TargetTrackingMetricStat_>>,
        pub period: Option<i64>,
        pub return_data: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_TargetTrackingMetricDataQuery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.TargetTrackingMetricDataQuery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_TargetTrackingMetricDataQuery as TargetTrackingMetricDataQuery;
    impl crate::value::ToValue for TargetTrackingMetricDataQuery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.expression {
                properties.insert(
                    "Expression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.label {
                properties.insert("Label".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.metric_stat {
                properties.insert(
                    "MetricStat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.period {
                properties.insert("Period".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.return_data {
                properties.insert(
                    "ReturnData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-scalingpolicy-targettrackingmetricstat.html
    pub struct TargetTrackingMetricStat_ {
        pub metric: Box<Metric_>,
        pub period: Option<i64>,
        pub stat: crate::value::ExpString,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_ScalingPolicy_TargetTrackingMetricStat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::ScalingPolicy.TargetTrackingMetricStat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_ScalingPolicy_TargetTrackingMetricStat as TargetTrackingMetricStat;
    impl crate::value::ToValue for TargetTrackingMetricStat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Metric".to_string(),
                crate::value::ToValue::to_value(&self.metric),
            );
            if let Some(ref value) = self.period {
                properties.insert("Period".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Stat".to_string(),
                crate::value::ToValue::to_value(&self.stat),
            );
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod warmpool {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-autoscaling-warmpool-instancereusepolicy.html
    pub struct InstanceReusePolicy_ {
        pub reuse_on_scale_in: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_autoscaling_WarmPool_InstanceReusePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AutoScaling::WarmPool.InstanceReusePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_autoscaling_WarmPool_InstanceReusePolicy as InstanceReusePolicy;
    impl crate::value::ToValue for InstanceReusePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.reuse_on_scale_in {
                properties.insert(
                    "ReuseOnScaleIn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-autoscaling-autoscalinggroup.html
pub struct AutoScalingGroup_ {
    pub auto_scaling_group_name: Option<crate::value::ExpString>,
    pub availability_zone_distribution:
        Option<super::autoscaling::autoscalinggroup::AvailabilityZoneDistribution_>,
    pub availability_zone_impairment_policy:
        Option<super::autoscaling::autoscalinggroup::AvailabilityZoneImpairmentPolicy_>,
    pub availability_zones: Option<Vec<crate::value::ExpString>>,
    pub capacity_rebalance: Option<crate::value::ExpBool>,
    pub capacity_reservation_specification:
        Option<super::autoscaling::autoscalinggroup::CapacityReservationSpecification_>,
    pub context: Option<crate::value::ExpString>,
    pub cooldown: Option<crate::value::ExpString>,
    pub default_instance_warmup: Option<i64>,
    pub desired_capacity: Option<crate::value::ExpString>,
    pub desired_capacity_type: Option<crate::value::ExpString>,
    pub health_check_grace_period: Option<i64>,
    pub health_check_type: Option<crate::value::ExpString>,
    pub instance_id: Option<crate::value::ExpString>,
    pub instance_maintenance_policy:
        Option<super::autoscaling::autoscalinggroup::InstanceMaintenancePolicy_>,
    pub launch_configuration_name: Option<crate::value::ExpString>,
    pub launch_template: Option<super::autoscaling::autoscalinggroup::LaunchTemplateSpecification_>,
    pub lifecycle_hook_specification_list:
        Option<Vec<super::autoscaling::autoscalinggroup::LifecycleHookSpecification_>>,
    pub load_balancer_names: Option<Vec<crate::value::ExpString>>,
    pub max_instance_lifetime: Option<i64>,
    pub max_size: crate::value::ExpString,
    pub metrics_collection: Option<Vec<super::autoscaling::autoscalinggroup::MetricsCollection_>>,
    pub min_size: crate::value::ExpString,
    pub mixed_instances_policy: Option<super::autoscaling::autoscalinggroup::MixedInstancesPolicy_>,
    pub new_instances_protected_from_scale_in: Option<crate::value::ExpBool>,
    pub notification_configurations:
        Option<Vec<super::autoscaling::autoscalinggroup::NotificationConfiguration_>>,
    pub placement_group: Option<crate::value::ExpString>,
    pub service_linked_role_arn: Option<crate::value::ExpString>,
    pub skip_zonal_shift_validation: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<super::autoscaling::autoscalinggroup::TagProperty_>>,
    pub target_group_ar_ns: Option<Vec<crate::value::ExpString>>,
    pub termination_policies: Option<Vec<crate::value::ExpString>>,
    pub traffic_sources:
        Option<Vec<super::autoscaling::autoscalinggroup::TrafficSourceIdentifier_>>,
    pub vpc_zone_identifier: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_autoscaling_AutoScalingGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AutoScaling::AutoScalingGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_autoscaling_AutoScalingGroup as AutoScalingGroup;
impl crate::template::ToResource for AutoScalingGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AutoScaling"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AutoScalingGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_scaling_group_name {
            properties.insert(
                "AutoScalingGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zone_distribution {
            properties.insert(
                "AvailabilityZoneDistribution".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zone_impairment_policy {
            properties.insert(
                "AvailabilityZoneImpairmentPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zones {
            properties.insert(
                "AvailabilityZones".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.capacity_rebalance {
            properties.insert(
                "CapacityRebalance".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.capacity_reservation_specification {
            properties.insert(
                "CapacityReservationSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.context {
            properties.insert(
                "Context".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cooldown {
            properties.insert(
                "Cooldown".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_instance_warmup {
            properties.insert(
                "DefaultInstanceWarmup".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.desired_capacity {
            properties.insert(
                "DesiredCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.desired_capacity_type {
            properties.insert(
                "DesiredCapacityType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_grace_period {
            properties.insert(
                "HealthCheckGracePeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_type {
            properties.insert(
                "HealthCheckType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_id {
            properties.insert(
                "InstanceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_maintenance_policy {
            properties.insert(
                "InstanceMaintenancePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.launch_configuration_name {
            properties.insert(
                "LaunchConfigurationName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.launch_template {
            properties.insert(
                "LaunchTemplate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lifecycle_hook_specification_list {
            properties.insert(
                "LifecycleHookSpecificationList".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.load_balancer_names {
            properties.insert(
                "LoadBalancerNames".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_instance_lifetime {
            properties.insert(
                "MaxInstanceLifetime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MaxSize".to_string(),
            crate::value::ToValue::to_value(&self.max_size),
        );
        if let Some(ref value) = self.metrics_collection {
            properties.insert(
                "MetricsCollection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MinSize".to_string(),
            crate::value::ToValue::to_value(&self.min_size),
        );
        if let Some(ref value) = self.mixed_instances_policy {
            properties.insert(
                "MixedInstancesPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.new_instances_protected_from_scale_in {
            properties.insert(
                "NewInstancesProtectedFromScaleIn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.notification_configurations {
            properties.insert(
                "NotificationConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.placement_group {
            properties.insert(
                "PlacementGroup".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_linked_role_arn {
            properties.insert(
                "ServiceLinkedRoleARN".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.skip_zonal_shift_validation {
            properties.insert(
                "SkipZonalShiftValidation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.target_group_ar_ns {
            properties.insert(
                "TargetGroupARNs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.termination_policies {
            properties.insert(
                "TerminationPolicies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.traffic_sources {
            properties.insert(
                "TrafficSources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_zone_identifier {
            properties.insert(
                "VPCZoneIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-autoscaling-launchconfiguration.html
pub struct LaunchConfiguration_ {
    pub associate_public_ip_address: Option<crate::value::ExpBool>,
    pub block_device_mappings:
        Option<Vec<super::autoscaling::launchconfiguration::BlockDeviceMapping_>>,
    pub classic_link_vpc_id: Option<crate::value::ExpString>,
    pub classic_link_vpc_security_groups: Option<Vec<crate::value::ExpString>>,
    pub ebs_optimized: Option<crate::value::ExpBool>,
    pub iam_instance_profile: Option<crate::value::ExpString>,
    pub image_id: crate::value::ExpString,
    pub instance_id: Option<crate::value::ExpString>,
    pub instance_monitoring: Option<crate::value::ExpBool>,
    pub instance_type: crate::value::ExpString,
    pub kernel_id: Option<crate::value::ExpString>,
    pub key_name: Option<crate::value::ExpString>,
    pub launch_configuration_name: Option<crate::value::ExpString>,
    pub metadata_options: Option<super::autoscaling::launchconfiguration::MetadataOptions_>,
    pub placement_tenancy: Option<crate::value::ExpString>,
    pub ram_disk_id: Option<crate::value::ExpString>,
    pub security_groups: Option<Vec<crate::value::ExpString>>,
    pub spot_price: Option<crate::value::ExpString>,
    pub user_data: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_autoscaling_LaunchConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AutoScaling::LaunchConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_autoscaling_LaunchConfiguration as LaunchConfiguration;
impl crate::template::ToResource for LaunchConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AutoScaling"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LaunchConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.associate_public_ip_address {
            properties.insert(
                "AssociatePublicIpAddress".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.block_device_mappings {
            properties.insert(
                "BlockDeviceMappings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.classic_link_vpc_id {
            properties.insert(
                "ClassicLinkVPCId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.classic_link_vpc_security_groups {
            properties.insert(
                "ClassicLinkVPCSecurityGroups".to_string(),
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
        if let Some(ref value) = self.instance_id {
            properties.insert(
                "InstanceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_monitoring {
            properties.insert(
                "InstanceMonitoring".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceType".to_string(),
            crate::value::ToValue::to_value(&self.instance_type),
        );
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
        if let Some(ref value) = self.launch_configuration_name {
            properties.insert(
                "LaunchConfigurationName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metadata_options {
            properties.insert(
                "MetadataOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.placement_tenancy {
            properties.insert(
                "PlacementTenancy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ram_disk_id {
            properties.insert(
                "RamDiskId".to_string(),
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
        if let Some(ref value) = self.user_data {
            properties.insert(
                "UserData".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-autoscaling-lifecyclehook.html
pub struct LifecycleHook_ {
    pub auto_scaling_group_name: crate::value::ExpString,
    pub default_result: Option<crate::value::ExpString>,
    pub heartbeat_timeout: Option<i64>,
    pub lifecycle_hook_name: Option<crate::value::ExpString>,
    pub lifecycle_transition: crate::value::ExpString,
    pub notification_metadata: Option<crate::value::ExpString>,
    pub notification_target_arn: Option<crate::value::ExpString>,
    pub role_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_autoscaling_LifecycleHook {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AutoScaling::LifecycleHook"
        $($field $value)*)
    };
}
pub use crate::__aws_autoscaling_LifecycleHook as LifecycleHook;
impl crate::template::ToResource for LifecycleHook_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AutoScaling"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LifecycleHook"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AutoScalingGroupName".to_string(),
            crate::value::ToValue::to_value(&self.auto_scaling_group_name),
        );
        if let Some(ref value) = self.default_result {
            properties.insert(
                "DefaultResult".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.heartbeat_timeout {
            properties.insert(
                "HeartbeatTimeout".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lifecycle_hook_name {
            properties.insert(
                "LifecycleHookName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LifecycleTransition".to_string(),
            crate::value::ToValue::to_value(&self.lifecycle_transition),
        );
        if let Some(ref value) = self.notification_metadata {
            properties.insert(
                "NotificationMetadata".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.notification_target_arn {
            properties.insert(
                "NotificationTargetARN".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-autoscaling-scalingpolicy.html
pub struct ScalingPolicy_ {
    pub adjustment_type: Option<crate::value::ExpString>,
    pub auto_scaling_group_name: crate::value::ExpString,
    pub cooldown: Option<crate::value::ExpString>,
    pub estimated_instance_warmup: Option<i64>,
    pub metric_aggregation_type: Option<crate::value::ExpString>,
    pub min_adjustment_magnitude: Option<i64>,
    pub policy_type: Option<crate::value::ExpString>,
    pub predictive_scaling_configuration:
        Option<super::autoscaling::scalingpolicy::PredictiveScalingConfiguration_>,
    pub scaling_adjustment: Option<i64>,
    pub step_adjustments: Option<Vec<super::autoscaling::scalingpolicy::StepAdjustment_>>,
    pub target_tracking_configuration:
        Option<super::autoscaling::scalingpolicy::TargetTrackingConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_autoscaling_ScalingPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AutoScaling::ScalingPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_autoscaling_ScalingPolicy as ScalingPolicy;
impl crate::template::ToResource for ScalingPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AutoScaling"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ScalingPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.adjustment_type {
            properties.insert(
                "AdjustmentType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AutoScalingGroupName".to_string(),
            crate::value::ToValue::to_value(&self.auto_scaling_group_name),
        );
        if let Some(ref value) = self.cooldown {
            properties.insert(
                "Cooldown".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.estimated_instance_warmup {
            properties.insert(
                "EstimatedInstanceWarmup".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metric_aggregation_type {
            properties.insert(
                "MetricAggregationType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.min_adjustment_magnitude {
            properties.insert(
                "MinAdjustmentMagnitude".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policy_type {
            properties.insert(
                "PolicyType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.predictive_scaling_configuration {
            properties.insert(
                "PredictiveScalingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scaling_adjustment {
            properties.insert(
                "ScalingAdjustment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.step_adjustments {
            properties.insert(
                "StepAdjustments".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.target_tracking_configuration {
            properties.insert(
                "TargetTrackingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-autoscaling-scheduledaction.html
pub struct ScheduledAction_ {
    pub auto_scaling_group_name: crate::value::ExpString,
    pub desired_capacity: Option<i64>,
    pub end_time: Option<crate::value::ExpString>,
    pub max_size: Option<i64>,
    pub min_size: Option<i64>,
    pub recurrence: Option<crate::value::ExpString>,
    pub start_time: Option<crate::value::ExpString>,
    pub time_zone: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_autoscaling_ScheduledAction {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AutoScaling::ScheduledAction"
        $($field $value)*)
    };
}
pub use crate::__aws_autoscaling_ScheduledAction as ScheduledAction;
impl crate::template::ToResource for ScheduledAction_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AutoScaling"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ScheduledAction"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AutoScalingGroupName".to_string(),
            crate::value::ToValue::to_value(&self.auto_scaling_group_name),
        );
        if let Some(ref value) = self.desired_capacity {
            properties.insert(
                "DesiredCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.end_time {
            properties.insert(
                "EndTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_size {
            properties.insert(
                "MaxSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.min_size {
            properties.insert(
                "MinSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.recurrence {
            properties.insert(
                "Recurrence".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.start_time {
            properties.insert(
                "StartTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-autoscaling-warmpool.html
pub struct WarmPool_ {
    pub auto_scaling_group_name: crate::value::ExpString,
    pub instance_reuse_policy: Option<super::autoscaling::warmpool::InstanceReusePolicy_>,
    pub max_group_prepared_capacity: Option<i64>,
    pub min_size: Option<i64>,
    pub pool_state: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_autoscaling_WarmPool {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AutoScaling::WarmPool"
        $($field $value)*)
    };
}
pub use crate::__aws_autoscaling_WarmPool as WarmPool;
impl crate::template::ToResource for WarmPool_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AutoScaling"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WarmPool"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AutoScalingGroupName".to_string(),
            crate::value::ToValue::to_value(&self.auto_scaling_group_name),
        );
        if let Some(ref value) = self.instance_reuse_policy {
            properties.insert(
                "InstanceReusePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_group_prepared_capacity {
            properties.insert(
                "MaxGroupPreparedCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.min_size {
            properties.insert(
                "MinSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.pool_state {
            properties.insert(
                "PoolState".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
