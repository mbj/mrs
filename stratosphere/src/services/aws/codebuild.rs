pub mod fleet {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-fleet-computeconfiguration.html
    pub struct ComputeConfiguration_ {
        pub disk: Option<i32>,
        pub instance_type: Option<crate::value::ExpString>,
        pub machine_type: Option<crate::value::ExpString>,
        pub memory: Option<i32>,
        pub v_cpu: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Fleet_ComputeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Fleet.ComputeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Fleet_ComputeConfiguration as ComputeConfiguration;
    impl crate::value::ToValue for ComputeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.disk {
                properties.insert("disk".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.instance_type {
                properties.insert(
                    "instanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.machine_type {
                properties.insert(
                    "machineType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory {
                properties.insert("memory".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.v_cpu {
                properties.insert("vCpu".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-fleet-fleetproxyrule.html
    pub struct FleetProxyRule_ {
        pub effect: Option<crate::value::ExpString>,
        pub entities: Option<Vec<crate::value::ExpString>>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Fleet_FleetProxyRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Fleet.FleetProxyRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Fleet_FleetProxyRule as FleetProxyRule;
    impl crate::value::ToValue for FleetProxyRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.effect {
                properties.insert("Effect".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.entities {
                properties.insert(
                    "Entities".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-fleet-proxyconfiguration.html
    pub struct ProxyConfiguration_ {
        pub default_behavior: Option<crate::value::ExpString>,
        pub ordered_proxy_rules: Option<Vec<FleetProxyRule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Fleet_ProxyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Fleet.ProxyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Fleet_ProxyConfiguration as ProxyConfiguration;
    impl crate::value::ToValue for ProxyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_behavior {
                properties.insert(
                    "DefaultBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ordered_proxy_rules {
                properties.insert(
                    "OrderedProxyRules".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-fleet-scalingconfigurationinput.html
    pub struct ScalingConfigurationInput_ {
        pub max_capacity: Option<i32>,
        pub scaling_type: Option<crate::value::ExpString>,
        pub target_tracking_scaling_configs: Option<Vec<TargetTrackingScalingConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Fleet_ScalingConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Fleet.ScalingConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Fleet_ScalingConfigurationInput as ScalingConfigurationInput;
    impl crate::value::ToValue for ScalingConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_capacity {
                properties.insert(
                    "MaxCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scaling_type {
                properties.insert(
                    "ScalingType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_tracking_scaling_configs {
                properties.insert(
                    "TargetTrackingScalingConfigs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-fleet-targettrackingscalingconfiguration.html
    pub struct TargetTrackingScalingConfiguration_ {
        pub metric_type: Option<crate::value::ExpString>,
        pub target_value: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Fleet_TargetTrackingScalingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Fleet.TargetTrackingScalingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Fleet_TargetTrackingScalingConfiguration as TargetTrackingScalingConfiguration;
    impl crate::value::ToValue for TargetTrackingScalingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.metric_type {
                properties.insert(
                    "MetricType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_value {
                properties.insert(
                    "TargetValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-fleet-vpcconfig.html
    pub struct VpcConfig_ {
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnets: Option<Vec<crate::value::ExpString>>,
        pub vpc_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Fleet_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Fleet.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Fleet_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnets {
                properties.insert(
                    "Subnets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_id {
                properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod project {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-artifacts.html
    pub struct Artifacts_ {
        pub artifact_identifier: Option<crate::value::ExpString>,
        pub encryption_disabled: Option<crate::value::ExpBool>,
        pub location: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub namespace_type: Option<crate::value::ExpString>,
        pub override_artifact_name: Option<crate::value::ExpBool>,
        pub packaging: Option<crate::value::ExpString>,
        pub path: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_Artifacts {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.Artifacts"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_Artifacts as Artifacts;
    impl crate::value::ToValue for Artifacts_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.artifact_identifier {
                properties.insert(
                    "ArtifactIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_disabled {
                properties.insert(
                    "EncryptionDisabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.location {
                properties.insert(
                    "Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.namespace_type {
                properties.insert(
                    "NamespaceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.override_artifact_name {
                properties.insert(
                    "OverrideArtifactName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.packaging {
                properties.insert(
                    "Packaging".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-batchrestrictions.html
    pub struct BatchRestrictions_ {
        pub compute_types_allowed: Option<Vec<crate::value::ExpString>>,
        pub maximum_builds_allowed: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_BatchRestrictions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.BatchRestrictions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_BatchRestrictions as BatchRestrictions;
    impl crate::value::ToValue for BatchRestrictions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.compute_types_allowed {
                properties.insert(
                    "ComputeTypesAllowed".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_builds_allowed {
                properties.insert(
                    "MaximumBuildsAllowed".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-buildstatusconfig.html
    pub struct BuildStatusConfig_ {
        pub context: Option<crate::value::ExpString>,
        pub target_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_BuildStatusConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.BuildStatusConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_BuildStatusConfig as BuildStatusConfig;
    impl crate::value::ToValue for BuildStatusConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.context {
                properties.insert(
                    "Context".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_url {
                properties.insert(
                    "TargetUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-cloudwatchlogsconfig.html
    pub struct CloudWatchLogsConfig_ {
        pub group_name: Option<crate::value::ExpString>,
        pub status: crate::value::ExpString,
        pub stream_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_CloudWatchLogsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.CloudWatchLogsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_CloudWatchLogsConfig as CloudWatchLogsConfig;
    impl crate::value::ToValue for CloudWatchLogsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.group_name {
                properties.insert(
                    "GroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            if let Some(ref value) = self.stream_name {
                properties.insert(
                    "StreamName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-dockerserver.html
    pub struct DockerServer_ {
        pub compute_type: crate::value::ExpString,
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_DockerServer {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.DockerServer"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_DockerServer as DockerServer;
    impl crate::value::ToValue for DockerServer_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ComputeType".to_string(),
                crate::value::ToValue::to_value(&self.compute_type),
            );
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environment.html
    pub struct Environment_ {
        pub certificate: Option<crate::value::ExpString>,
        pub compute_type: crate::value::ExpString,
        pub docker_server: Option<Box<DockerServer_>>,
        pub environment_variables: Option<Vec<EnvironmentVariable_>>,
        pub fleet: Option<Box<ProjectFleet_>>,
        pub image: crate::value::ExpString,
        pub image_pull_credentials_type: Option<crate::value::ExpString>,
        pub privileged_mode: Option<crate::value::ExpBool>,
        pub registry_credential: Option<Box<RegistryCredential_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_Environment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.Environment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_Environment as Environment;
    impl crate::value::ToValue for Environment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate {
                properties.insert(
                    "Certificate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ComputeType".to_string(),
                crate::value::ToValue::to_value(&self.compute_type),
            );
            if let Some(ref value) = self.docker_server {
                properties.insert(
                    "DockerServer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment_variables {
                properties.insert(
                    "EnvironmentVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fleet {
                properties.insert("Fleet".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Image".to_string(),
                crate::value::ToValue::to_value(&self.image),
            );
            if let Some(ref value) = self.image_pull_credentials_type {
                properties.insert(
                    "ImagePullCredentialsType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.privileged_mode {
                properties.insert(
                    "PrivilegedMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.registry_credential {
                properties.insert(
                    "RegistryCredential".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environmentvariable.html
    pub struct EnvironmentVariable_ {
        pub name: crate::value::ExpString,
        pub r#type: Option<crate::value::ExpString>,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_EnvironmentVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.EnvironmentVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_EnvironmentVariable as EnvironmentVariable;
    impl crate::value::ToValue for EnvironmentVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-filtergroup.html
    pub struct FilterGroup_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_FilterGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.FilterGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_FilterGroup as FilterGroup;
    impl crate::value::ToValue for FilterGroup_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-gitsubmodulesconfig.html
    pub struct GitSubmodulesConfig_ {
        pub fetch_submodules: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_GitSubmodulesConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.GitSubmodulesConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_GitSubmodulesConfig as GitSubmodulesConfig;
    impl crate::value::ToValue for GitSubmodulesConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FetchSubmodules".to_string(),
                crate::value::ToValue::to_value(&self.fetch_submodules),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-logsconfig.html
    pub struct LogsConfig_ {
        pub cloud_watch_logs: Option<Box<CloudWatchLogsConfig_>>,
        pub s3_logs: Option<Box<S3LogsConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_LogsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.LogsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_LogsConfig as LogsConfig;
    impl crate::value::ToValue for LogsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_logs {
                properties.insert(
                    "CloudWatchLogs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_logs {
                properties.insert("S3Logs".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectbuildbatchconfig.html
    pub struct ProjectBuildBatchConfig_ {
        pub batch_report_mode: Option<crate::value::ExpString>,
        pub combine_artifacts: Option<crate::value::ExpBool>,
        pub restrictions: Option<Box<BatchRestrictions_>>,
        pub service_role: Option<crate::value::ExpString>,
        pub timeout_in_mins: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_ProjectBuildBatchConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.ProjectBuildBatchConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_ProjectBuildBatchConfig as ProjectBuildBatchConfig;
    impl crate::value::ToValue for ProjectBuildBatchConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_report_mode {
                properties.insert(
                    "BatchReportMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.combine_artifacts {
                properties.insert(
                    "CombineArtifacts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.restrictions {
                properties.insert(
                    "Restrictions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_role {
                properties.insert(
                    "ServiceRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_in_mins {
                properties.insert(
                    "TimeoutInMins".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectcache.html
    pub struct ProjectCache_ {
        pub cache_namespace: Option<crate::value::ExpString>,
        pub location: Option<crate::value::ExpString>,
        pub modes: Option<Vec<crate::value::ExpString>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_ProjectCache {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.ProjectCache"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_ProjectCache as ProjectCache;
    impl crate::value::ToValue for ProjectCache_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cache_namespace {
                properties.insert(
                    "CacheNamespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.location {
                properties.insert(
                    "Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.modes {
                properties.insert("Modes".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectfilesystemlocation.html
    pub struct ProjectFileSystemLocation_ {
        pub identifier: crate::value::ExpString,
        pub location: crate::value::ExpString,
        pub mount_options: Option<crate::value::ExpString>,
        pub mount_point: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_ProjectFileSystemLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.ProjectFileSystemLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_ProjectFileSystemLocation as ProjectFileSystemLocation;
    impl crate::value::ToValue for ProjectFileSystemLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Identifier".to_string(),
                crate::value::ToValue::to_value(&self.identifier),
            );
            properties.insert(
                "Location".to_string(),
                crate::value::ToValue::to_value(&self.location),
            );
            if let Some(ref value) = self.mount_options {
                properties.insert(
                    "MountOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MountPoint".to_string(),
                crate::value::ToValue::to_value(&self.mount_point),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectfleet.html
    pub struct ProjectFleet_ {
        pub fleet_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_ProjectFleet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.ProjectFleet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_ProjectFleet as ProjectFleet;
    impl crate::value::ToValue for ProjectFleet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.fleet_arn {
                properties.insert(
                    "FleetArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectsourceversion.html
    pub struct ProjectSourceVersion_ {
        pub source_identifier: crate::value::ExpString,
        pub source_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_ProjectSourceVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.ProjectSourceVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_ProjectSourceVersion as ProjectSourceVersion;
    impl crate::value::ToValue for ProjectSourceVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SourceIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.source_identifier),
            );
            if let Some(ref value) = self.source_version {
                properties.insert(
                    "SourceVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projecttriggers.html
    pub struct ProjectTriggers_ {
        pub build_type: Option<crate::value::ExpString>,
        pub filter_groups: Option<Vec<FilterGroup_>>,
        pub pull_request_build_policy: Option<Box<PullRequestBuildPolicy_>>,
        pub scope_configuration: Option<Box<ScopeConfiguration_>>,
        pub webhook: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_ProjectTriggers {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.ProjectTriggers"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_ProjectTriggers as ProjectTriggers;
    impl crate::value::ToValue for ProjectTriggers_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.build_type {
                properties.insert(
                    "BuildType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filter_groups {
                properties.insert(
                    "FilterGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pull_request_build_policy {
                properties.insert(
                    "PullRequestBuildPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scope_configuration {
                properties.insert(
                    "ScopeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.webhook {
                properties.insert(
                    "Webhook".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-pullrequestbuildpolicy.html
    pub struct PullRequestBuildPolicy_ {
        pub approver_roles: Option<Vec<crate::value::ExpString>>,
        pub requires_comment_approval: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_PullRequestBuildPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.PullRequestBuildPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_PullRequestBuildPolicy as PullRequestBuildPolicy;
    impl crate::value::ToValue for PullRequestBuildPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.approver_roles {
                properties.insert(
                    "ApproverRoles".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RequiresCommentApproval".to_string(),
                crate::value::ToValue::to_value(&self.requires_comment_approval),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-registrycredential.html
    pub struct RegistryCredential_ {
        pub credential: crate::value::ExpString,
        pub credential_provider: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_RegistryCredential {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.RegistryCredential"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_RegistryCredential as RegistryCredential;
    impl crate::value::ToValue for RegistryCredential_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Credential".to_string(),
                crate::value::ToValue::to_value(&self.credential),
            );
            properties.insert(
                "CredentialProvider".to_string(),
                crate::value::ToValue::to_value(&self.credential_provider),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-s3logsconfig.html
    pub struct S3LogsConfig_ {
        pub encryption_disabled: Option<crate::value::ExpBool>,
        pub location: Option<crate::value::ExpString>,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_S3LogsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.S3LogsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_S3LogsConfig as S3LogsConfig;
    impl crate::value::ToValue for S3LogsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption_disabled {
                properties.insert(
                    "EncryptionDisabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.location {
                properties.insert(
                    "Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-scopeconfiguration.html
    pub struct ScopeConfiguration_ {
        pub domain: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub scope: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_ScopeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.ScopeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_ScopeConfiguration as ScopeConfiguration;
    impl crate::value::ToValue for ScopeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.domain {
                properties.insert("Domain".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.scope {
                properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html
    pub struct Source_ {
        pub auth: Option<Box<SourceAuth_>>,
        pub build_spec: Option<crate::value::ExpString>,
        pub build_status_config: Option<Box<BuildStatusConfig_>>,
        pub git_clone_depth: Option<i32>,
        pub git_submodules_config: Option<Box<GitSubmodulesConfig_>>,
        pub insecure_ssl: Option<crate::value::ExpBool>,
        pub location: Option<crate::value::ExpString>,
        pub report_build_status: Option<crate::value::ExpBool>,
        pub source_identifier: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_Source {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.Source"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_Source as Source;
    impl crate::value::ToValue for Source_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auth {
                properties.insert("Auth".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.build_spec {
                properties.insert(
                    "BuildSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.build_status_config {
                properties.insert(
                    "BuildStatusConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.git_clone_depth {
                properties.insert(
                    "GitCloneDepth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.git_submodules_config {
                properties.insert(
                    "GitSubmodulesConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.insecure_ssl {
                properties.insert(
                    "InsecureSsl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.location {
                properties.insert(
                    "Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.report_build_status {
                properties.insert(
                    "ReportBuildStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_identifier {
                properties.insert(
                    "SourceIdentifier".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-sourceauth.html
    pub struct SourceAuth_ {
        pub resource: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_SourceAuth {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.SourceAuth"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_SourceAuth as SourceAuth;
    impl crate::value::ToValue for SourceAuth_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.resource {
                properties.insert(
                    "Resource".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-vpcconfig.html
    pub struct VpcConfig_ {
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnets: Option<Vec<crate::value::ExpString>>,
        pub vpc_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnets {
                properties.insert(
                    "Subnets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_id {
                properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-webhookfilter.html
    pub struct WebhookFilter_ {
        pub exclude_matched_pattern: Option<crate::value::ExpBool>,
        pub pattern: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_Project_WebhookFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::Project.WebhookFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_Project_WebhookFilter as WebhookFilter;
    impl crate::value::ToValue for WebhookFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclude_matched_pattern {
                properties.insert(
                    "ExcludeMatchedPattern".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Pattern".to_string(),
                crate::value::ToValue::to_value(&self.pattern),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
pub mod reportgroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-reportgroup-reportexportconfig.html
    pub struct ReportExportConfig_ {
        pub export_config_type: crate::value::ExpString,
        pub s3_destination: Option<Box<S3ReportExportConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_ReportGroup_ReportExportConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::ReportGroup.ReportExportConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_ReportGroup_ReportExportConfig as ReportExportConfig;
    impl crate::value::ToValue for ReportExportConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ExportConfigType".to_string(),
                crate::value::ToValue::to_value(&self.export_config_type),
            );
            if let Some(ref value) = self.s3_destination {
                properties.insert(
                    "S3Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-reportgroup-s3reportexportconfig.html
    pub struct S3ReportExportConfig_ {
        pub bucket: crate::value::ExpString,
        pub bucket_owner: Option<crate::value::ExpString>,
        pub encryption_disabled: Option<crate::value::ExpBool>,
        pub encryption_key: Option<crate::value::ExpString>,
        pub packaging: Option<crate::value::ExpString>,
        pub path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codebuild_ReportGroup_S3ReportExportConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeBuild::ReportGroup.S3ReportExportConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codebuild_ReportGroup_S3ReportExportConfig as S3ReportExportConfig;
    impl crate::value::ToValue for S3ReportExportConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            if let Some(ref value) = self.bucket_owner {
                properties.insert(
                    "BucketOwner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_disabled {
                properties.insert(
                    "EncryptionDisabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_key {
                properties.insert(
                    "EncryptionKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.packaging {
                properties.insert(
                    "Packaging".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-fleet.html
pub struct Fleet_ {
    pub base_capacity: Option<i32>,
    pub compute_configuration: Option<super::codebuild::fleet::ComputeConfiguration_>,
    pub compute_type: Option<crate::value::ExpString>,
    pub environment_type: Option<crate::value::ExpString>,
    pub fleet_proxy_configuration: Option<super::codebuild::fleet::ProxyConfiguration_>,
    pub fleet_service_role: Option<crate::value::ExpString>,
    pub fleet_vpc_config: Option<super::codebuild::fleet::VpcConfig_>,
    pub image_id: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub overflow_behavior: Option<crate::value::ExpString>,
    pub scaling_configuration: Option<super::codebuild::fleet::ScalingConfigurationInput_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codebuild_Fleet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeBuild::Fleet"
        $($field $value)*)
    };
}
pub use crate::__aws_codebuild_Fleet as Fleet;
impl crate::template::ToResource for Fleet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeBuild"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Fleet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.base_capacity {
            properties.insert(
                "BaseCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.compute_configuration {
            properties.insert(
                "ComputeConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.compute_type {
            properties.insert(
                "ComputeType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_type {
            properties.insert(
                "EnvironmentType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.fleet_proxy_configuration {
            properties.insert(
                "FleetProxyConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.fleet_service_role {
            properties.insert(
                "FleetServiceRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.fleet_vpc_config {
            properties.insert(
                "FleetVpcConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_id {
            properties.insert(
                "ImageId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.overflow_behavior {
            properties.insert(
                "OverflowBehavior".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scaling_configuration {
            properties.insert(
                "ScalingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html
pub struct Project_ {
    pub artifacts: super::codebuild::project::Artifacts_,
    pub auto_retry_limit: Option<i32>,
    pub badge_enabled: Option<crate::value::ExpBool>,
    pub build_batch_config: Option<super::codebuild::project::ProjectBuildBatchConfig_>,
    pub cache: Option<super::codebuild::project::ProjectCache_>,
    pub concurrent_build_limit: Option<i32>,
    pub description: Option<crate::value::ExpString>,
    pub encryption_key: Option<crate::value::ExpString>,
    pub environment: super::codebuild::project::Environment_,
    pub file_system_locations: Option<Vec<super::codebuild::project::ProjectFileSystemLocation_>>,
    pub logs_config: Option<super::codebuild::project::LogsConfig_>,
    pub name: Option<crate::value::ExpString>,
    pub queued_timeout_in_minutes: Option<i32>,
    pub resource_access_role: Option<crate::value::ExpString>,
    pub secondary_artifacts: Option<Vec<super::codebuild::project::Artifacts_>>,
    pub secondary_source_versions: Option<Vec<super::codebuild::project::ProjectSourceVersion_>>,
    pub secondary_sources: Option<Vec<super::codebuild::project::Source_>>,
    pub service_role: crate::value::ExpString,
    pub source: super::codebuild::project::Source_,
    pub source_version: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub timeout_in_minutes: Option<i32>,
    pub triggers: Option<super::codebuild::project::ProjectTriggers_>,
    pub visibility: Option<crate::value::ExpString>,
    pub vpc_config: Option<super::codebuild::project::VpcConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codebuild_Project {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeBuild::Project"
        $($field $value)*)
    };
}
pub use crate::__aws_codebuild_Project as Project;
impl crate::template::ToResource for Project_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeBuild"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Project"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Artifacts".to_string(),
            crate::value::ToValue::to_value(&self.artifacts),
        );
        if let Some(ref value) = self.auto_retry_limit {
            properties.insert(
                "AutoRetryLimit".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.badge_enabled {
            properties.insert(
                "BadgeEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.build_batch_config {
            properties.insert(
                "BuildBatchConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cache {
            properties.insert("Cache".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.concurrent_build_limit {
            properties.insert(
                "ConcurrentBuildLimit".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_key {
            properties.insert(
                "EncryptionKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Environment".to_string(),
            crate::value::ToValue::to_value(&self.environment),
        );
        if let Some(ref value) = self.file_system_locations {
            properties.insert(
                "FileSystemLocations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logs_config {
            properties.insert(
                "LogsConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.queued_timeout_in_minutes {
            properties.insert(
                "QueuedTimeoutInMinutes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_access_role {
            properties.insert(
                "ResourceAccessRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.secondary_artifacts {
            properties.insert(
                "SecondaryArtifacts".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.secondary_source_versions {
            properties.insert(
                "SecondarySourceVersions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.secondary_sources {
            properties.insert(
                "SecondarySources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ServiceRole".to_string(),
            crate::value::ToValue::to_value(&self.service_role),
        );
        properties.insert(
            "Source".to_string(),
            crate::value::ToValue::to_value(&self.source),
        );
        if let Some(ref value) = self.source_version {
            properties.insert(
                "SourceVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.timeout_in_minutes {
            properties.insert(
                "TimeoutInMinutes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.triggers {
            properties.insert(
                "Triggers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.visibility {
            properties.insert(
                "Visibility".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_config {
            properties.insert(
                "VpcConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-reportgroup.html
pub struct ReportGroup_ {
    pub delete_reports: Option<crate::value::ExpBool>,
    pub export_config: super::codebuild::reportgroup::ReportExportConfig_,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codebuild_ReportGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeBuild::ReportGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_codebuild_ReportGroup as ReportGroup;
impl crate::template::ToResource for ReportGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeBuild"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ReportGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.delete_reports {
            properties.insert(
                "DeleteReports".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ExportConfig".to_string(),
            crate::value::ToValue::to_value(&self.export_config),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-sourcecredential.html
pub struct SourceCredential_ {
    pub auth_type: crate::value::ExpString,
    pub server_type: crate::value::ExpString,
    pub token: crate::value::ExpString,
    pub username: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codebuild_SourceCredential {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeBuild::SourceCredential"
        $($field $value)*)
    };
}
pub use crate::__aws_codebuild_SourceCredential as SourceCredential;
impl crate::template::ToResource for SourceCredential_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeBuild"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SourceCredential"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AuthType".to_string(),
            crate::value::ToValue::to_value(&self.auth_type),
        );
        properties.insert(
            "ServerType".to_string(),
            crate::value::ToValue::to_value(&self.server_type),
        );
        properties.insert(
            "Token".to_string(),
            crate::value::ToValue::to_value(&self.token),
        );
        if let Some(ref value) = self.username {
            properties.insert(
                "Username".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
