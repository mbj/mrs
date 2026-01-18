pub mod deploymentconfig {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentconfig-minimumhealthyhosts.html
    pub struct MinimumHealthyHosts_ {
        pub r#type: crate::value::ExpString,
        pub value: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentConfig_MinimumHealthyHosts {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentConfig.MinimumHealthyHosts"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentConfig_MinimumHealthyHosts as MinimumHealthyHosts;
    impl crate::value::ToValue for MinimumHealthyHosts_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentconfig-minimumhealthyhostsperzone.html
    pub struct MinimumHealthyHostsPerZone_ {
        pub r#type: crate::value::ExpString,
        pub value: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentConfig_MinimumHealthyHostsPerZone {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentConfig.MinimumHealthyHostsPerZone"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentConfig_MinimumHealthyHostsPerZone as MinimumHealthyHostsPerZone;
    impl crate::value::ToValue for MinimumHealthyHostsPerZone_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentconfig-timebasedcanary.html
    pub struct TimeBasedCanary_ {
        pub canary_interval: i64,
        pub canary_percentage: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentConfig_TimeBasedCanary {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentConfig.TimeBasedCanary"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentConfig_TimeBasedCanary as TimeBasedCanary;
    impl crate::value::ToValue for TimeBasedCanary_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CanaryInterval".to_string(),
                crate::value::ToValue::to_value(&self.canary_interval),
            );
            properties.insert(
                "CanaryPercentage".to_string(),
                crate::value::ToValue::to_value(&self.canary_percentage),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentconfig-timebasedlinear.html
    pub struct TimeBasedLinear_ {
        pub linear_interval: i64,
        pub linear_percentage: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentConfig_TimeBasedLinear {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentConfig.TimeBasedLinear"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentConfig_TimeBasedLinear as TimeBasedLinear;
    impl crate::value::ToValue for TimeBasedLinear_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LinearInterval".to_string(),
                crate::value::ToValue::to_value(&self.linear_interval),
            );
            properties.insert(
                "LinearPercentage".to_string(),
                crate::value::ToValue::to_value(&self.linear_percentage),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentconfig-trafficroutingconfig.html
    pub struct TrafficRoutingConfig_ {
        pub time_based_canary: Option<Box<TimeBasedCanary_>>,
        pub time_based_linear: Option<Box<TimeBasedLinear_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentConfig_TrafficRoutingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentConfig.TrafficRoutingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentConfig_TrafficRoutingConfig as TrafficRoutingConfig;
    impl crate::value::ToValue for TrafficRoutingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.time_based_canary {
                properties.insert(
                    "TimeBasedCanary".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.time_based_linear {
                properties.insert(
                    "TimeBasedLinear".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentconfig-zonalconfig.html
    pub struct ZonalConfig_ {
        pub first_zone_monitor_duration_in_seconds: Option<i64>,
        pub minimum_healthy_hosts_per_zone: Option<Box<MinimumHealthyHostsPerZone_>>,
        pub monitor_duration_in_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentConfig_ZonalConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentConfig.ZonalConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentConfig_ZonalConfig as ZonalConfig;
    impl crate::value::ToValue for ZonalConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.first_zone_monitor_duration_in_seconds {
                properties.insert(
                    "FirstZoneMonitorDurationInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.minimum_healthy_hosts_per_zone {
                properties.insert(
                    "MinimumHealthyHostsPerZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.monitor_duration_in_seconds {
                properties.insert(
                    "MonitorDurationInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod deploymentgroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-alarm.html
    pub struct Alarm_ {
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_Alarm {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.Alarm"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_Alarm as Alarm;
    impl crate::value::ToValue for Alarm_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-alarmconfiguration.html
    pub struct AlarmConfiguration_ {
        pub alarms: Option<Vec<Alarm_>>,
        pub enabled: Option<crate::value::ExpBool>,
        pub ignore_poll_alarm_failure: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_AlarmConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.AlarmConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_AlarmConfiguration as AlarmConfiguration;
    impl crate::value::ToValue for AlarmConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alarms {
                properties.insert("Alarms".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ignore_poll_alarm_failure {
                properties.insert(
                    "IgnorePollAlarmFailure".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-autorollbackconfiguration.html
    pub struct AutoRollbackConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub events: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_AutoRollbackConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.AutoRollbackConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_AutoRollbackConfiguration as AutoRollbackConfiguration;
    impl crate::value::ToValue for AutoRollbackConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.events {
                properties.insert("Events".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-bluegreendeploymentconfiguration.html
    pub struct BlueGreenDeploymentConfiguration_ {
        pub deployment_ready_option: Option<Box<DeploymentReadyOption_>>,
        pub green_fleet_provisioning_option: Option<Box<GreenFleetProvisioningOption_>>,
        pub terminate_blue_instances_on_deployment_success:
            Option<Box<BlueInstanceTerminationOption_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_BlueGreenDeploymentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.BlueGreenDeploymentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_BlueGreenDeploymentConfiguration as BlueGreenDeploymentConfiguration;
    impl crate::value::ToValue for BlueGreenDeploymentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.deployment_ready_option {
                properties.insert(
                    "DeploymentReadyOption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.green_fleet_provisioning_option {
                properties.insert(
                    "GreenFleetProvisioningOption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.terminate_blue_instances_on_deployment_success {
                properties.insert(
                    "TerminateBlueInstancesOnDeploymentSuccess".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-blueinstanceterminationoption.html
    pub struct BlueInstanceTerminationOption_ {
        pub action: Option<crate::value::ExpString>,
        pub termination_wait_time_in_minutes: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_BlueInstanceTerminationOption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.BlueInstanceTerminationOption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_BlueInstanceTerminationOption as BlueInstanceTerminationOption;
    impl crate::value::ToValue for BlueInstanceTerminationOption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.termination_wait_time_in_minutes {
                properties.insert(
                    "TerminationWaitTimeInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment.html
    pub struct Deployment_ {
        pub description: Option<crate::value::ExpString>,
        pub ignore_application_stop_failures: Option<crate::value::ExpBool>,
        pub revision: Box<RevisionLocation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_Deployment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.Deployment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_Deployment as Deployment;
    impl crate::value::ToValue for Deployment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ignore_application_stop_failures {
                properties.insert(
                    "IgnoreApplicationStopFailures".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Revision".to_string(),
                crate::value::ToValue::to_value(&self.revision),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deploymentreadyoption.html
    pub struct DeploymentReadyOption_ {
        pub action_on_timeout: Option<crate::value::ExpString>,
        pub wait_time_in_minutes: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_DeploymentReadyOption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.DeploymentReadyOption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_DeploymentReadyOption as DeploymentReadyOption;
    impl crate::value::ToValue for DeploymentReadyOption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action_on_timeout {
                properties.insert(
                    "ActionOnTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.wait_time_in_minutes {
                properties.insert(
                    "WaitTimeInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deploymentstyle.html
    pub struct DeploymentStyle_ {
        pub deployment_option: Option<crate::value::ExpString>,
        pub deployment_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_DeploymentStyle {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.DeploymentStyle"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_DeploymentStyle as DeploymentStyle;
    impl crate::value::ToValue for DeploymentStyle_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.deployment_option {
                properties.insert(
                    "DeploymentOption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.deployment_type {
                properties.insert(
                    "DeploymentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-ec2tagfilter.html
    pub struct EC2TagFilter_ {
        pub key: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_EC2TagFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.EC2TagFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_EC2TagFilter as EC2TagFilter;
    impl crate::value::ToValue for EC2TagFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-ec2tagset.html
    pub struct EC2TagSet_ {
        pub ec2_tag_set_list: Option<Vec<EC2TagSetListObject_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_EC2TagSet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.EC2TagSet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_EC2TagSet as EC2TagSet;
    impl crate::value::ToValue for EC2TagSet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ec2_tag_set_list {
                properties.insert(
                    "Ec2TagSetList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-ec2tagsetlistobject.html
    pub struct EC2TagSetListObject_ {
        pub ec2_tag_group: Option<Vec<EC2TagFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_EC2TagSetListObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.EC2TagSetListObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_EC2TagSetListObject as EC2TagSetListObject;
    impl crate::value::ToValue for EC2TagSetListObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ec2_tag_group {
                properties.insert(
                    "Ec2TagGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-ecsservice.html
    pub struct ECSService_ {
        pub cluster_name: crate::value::ExpString,
        pub service_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_ECSService {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.ECSService"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_ECSService as ECSService;
    impl crate::value::ToValue for ECSService_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterName".to_string(),
                crate::value::ToValue::to_value(&self.cluster_name),
            );
            properties.insert(
                "ServiceName".to_string(),
                crate::value::ToValue::to_value(&self.service_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-elbinfo.html
    pub struct ELBInfo_ {
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_ELBInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.ELBInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_ELBInfo as ELBInfo;
    impl crate::value::ToValue for ELBInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment-revision-githublocation.html
    pub struct GitHubLocation_ {
        pub commit_id: crate::value::ExpString,
        pub repository: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_GitHubLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.GitHubLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_GitHubLocation as GitHubLocation;
    impl crate::value::ToValue for GitHubLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CommitId".to_string(),
                crate::value::ToValue::to_value(&self.commit_id),
            );
            properties.insert(
                "Repository".to_string(),
                crate::value::ToValue::to_value(&self.repository),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-greenfleetprovisioningoption.html
    pub struct GreenFleetProvisioningOption_ {
        pub action: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_GreenFleetProvisioningOption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.GreenFleetProvisioningOption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_GreenFleetProvisioningOption as GreenFleetProvisioningOption;
    impl crate::value::ToValue for GreenFleetProvisioningOption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-loadbalancerinfo.html
    pub struct LoadBalancerInfo_ {
        pub elb_info_list: Option<Vec<ELBInfo_>>,
        pub target_group_info_list: Option<Vec<TargetGroupInfo_>>,
        pub target_group_pair_info_list: Option<Vec<TargetGroupPairInfo_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_LoadBalancerInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.LoadBalancerInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_LoadBalancerInfo as LoadBalancerInfo;
    impl crate::value::ToValue for LoadBalancerInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.elb_info_list {
                properties.insert(
                    "ElbInfoList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_group_info_list {
                properties.insert(
                    "TargetGroupInfoList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_group_pair_info_list {
                properties.insert(
                    "TargetGroupPairInfoList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-onpremisestagset.html
    pub struct OnPremisesTagSet_ {
        pub on_premises_tag_set_list: Option<Vec<OnPremisesTagSetListObject_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_OnPremisesTagSet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.OnPremisesTagSet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_OnPremisesTagSet as OnPremisesTagSet;
    impl crate::value::ToValue for OnPremisesTagSet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.on_premises_tag_set_list {
                properties.insert(
                    "OnPremisesTagSetList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-onpremisestagsetlistobject.html
    pub struct OnPremisesTagSetListObject_ {
        pub on_premises_tag_group: Option<Vec<TagFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_OnPremisesTagSetListObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.OnPremisesTagSetListObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_OnPremisesTagSetListObject as OnPremisesTagSetListObject;
    impl crate::value::ToValue for OnPremisesTagSetListObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.on_premises_tag_group {
                properties.insert(
                    "OnPremisesTagGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment-revision.html
    pub struct RevisionLocation_ {
        pub git_hub_location: Option<Box<GitHubLocation_>>,
        pub revision_type: Option<crate::value::ExpString>,
        pub s3_location: Option<Box<S3Location_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_RevisionLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.RevisionLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_RevisionLocation as RevisionLocation;
    impl crate::value::ToValue for RevisionLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.git_hub_location {
                properties.insert(
                    "GitHubLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.revision_type {
                properties.insert(
                    "RevisionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_location {
                properties.insert(
                    "S3Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-deployment-revision-s3location.html
    pub struct S3Location_ {
        pub bucket: crate::value::ExpString,
        pub bundle_type: Option<crate::value::ExpString>,
        pub e_tag: Option<crate::value::ExpString>,
        pub key: crate::value::ExpString,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            if let Some(ref value) = self.bundle_type {
                properties.insert(
                    "BundleType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.e_tag {
                properties.insert("ETag".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-tagfilter.html
    pub struct TagFilter_ {
        pub key: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_TagFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.TagFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_TagFilter as TagFilter;
    impl crate::value::ToValue for TagFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-targetgroupinfo.html
    pub struct TargetGroupInfo_ {
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_TargetGroupInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.TargetGroupInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_TargetGroupInfo as TargetGroupInfo;
    impl crate::value::ToValue for TargetGroupInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-targetgrouppairinfo.html
    pub struct TargetGroupPairInfo_ {
        pub prod_traffic_route: Option<Box<TrafficRoute_>>,
        pub target_groups: Option<Vec<TargetGroupInfo_>>,
        pub test_traffic_route: Option<Box<TrafficRoute_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_TargetGroupPairInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.TargetGroupPairInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_TargetGroupPairInfo as TargetGroupPairInfo;
    impl crate::value::ToValue for TargetGroupPairInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.prod_traffic_route {
                properties.insert(
                    "ProdTrafficRoute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_groups {
                properties.insert(
                    "TargetGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.test_traffic_route {
                properties.insert(
                    "TestTrafficRoute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-trafficroute.html
    pub struct TrafficRoute_ {
        pub listener_arns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_TrafficRoute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.TrafficRoute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_TrafficRoute as TrafficRoute;
    impl crate::value::ToValue for TrafficRoute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.listener_arns {
                properties.insert(
                    "ListenerArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codedeploy-deploymentgroup-triggerconfig.html
    pub struct TriggerConfig_ {
        pub trigger_events: Option<Vec<crate::value::ExpString>>,
        pub trigger_name: Option<crate::value::ExpString>,
        pub trigger_target_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codedeploy_DeploymentGroup_TriggerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeDeploy::DeploymentGroup.TriggerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codedeploy_DeploymentGroup_TriggerConfig as TriggerConfig;
    impl crate::value::ToValue for TriggerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.trigger_events {
                properties.insert(
                    "TriggerEvents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.trigger_name {
                properties.insert(
                    "TriggerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.trigger_target_arn {
                properties.insert(
                    "TriggerTargetArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codedeploy-application.html
pub struct Application_ {
    pub application_name: Option<crate::value::ExpString>,
    pub compute_platform: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codedeploy_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeDeploy::Application"
        $($field $value)*)
    };
}
pub use crate::__aws_codedeploy_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeDeploy"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.application_name {
            properties.insert(
                "ApplicationName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.compute_platform {
            properties.insert(
                "ComputePlatform".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codedeploy-deploymentconfig.html
pub struct DeploymentConfig_ {
    pub compute_platform: Option<crate::value::ExpString>,
    pub deployment_config_name: Option<crate::value::ExpString>,
    pub minimum_healthy_hosts: Option<super::codedeploy::deploymentconfig::MinimumHealthyHosts_>,
    pub traffic_routing_config: Option<super::codedeploy::deploymentconfig::TrafficRoutingConfig_>,
    pub zonal_config: Option<super::codedeploy::deploymentconfig::ZonalConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codedeploy_DeploymentConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeDeploy::DeploymentConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_codedeploy_DeploymentConfig as DeploymentConfig;
impl crate::template::ToResource for DeploymentConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeDeploy"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DeploymentConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.compute_platform {
            properties.insert(
                "ComputePlatform".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deployment_config_name {
            properties.insert(
                "DeploymentConfigName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.minimum_healthy_hosts {
            properties.insert(
                "MinimumHealthyHosts".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.traffic_routing_config {
            properties.insert(
                "TrafficRoutingConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.zonal_config {
            properties.insert(
                "ZonalConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codedeploy-deploymentgroup.html
pub struct DeploymentGroup_ {
    pub alarm_configuration: Option<super::codedeploy::deploymentgroup::AlarmConfiguration_>,
    pub application_name: crate::value::ExpString,
    pub auto_rollback_configuration:
        Option<super::codedeploy::deploymentgroup::AutoRollbackConfiguration_>,
    pub auto_scaling_groups: Option<Vec<crate::value::ExpString>>,
    pub blue_green_deployment_configuration:
        Option<super::codedeploy::deploymentgroup::BlueGreenDeploymentConfiguration_>,
    pub deployment: Option<super::codedeploy::deploymentgroup::Deployment_>,
    pub deployment_config_name: Option<crate::value::ExpString>,
    pub deployment_group_name: Option<crate::value::ExpString>,
    pub deployment_style: Option<super::codedeploy::deploymentgroup::DeploymentStyle_>,
    pub ecs_services: Option<Vec<super::codedeploy::deploymentgroup::ECSService_>>,
    pub ec2_tag_filters: Option<Vec<super::codedeploy::deploymentgroup::EC2TagFilter_>>,
    pub ec2_tag_set: Option<super::codedeploy::deploymentgroup::EC2TagSet_>,
    pub load_balancer_info: Option<super::codedeploy::deploymentgroup::LoadBalancerInfo_>,
    pub on_premises_instance_tag_filters:
        Option<Vec<super::codedeploy::deploymentgroup::TagFilter_>>,
    pub on_premises_tag_set: Option<super::codedeploy::deploymentgroup::OnPremisesTagSet_>,
    pub outdated_instances_strategy: Option<crate::value::ExpString>,
    pub service_role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub termination_hook_enabled: Option<crate::value::ExpBool>,
    pub trigger_configurations: Option<Vec<super::codedeploy::deploymentgroup::TriggerConfig_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codedeploy_DeploymentGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeDeploy::DeploymentGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_codedeploy_DeploymentGroup as DeploymentGroup;
impl crate::template::ToResource for DeploymentGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeDeploy"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DeploymentGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.alarm_configuration {
            properties.insert(
                "AlarmConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ApplicationName".to_string(),
            crate::value::ToValue::to_value(&self.application_name),
        );
        if let Some(ref value) = self.auto_rollback_configuration {
            properties.insert(
                "AutoRollbackConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_scaling_groups {
            properties.insert(
                "AutoScalingGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.blue_green_deployment_configuration {
            properties.insert(
                "BlueGreenDeploymentConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deployment {
            properties.insert(
                "Deployment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deployment_config_name {
            properties.insert(
                "DeploymentConfigName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deployment_group_name {
            properties.insert(
                "DeploymentGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deployment_style {
            properties.insert(
                "DeploymentStyle".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ecs_services {
            properties.insert(
                "ECSServices".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ec2_tag_filters {
            properties.insert(
                "Ec2TagFilters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ec2_tag_set {
            properties.insert(
                "Ec2TagSet".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.load_balancer_info {
            properties.insert(
                "LoadBalancerInfo".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.on_premises_instance_tag_filters {
            properties.insert(
                "OnPremisesInstanceTagFilters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.on_premises_tag_set {
            properties.insert(
                "OnPremisesTagSet".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.outdated_instances_strategy {
            properties.insert(
                "OutdatedInstancesStrategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ServiceRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.service_role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.termination_hook_enabled {
            properties.insert(
                "TerminationHookEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.trigger_configurations {
            properties.insert(
                "TriggerConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
