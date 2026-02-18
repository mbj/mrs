pub mod plan {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-arcroutingcontrolconfiguration.html>
    pub struct ArcRoutingControlConfiguration_ {
        pub cross_account_role: Option<crate::value::ExpString>,
        pub external_id: Option<crate::value::ExpString>,
        pub region_and_routing_controls: serde_json::Value,
        pub timeout_minutes: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_ArcRoutingControlConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.ArcRoutingControlConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_ArcRoutingControlConfiguration as ArcRoutingControlConfiguration;
    impl crate::value::ToValue for ArcRoutingControlConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cross_account_role {
                properties.insert(
                    "CrossAccountRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RegionAndRoutingControls".to_string(),
                crate::value::ToValue::to_value(&self.region_and_routing_controls),
            );
            if let Some(ref value) = self.timeout_minutes {
                properties.insert(
                    "TimeoutMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-asg.html>
    pub struct Asg_ {
        pub arn: Option<crate::value::ExpString>,
        pub cross_account_role: Option<crate::value::ExpString>,
        pub external_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_Asg {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.Asg"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_Asg as Asg;
    impl crate::value::ToValue for Asg_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.cross_account_role {
                properties.insert(
                    "CrossAccountRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-associatedalarm.html>
    pub struct AssociatedAlarm_ {
        pub alarm_type: crate::value::ExpString,
        pub cross_account_role: Option<crate::value::ExpString>,
        pub external_id: Option<crate::value::ExpString>,
        pub resource_identifier: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_AssociatedAlarm {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.AssociatedAlarm"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_AssociatedAlarm as AssociatedAlarm;
    impl crate::value::ToValue for AssociatedAlarm_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AlarmType".to_string(),
                crate::value::ToValue::to_value(&self.alarm_type),
            );
            if let Some(ref value) = self.cross_account_role {
                properties.insert(
                    "CrossAccountRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResourceIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.resource_identifier),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-customactionlambdaconfiguration.html>
    pub struct CustomActionLambdaConfiguration_ {
        pub lambdas: Vec<Lambdas_>,
        pub region_to_run: crate::value::ExpString,
        pub retry_interval_minutes: f64,
        pub timeout_minutes: Option<f64>,
        pub ungraceful: Option<Box<LambdaUngraceful_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_CustomActionLambdaConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.CustomActionLambdaConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_CustomActionLambdaConfiguration as CustomActionLambdaConfiguration;
    impl crate::value::ToValue for CustomActionLambdaConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Lambdas".to_string(),
                crate::value::ToValue::to_value(&self.lambdas),
            );
            properties.insert(
                "RegionToRun".to_string(),
                crate::value::ToValue::to_value(&self.region_to_run),
            );
            properties.insert(
                "RetryIntervalMinutes".to_string(),
                crate::value::ToValue::to_value(&self.retry_interval_minutes),
            );
            if let Some(ref value) = self.timeout_minutes {
                properties.insert(
                    "TimeoutMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ungraceful {
                properties.insert(
                    "Ungraceful".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-documentdbconfiguration.html>
    pub struct DocumentDbConfiguration_ {
        pub behavior: serde_json::Value,
        pub cross_account_role: Option<crate::value::ExpString>,
        pub database_cluster_arns: Vec<crate::value::ExpString>,
        pub external_id: Option<crate::value::ExpString>,
        pub global_cluster_identifier: crate::value::ExpString,
        pub timeout_minutes: Option<f64>,
        pub ungraceful: Option<Box<DocumentDbUngraceful_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_DocumentDbConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.DocumentDbConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_DocumentDbConfiguration as DocumentDbConfiguration;
    impl crate::value::ToValue for DocumentDbConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Behavior".to_string(),
                crate::value::ToValue::to_value(&self.behavior),
            );
            if let Some(ref value) = self.cross_account_role {
                properties.insert(
                    "CrossAccountRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DatabaseClusterArns".to_string(),
                crate::value::ToValue::to_value(&self.database_cluster_arns),
            );
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "GlobalClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.global_cluster_identifier),
            );
            if let Some(ref value) = self.timeout_minutes {
                properties.insert(
                    "TimeoutMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ungraceful {
                properties.insert(
                    "Ungraceful".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-documentdbungraceful.html>
    pub struct DocumentDbUngraceful_ {
        pub ungraceful: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_DocumentDbUngraceful {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.DocumentDbUngraceful"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_DocumentDbUngraceful as DocumentDbUngraceful;
    impl crate::value::ToValue for DocumentDbUngraceful_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ungraceful {
                properties.insert(
                    "Ungraceful".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-ec2asgcapacityincreaseconfiguration.html>
    pub struct Ec2AsgCapacityIncreaseConfiguration_ {
        pub asgs: Vec<Asg_>,
        pub capacity_monitoring_approach: Option<serde_json::Value>,
        pub target_percent: Option<f64>,
        pub timeout_minutes: Option<f64>,
        pub ungraceful: Option<Box<Ec2Ungraceful_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_Ec2AsgCapacityIncreaseConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.Ec2AsgCapacityIncreaseConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_Ec2AsgCapacityIncreaseConfiguration as Ec2AsgCapacityIncreaseConfiguration;
    impl crate::value::ToValue for Ec2AsgCapacityIncreaseConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Asgs".to_string(),
                crate::value::ToValue::to_value(&self.asgs),
            );
            if let Some(ref value) = self.capacity_monitoring_approach {
                properties.insert(
                    "CapacityMonitoringApproach".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_percent {
                properties.insert(
                    "TargetPercent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_minutes {
                properties.insert(
                    "TimeoutMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ungraceful {
                properties.insert(
                    "Ungraceful".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-ec2ungraceful.html>
    pub struct Ec2Ungraceful_ {
        pub minimum_success_percentage: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_Ec2Ungraceful {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.Ec2Ungraceful"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_Ec2Ungraceful as Ec2Ungraceful;
    impl crate::value::ToValue for Ec2Ungraceful_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MinimumSuccessPercentage".to_string(),
                crate::value::ToValue::to_value(&self.minimum_success_percentage),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-ecscapacityincreaseconfiguration.html>
    pub struct EcsCapacityIncreaseConfiguration_ {
        pub capacity_monitoring_approach: Option<serde_json::Value>,
        pub services: Vec<Service_>,
        pub target_percent: Option<f64>,
        pub timeout_minutes: Option<f64>,
        pub ungraceful: Option<Box<EcsUngraceful_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_EcsCapacityIncreaseConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.EcsCapacityIncreaseConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_EcsCapacityIncreaseConfiguration as EcsCapacityIncreaseConfiguration;
    impl crate::value::ToValue for EcsCapacityIncreaseConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity_monitoring_approach {
                properties.insert(
                    "CapacityMonitoringApproach".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Services".to_string(),
                crate::value::ToValue::to_value(&self.services),
            );
            if let Some(ref value) = self.target_percent {
                properties.insert(
                    "TargetPercent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_minutes {
                properties.insert(
                    "TimeoutMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ungraceful {
                properties.insert(
                    "Ungraceful".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-ecsungraceful.html>
    pub struct EcsUngraceful_ {
        pub minimum_success_percentage: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_EcsUngraceful {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.EcsUngraceful"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_EcsUngraceful as EcsUngraceful;
    impl crate::value::ToValue for EcsUngraceful_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MinimumSuccessPercentage".to_string(),
                crate::value::ToValue::to_value(&self.minimum_success_percentage),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-ekscluster.html>
    pub struct EksCluster_ {
        pub cluster_arn: crate::value::ExpString,
        pub cross_account_role: Option<crate::value::ExpString>,
        pub external_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_EksCluster {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.EksCluster"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_EksCluster as EksCluster;
    impl crate::value::ToValue for EksCluster_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterArn".to_string(),
                crate::value::ToValue::to_value(&self.cluster_arn),
            );
            if let Some(ref value) = self.cross_account_role {
                properties.insert(
                    "CrossAccountRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-eksresourcescalingconfiguration.html>
    pub struct EksResourceScalingConfiguration_ {
        pub capacity_monitoring_approach: Option<serde_json::Value>,
        pub eks_clusters: Option<Vec<EksCluster_>>,
        pub kubernetes_resource_type: Box<KubernetesResourceType_>,
        pub scaling_resources: Option<serde_json::Value>,
        pub target_percent: Option<f64>,
        pub timeout_minutes: Option<f64>,
        pub ungraceful: Option<Box<EksResourceScalingUngraceful_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_EksResourceScalingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.EksResourceScalingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_EksResourceScalingConfiguration as EksResourceScalingConfiguration;
    impl crate::value::ToValue for EksResourceScalingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity_monitoring_approach {
                properties.insert(
                    "CapacityMonitoringApproach".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.eks_clusters {
                properties.insert(
                    "EksClusters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "KubernetesResourceType".to_string(),
                crate::value::ToValue::to_value(&self.kubernetes_resource_type),
            );
            if let Some(ref value) = self.scaling_resources {
                properties.insert(
                    "ScalingResources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_percent {
                properties.insert(
                    "TargetPercent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_minutes {
                properties.insert(
                    "TimeoutMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ungraceful {
                properties.insert(
                    "Ungraceful".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-eksresourcescalingungraceful.html>
    pub struct EksResourceScalingUngraceful_ {
        pub minimum_success_percentage: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_EksResourceScalingUngraceful {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.EksResourceScalingUngraceful"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_EksResourceScalingUngraceful as EksResourceScalingUngraceful;
    impl crate::value::ToValue for EksResourceScalingUngraceful_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MinimumSuccessPercentage".to_string(),
                crate::value::ToValue::to_value(&self.minimum_success_percentage),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-executionapprovalconfiguration.html>
    pub struct ExecutionApprovalConfiguration_ {
        pub approval_role: crate::value::ExpString,
        pub timeout_minutes: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_ExecutionApprovalConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.ExecutionApprovalConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_ExecutionApprovalConfiguration as ExecutionApprovalConfiguration;
    impl crate::value::ToValue for ExecutionApprovalConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApprovalRole".to_string(),
                crate::value::ToValue::to_value(&self.approval_role),
            );
            if let Some(ref value) = self.timeout_minutes {
                properties.insert(
                    "TimeoutMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-executionblockconfiguration.html>
    pub struct ExecutionBlockConfiguration_ {
        pub arc_routing_control_config: Option<Box<ArcRoutingControlConfiguration_>>,
        pub custom_action_lambda_config: Option<Box<CustomActionLambdaConfiguration_>>,
        pub document_db_config: Option<Box<DocumentDbConfiguration_>>,
        pub ec2_asg_capacity_increase_config: Option<Box<Ec2AsgCapacityIncreaseConfiguration_>>,
        pub ecs_capacity_increase_config: Option<Box<EcsCapacityIncreaseConfiguration_>>,
        pub eks_resource_scaling_config: Option<Box<EksResourceScalingConfiguration_>>,
        pub execution_approval_config: Option<Box<ExecutionApprovalConfiguration_>>,
        pub global_aurora_config: Option<Box<GlobalAuroraConfiguration_>>,
        pub parallel_config: Option<Box<ParallelExecutionBlockConfiguration_>>,
        pub region_switch_plan_config: Option<Box<RegionSwitchPlanConfiguration_>>,
        pub route53_health_check_config: Option<Box<Route53HealthCheckConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_ExecutionBlockConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.ExecutionBlockConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_ExecutionBlockConfiguration as ExecutionBlockConfiguration;
    impl crate::value::ToValue for ExecutionBlockConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arc_routing_control_config {
                properties.insert(
                    "ArcRoutingControlConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_action_lambda_config {
                properties.insert(
                    "CustomActionLambdaConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.document_db_config {
                properties.insert(
                    "DocumentDbConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ec2_asg_capacity_increase_config {
                properties.insert(
                    "Ec2AsgCapacityIncreaseConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ecs_capacity_increase_config {
                properties.insert(
                    "EcsCapacityIncreaseConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.eks_resource_scaling_config {
                properties.insert(
                    "EksResourceScalingConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execution_approval_config {
                properties.insert(
                    "ExecutionApprovalConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.global_aurora_config {
                properties.insert(
                    "GlobalAuroraConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parallel_config {
                properties.insert(
                    "ParallelConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region_switch_plan_config {
                properties.insert(
                    "RegionSwitchPlanConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.route53_health_check_config {
                properties.insert(
                    "Route53HealthCheckConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-globalauroraconfiguration.html>
    pub struct GlobalAuroraConfiguration_ {
        pub behavior: serde_json::Value,
        pub cross_account_role: Option<crate::value::ExpString>,
        pub database_cluster_arns: Vec<crate::value::ExpString>,
        pub external_id: Option<crate::value::ExpString>,
        pub global_cluster_identifier: crate::value::ExpString,
        pub timeout_minutes: Option<f64>,
        pub ungraceful: Option<Box<GlobalAuroraUngraceful_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_GlobalAuroraConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.GlobalAuroraConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_GlobalAuroraConfiguration as GlobalAuroraConfiguration;
    impl crate::value::ToValue for GlobalAuroraConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Behavior".to_string(),
                crate::value::ToValue::to_value(&self.behavior),
            );
            if let Some(ref value) = self.cross_account_role {
                properties.insert(
                    "CrossAccountRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DatabaseClusterArns".to_string(),
                crate::value::ToValue::to_value(&self.database_cluster_arns),
            );
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "GlobalClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.global_cluster_identifier),
            );
            if let Some(ref value) = self.timeout_minutes {
                properties.insert(
                    "TimeoutMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ungraceful {
                properties.insert(
                    "Ungraceful".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-globalauroraungraceful.html>
    pub struct GlobalAuroraUngraceful_ {
        pub ungraceful: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_GlobalAuroraUngraceful {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.GlobalAuroraUngraceful"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_GlobalAuroraUngraceful as GlobalAuroraUngraceful;
    impl crate::value::ToValue for GlobalAuroraUngraceful_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ungraceful {
                properties.insert(
                    "Ungraceful".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-kubernetesresourcetype.html>
    pub struct KubernetesResourceType_ {
        pub api_version: crate::value::ExpString,
        pub kind: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_KubernetesResourceType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.KubernetesResourceType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_KubernetesResourceType as KubernetesResourceType;
    impl crate::value::ToValue for KubernetesResourceType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApiVersion".to_string(),
                crate::value::ToValue::to_value(&self.api_version),
            );
            properties.insert(
                "Kind".to_string(),
                crate::value::ToValue::to_value(&self.kind),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-lambdaungraceful.html>
    pub struct LambdaUngraceful_ {
        pub behavior: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_LambdaUngraceful {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.LambdaUngraceful"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_LambdaUngraceful as LambdaUngraceful;
    impl crate::value::ToValue for LambdaUngraceful_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-lambdas.html>
    pub struct Lambdas_ {
        pub arn: Option<crate::value::ExpString>,
        pub cross_account_role: Option<crate::value::ExpString>,
        pub external_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_Lambdas {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.Lambdas"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_Lambdas as Lambdas;
    impl crate::value::ToValue for Lambdas_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.cross_account_role {
                properties.insert(
                    "CrossAccountRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-parallelexecutionblockconfiguration.html>
    pub struct ParallelExecutionBlockConfiguration_ {
        pub steps: Vec<Step_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_ParallelExecutionBlockConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.ParallelExecutionBlockConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_ParallelExecutionBlockConfiguration as ParallelExecutionBlockConfiguration;
    impl crate::value::ToValue for ParallelExecutionBlockConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Steps".to_string(),
                crate::value::ToValue::to_value(&self.steps),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-regionswitchplanconfiguration.html>
    pub struct RegionSwitchPlanConfiguration_ {
        pub arn: crate::value::ExpString,
        pub cross_account_role: Option<crate::value::ExpString>,
        pub external_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_RegionSwitchPlanConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.RegionSwitchPlanConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_RegionSwitchPlanConfiguration as RegionSwitchPlanConfiguration;
    impl crate::value::ToValue for RegionSwitchPlanConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            if let Some(ref value) = self.cross_account_role {
                properties.insert(
                    "CrossAccountRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-reportconfiguration.html>
    pub struct ReportConfiguration_ {
        pub report_output: Option<Vec<ReportOutputConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_ReportConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.ReportConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_ReportConfiguration as ReportConfiguration;
    impl crate::value::ToValue for ReportConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.report_output {
                properties.insert(
                    "ReportOutput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-reportoutputconfiguration.html>
    pub struct ReportOutputConfiguration_ {
        pub s3_configuration: Box<S3ReportOutputConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_ReportOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.ReportOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_ReportOutputConfiguration as ReportOutputConfiguration;
    impl crate::value::ToValue for ReportOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Configuration".to_string(),
                crate::value::ToValue::to_value(&self.s3_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-route53healthcheckconfiguration.html>
    pub struct Route53HealthCheckConfiguration_ {
        pub cross_account_role: Option<crate::value::ExpString>,
        pub external_id: Option<crate::value::ExpString>,
        pub hosted_zone_id: crate::value::ExpString,
        pub record_name: crate::value::ExpString,
        pub record_sets: Option<Vec<Route53ResourceRecordSet_>>,
        pub timeout_minutes: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_Route53HealthCheckConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.Route53HealthCheckConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_Route53HealthCheckConfiguration as Route53HealthCheckConfiguration;
    impl crate::value::ToValue for Route53HealthCheckConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cross_account_role {
                properties.insert(
                    "CrossAccountRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "HostedZoneId".to_string(),
                crate::value::ToValue::to_value(&self.hosted_zone_id),
            );
            properties.insert(
                "RecordName".to_string(),
                crate::value::ToValue::to_value(&self.record_name),
            );
            if let Some(ref value) = self.record_sets {
                properties.insert(
                    "RecordSets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_minutes {
                properties.insert(
                    "TimeoutMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-route53resourcerecordset.html>
    pub struct Route53ResourceRecordSet_ {
        pub record_set_identifier: Option<crate::value::ExpString>,
        pub region: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_Route53ResourceRecordSet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.Route53ResourceRecordSet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_Route53ResourceRecordSet as Route53ResourceRecordSet;
    impl crate::value::ToValue for Route53ResourceRecordSet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.record_set_identifier {
                properties.insert(
                    "RecordSetIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-s3reportoutputconfiguration.html>
    pub struct S3ReportOutputConfiguration_ {
        pub bucket_owner: Option<crate::value::ExpString>,
        pub bucket_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_S3ReportOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.S3ReportOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_S3ReportOutputConfiguration as S3ReportOutputConfiguration;
    impl crate::value::ToValue for S3ReportOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_owner {
                properties.insert(
                    "BucketOwner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bucket_path {
                properties.insert(
                    "BucketPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-service.html>
    pub struct Service_ {
        pub cluster_arn: Option<crate::value::ExpString>,
        pub cross_account_role: Option<crate::value::ExpString>,
        pub external_id: Option<crate::value::ExpString>,
        pub service_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_Service {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.Service"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_Service as Service;
    impl crate::value::ToValue for Service_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cluster_arn {
                properties.insert(
                    "ClusterArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cross_account_role {
                properties.insert(
                    "CrossAccountRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_arn {
                properties.insert(
                    "ServiceArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-step.html>
    pub struct Step_ {
        pub description: Option<crate::value::ExpString>,
        pub execution_block_configuration: Box<ExecutionBlockConfiguration_>,
        pub execution_block_type: crate::value::ExpString,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_Step {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.Step"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_Step as Step;
    impl crate::value::ToValue for Step_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ExecutionBlockConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.execution_block_configuration),
            );
            properties.insert(
                "ExecutionBlockType".to_string(),
                crate::value::ToValue::to_value(&self.execution_block_type),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-trigger.html>
    pub struct Trigger_ {
        pub action: crate::value::ExpString,
        pub conditions: Vec<TriggerCondition_>,
        pub description: Option<crate::value::ExpString>,
        pub min_delay_minutes_between_executions: f64,
        pub target_region: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_Trigger {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.Trigger"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_Trigger as Trigger;
    impl crate::value::ToValue for Trigger_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "Conditions".to_string(),
                crate::value::ToValue::to_value(&self.conditions),
            );
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MinDelayMinutesBetweenExecutions".to_string(),
                crate::value::ToValue::to_value(&self.min_delay_minutes_between_executions),
            );
            properties.insert(
                "TargetRegion".to_string(),
                crate::value::ToValue::to_value(&self.target_region),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-triggercondition.html>
    pub struct TriggerCondition_ {
        pub associated_alarm_name: crate::value::ExpString,
        pub condition: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_TriggerCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.TriggerCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_TriggerCondition as TriggerCondition;
    impl crate::value::ToValue for TriggerCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AssociatedAlarmName".to_string(),
                crate::value::ToValue::to_value(&self.associated_alarm_name),
            );
            properties.insert(
                "Condition".to_string(),
                crate::value::ToValue::to_value(&self.condition),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-arcregionswitch-plan-workflow.html>
    pub struct Workflow_ {
        pub steps: Option<Vec<Step_>>,
        pub workflow_description: Option<crate::value::ExpString>,
        pub workflow_target_action: crate::value::ExpString,
        pub workflow_target_region: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_arcregionswitch_Plan_Workflow {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ARCRegionSwitch::Plan.Workflow"
            $($field $value)*)
        };
    }
    pub use crate::__aws_arcregionswitch_Plan_Workflow as Workflow;
    impl crate::value::ToValue for Workflow_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.steps {
                properties.insert("Steps".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.workflow_description {
                properties.insert(
                    "WorkflowDescription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "WorkflowTargetAction".to_string(),
                crate::value::ToValue::to_value(&self.workflow_target_action),
            );
            if let Some(ref value) = self.workflow_target_region {
                properties.insert(
                    "WorkflowTargetRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-arcregionswitch-plan.html>
pub struct Plan_ {
    pub associated_alarms:
        Option<std::collections::BTreeMap<String, super::arcregionswitch::plan::AssociatedAlarm_>>,
    pub description: Option<crate::value::ExpString>,
    pub execution_role: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub primary_region: Option<crate::value::ExpString>,
    pub recovery_approach: crate::value::ExpString,
    pub recovery_time_objective_minutes: Option<f64>,
    pub regions: Vec<crate::value::ExpString>,
    pub report_configuration: Option<super::arcregionswitch::plan::ReportConfiguration_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub triggers: Option<Vec<super::arcregionswitch::plan::Trigger_>>,
    pub workflows: Vec<super::arcregionswitch::plan::Workflow_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_arcregionswitch_Plan {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ARCRegionSwitch::Plan"
        $($field $value)*)
    };
}
pub use crate::__aws_arcregionswitch_Plan as Plan;
impl crate::template::ToResource for Plan_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ARCRegionSwitch"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Plan"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.associated_alarms {
            properties.insert(
                "AssociatedAlarms".to_string(),
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
            "ExecutionRole".to_string(),
            crate::value::ToValue::to_value(&self.execution_role),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.primary_region {
            properties.insert(
                "PrimaryRegion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RecoveryApproach".to_string(),
            crate::value::ToValue::to_value(&self.recovery_approach),
        );
        if let Some(ref value) = self.recovery_time_objective_minutes {
            properties.insert(
                "RecoveryTimeObjectiveMinutes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Regions".to_string(),
            crate::value::ToValue::to_value(&self.regions),
        );
        if let Some(ref value) = self.report_configuration {
            properties.insert(
                "ReportConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.triggers {
            properties.insert(
                "Triggers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Workflows".to_string(),
            crate::value::ToValue::to_value(&self.workflows),
        );
        properties
    }
}
