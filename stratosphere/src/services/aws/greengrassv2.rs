pub mod componentversion {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-componentdependencyrequirement.html
    pub struct ComponentDependencyRequirement_ {
        pub dependency_type: Option<crate::value::ExpString>,
        pub version_requirement: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_ComponentVersion_ComponentDependencyRequirement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::ComponentVersion.ComponentDependencyRequirement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_ComponentVersion_ComponentDependencyRequirement as ComponentDependencyRequirement;
    impl crate::value::ToValue for ComponentDependencyRequirement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dependency_type {
                properties.insert(
                    "DependencyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.version_requirement {
                properties.insert(
                    "VersionRequirement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-componentplatform.html
    pub struct ComponentPlatform_ {
        pub attributes: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_ComponentVersion_ComponentPlatform {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::ComponentVersion.ComponentPlatform"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_ComponentVersion_ComponentPlatform as ComponentPlatform;
    impl crate::value::ToValue for ComponentPlatform_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attributes {
                properties.insert(
                    "Attributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdacontainerparams.html
    pub struct LambdaContainerParams_ {
        pub devices: Option<Vec<LambdaDeviceMount_>>,
        pub memory_size_in_kb: Option<i64>,
        pub mount_ro_sysfs: Option<crate::value::ExpBool>,
        pub volumes: Option<Vec<LambdaVolumeMount_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_ComponentVersion_LambdaContainerParams {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::ComponentVersion.LambdaContainerParams"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_ComponentVersion_LambdaContainerParams as LambdaContainerParams;
    impl crate::value::ToValue for LambdaContainerParams_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.devices {
                properties.insert(
                    "Devices".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory_size_in_kb {
                properties.insert(
                    "MemorySizeInKB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mount_ro_sysfs {
                properties.insert(
                    "MountROSysfs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.volumes {
                properties.insert(
                    "Volumes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdadevicemount.html
    pub struct LambdaDeviceMount_ {
        pub add_group_owner: Option<crate::value::ExpBool>,
        pub path: Option<crate::value::ExpString>,
        pub permission: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_ComponentVersion_LambdaDeviceMount {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::ComponentVersion.LambdaDeviceMount"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_ComponentVersion_LambdaDeviceMount as LambdaDeviceMount;
    impl crate::value::ToValue for LambdaDeviceMount_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.add_group_owner {
                properties.insert(
                    "AddGroupOwner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.permission {
                properties.insert(
                    "Permission".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaeventsource.html
    pub struct LambdaEventSource_ {
        pub topic: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_ComponentVersion_LambdaEventSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::ComponentVersion.LambdaEventSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_ComponentVersion_LambdaEventSource as LambdaEventSource;
    impl crate::value::ToValue for LambdaEventSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.topic {
                properties.insert("Topic".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdaexecutionparameters.html
    pub struct LambdaExecutionParameters_ {
        pub environment_variables:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub event_sources: Option<Vec<LambdaEventSource_>>,
        pub exec_args: Option<Vec<crate::value::ExpString>>,
        pub input_payload_encoding_type: Option<crate::value::ExpString>,
        pub linux_process_params: Option<Box<LambdaLinuxProcessParams_>>,
        pub max_idle_time_in_seconds: Option<i64>,
        pub max_instances_count: Option<i64>,
        pub max_queue_size: Option<i64>,
        pub pinned: Option<crate::value::ExpBool>,
        pub status_timeout_in_seconds: Option<i64>,
        pub timeout_in_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_ComponentVersion_LambdaExecutionParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::ComponentVersion.LambdaExecutionParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_ComponentVersion_LambdaExecutionParameters as LambdaExecutionParameters;
    impl crate::value::ToValue for LambdaExecutionParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.environment_variables {
                properties.insert(
                    "EnvironmentVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_sources {
                properties.insert(
                    "EventSources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exec_args {
                properties.insert(
                    "ExecArgs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_payload_encoding_type {
                properties.insert(
                    "InputPayloadEncodingType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.linux_process_params {
                properties.insert(
                    "LinuxProcessParams".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_idle_time_in_seconds {
                properties.insert(
                    "MaxIdleTimeInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_instances_count {
                properties.insert(
                    "MaxInstancesCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_queue_size {
                properties.insert(
                    "MaxQueueSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pinned {
                properties.insert("Pinned".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.status_timeout_in_seconds {
                properties.insert(
                    "StatusTimeoutInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_in_seconds {
                properties.insert(
                    "TimeoutInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdafunctionrecipesource.html
    pub struct LambdaFunctionRecipeSource_ {
        pub component_dependencies:
            Option<std::collections::BTreeMap<String, ComponentDependencyRequirement_>>,
        pub component_lambda_parameters: Option<Box<LambdaExecutionParameters_>>,
        pub component_name: Option<crate::value::ExpString>,
        pub component_platforms: Option<Vec<ComponentPlatform_>>,
        pub component_version: Option<crate::value::ExpString>,
        pub lambda_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_ComponentVersion_LambdaFunctionRecipeSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::ComponentVersion.LambdaFunctionRecipeSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_ComponentVersion_LambdaFunctionRecipeSource as LambdaFunctionRecipeSource;
    impl crate::value::ToValue for LambdaFunctionRecipeSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.component_dependencies {
                properties.insert(
                    "ComponentDependencies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.component_lambda_parameters {
                properties.insert(
                    "ComponentLambdaParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.component_name {
                properties.insert(
                    "ComponentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.component_platforms {
                properties.insert(
                    "ComponentPlatforms".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.component_version {
                properties.insert(
                    "ComponentVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_arn {
                properties.insert(
                    "LambdaArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdalinuxprocessparams.html
    pub struct LambdaLinuxProcessParams_ {
        pub container_params: Option<Box<LambdaContainerParams_>>,
        pub isolation_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_ComponentVersion_LambdaLinuxProcessParams {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::ComponentVersion.LambdaLinuxProcessParams"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_ComponentVersion_LambdaLinuxProcessParams as LambdaLinuxProcessParams;
    impl crate::value::ToValue for LambdaLinuxProcessParams_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_params {
                properties.insert(
                    "ContainerParams".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.isolation_mode {
                properties.insert(
                    "IsolationMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-componentversion-lambdavolumemount.html
    pub struct LambdaVolumeMount_ {
        pub add_group_owner: Option<crate::value::ExpBool>,
        pub destination_path: Option<crate::value::ExpString>,
        pub permission: Option<crate::value::ExpString>,
        pub source_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_ComponentVersion_LambdaVolumeMount {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::ComponentVersion.LambdaVolumeMount"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_ComponentVersion_LambdaVolumeMount as LambdaVolumeMount;
    impl crate::value::ToValue for LambdaVolumeMount_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.add_group_owner {
                properties.insert(
                    "AddGroupOwner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_path {
                properties.insert(
                    "DestinationPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.permission {
                properties.insert(
                    "Permission".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_path {
                properties.insert(
                    "SourcePath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod deployment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-deployment-componentconfigurationupdate.html
    pub struct ComponentConfigurationUpdate_ {
        pub merge: Option<crate::value::ExpString>,
        pub reset: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_Deployment_ComponentConfigurationUpdate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::Deployment.ComponentConfigurationUpdate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_Deployment_ComponentConfigurationUpdate as ComponentConfigurationUpdate;
    impl crate::value::ToValue for ComponentConfigurationUpdate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.merge {
                properties.insert("Merge".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.reset {
                properties.insert("Reset".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-deployment-componentdeploymentspecification.html
    pub struct ComponentDeploymentSpecification_ {
        pub component_version: Option<crate::value::ExpString>,
        pub configuration_update: Option<Box<ComponentConfigurationUpdate_>>,
        pub run_with: Option<Box<ComponentRunWith_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_Deployment_ComponentDeploymentSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::Deployment.ComponentDeploymentSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_Deployment_ComponentDeploymentSpecification as ComponentDeploymentSpecification;
    impl crate::value::ToValue for ComponentDeploymentSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.component_version {
                properties.insert(
                    "ComponentVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.configuration_update {
                properties.insert(
                    "ConfigurationUpdate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.run_with {
                properties.insert(
                    "RunWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-deployment-componentrunwith.html
    pub struct ComponentRunWith_ {
        pub posix_user: Option<crate::value::ExpString>,
        pub system_resource_limits: Option<Box<SystemResourceLimits_>>,
        pub windows_user: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_Deployment_ComponentRunWith {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::Deployment.ComponentRunWith"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_Deployment_ComponentRunWith as ComponentRunWith;
    impl crate::value::ToValue for ComponentRunWith_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.posix_user {
                properties.insert(
                    "PosixUser".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.system_resource_limits {
                properties.insert(
                    "SystemResourceLimits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.windows_user {
                properties.insert(
                    "WindowsUser".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-deployment-deploymentcomponentupdatepolicy.html
    pub struct DeploymentComponentUpdatePolicy_ {
        pub action: Option<crate::value::ExpString>,
        pub timeout_in_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_Deployment_DeploymentComponentUpdatePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::Deployment.DeploymentComponentUpdatePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_Deployment_DeploymentComponentUpdatePolicy as DeploymentComponentUpdatePolicy;
    impl crate::value::ToValue for DeploymentComponentUpdatePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.timeout_in_seconds {
                properties.insert(
                    "TimeoutInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-deployment-deploymentconfigurationvalidationpolicy.html
    pub struct DeploymentConfigurationValidationPolicy_ {
        pub timeout_in_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_Deployment_DeploymentConfigurationValidationPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::Deployment.DeploymentConfigurationValidationPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_Deployment_DeploymentConfigurationValidationPolicy as DeploymentConfigurationValidationPolicy;
    impl crate::value::ToValue for DeploymentConfigurationValidationPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.timeout_in_seconds {
                properties.insert(
                    "TimeoutInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-deployment-deploymentiotjobconfiguration.html
    pub struct DeploymentIoTJobConfiguration_ {
        pub abort_config: Option<Box<IoTJobAbortConfig_>>,
        pub job_executions_rollout_config: Option<Box<IoTJobExecutionsRolloutConfig_>>,
        pub timeout_config: Option<Box<IoTJobTimeoutConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_Deployment_DeploymentIoTJobConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::Deployment.DeploymentIoTJobConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_Deployment_DeploymentIoTJobConfiguration as DeploymentIoTJobConfiguration;
    impl crate::value::ToValue for DeploymentIoTJobConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.abort_config {
                properties.insert(
                    "AbortConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.job_executions_rollout_config {
                properties.insert(
                    "JobExecutionsRolloutConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_config {
                properties.insert(
                    "TimeoutConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-deployment-deploymentpolicies.html
    pub struct DeploymentPolicies_ {
        pub component_update_policy: Option<Box<DeploymentComponentUpdatePolicy_>>,
        pub configuration_validation_policy: Option<Box<DeploymentConfigurationValidationPolicy_>>,
        pub failure_handling_policy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_Deployment_DeploymentPolicies {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::Deployment.DeploymentPolicies"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_Deployment_DeploymentPolicies as DeploymentPolicies;
    impl crate::value::ToValue for DeploymentPolicies_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.component_update_policy {
                properties.insert(
                    "ComponentUpdatePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.configuration_validation_policy {
                properties.insert(
                    "ConfigurationValidationPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failure_handling_policy {
                properties.insert(
                    "FailureHandlingPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-deployment-iotjobabortconfig.html
    pub struct IoTJobAbortConfig_ {
        pub criteria_list: Vec<IoTJobAbortCriteria_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_Deployment_IoTJobAbortConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::Deployment.IoTJobAbortConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_Deployment_IoTJobAbortConfig as IoTJobAbortConfig;
    impl crate::value::ToValue for IoTJobAbortConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CriteriaList".to_string(),
                crate::value::ToValue::to_value(&self.criteria_list),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-deployment-iotjobabortcriteria.html
    pub struct IoTJobAbortCriteria_ {
        pub action: crate::value::ExpString,
        pub failure_type: crate::value::ExpString,
        pub min_number_of_executed_things: i64,
        pub threshold_percentage: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_Deployment_IoTJobAbortCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::Deployment.IoTJobAbortCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_Deployment_IoTJobAbortCriteria as IoTJobAbortCriteria;
    impl crate::value::ToValue for IoTJobAbortCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "FailureType".to_string(),
                crate::value::ToValue::to_value(&self.failure_type),
            );
            properties.insert(
                "MinNumberOfExecutedThings".to_string(),
                crate::value::ToValue::to_value(&self.min_number_of_executed_things),
            );
            properties.insert(
                "ThresholdPercentage".to_string(),
                crate::value::ToValue::to_value(&self.threshold_percentage),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-deployment-iotjobexecutionsrolloutconfig.html
    pub struct IoTJobExecutionsRolloutConfig_ {
        pub exponential_rate: Option<Box<IoTJobExponentialRolloutRate_>>,
        pub maximum_per_minute: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_Deployment_IoTJobExecutionsRolloutConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::Deployment.IoTJobExecutionsRolloutConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_Deployment_IoTJobExecutionsRolloutConfig as IoTJobExecutionsRolloutConfig;
    impl crate::value::ToValue for IoTJobExecutionsRolloutConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exponential_rate {
                properties.insert(
                    "ExponentialRate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_per_minute {
                properties.insert(
                    "MaximumPerMinute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-deployment-iotjobexponentialrolloutrate.html
    pub struct IoTJobExponentialRolloutRate_ {
        pub base_rate_per_minute: i64,
        pub increment_factor: f64,
        pub rate_increase_criteria: Box<IoTJobRateIncreaseCriteria_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_Deployment_IoTJobExponentialRolloutRate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::Deployment.IoTJobExponentialRolloutRate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_Deployment_IoTJobExponentialRolloutRate as IoTJobExponentialRolloutRate;
    impl crate::value::ToValue for IoTJobExponentialRolloutRate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BaseRatePerMinute".to_string(),
                crate::value::ToValue::to_value(&self.base_rate_per_minute),
            );
            properties.insert(
                "IncrementFactor".to_string(),
                crate::value::ToValue::to_value(&self.increment_factor),
            );
            properties.insert(
                "RateIncreaseCriteria".to_string(),
                crate::value::ToValue::to_value(&self.rate_increase_criteria),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-deployment-iotjobrateincreasecriteria.html
    pub struct IoTJobRateIncreaseCriteria_ {
        pub number_of_notified_things: Option<i64>,
        pub number_of_succeeded_things: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_Deployment_IoTJobRateIncreaseCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::Deployment.IoTJobRateIncreaseCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_Deployment_IoTJobRateIncreaseCriteria as IoTJobRateIncreaseCriteria;
    impl crate::value::ToValue for IoTJobRateIncreaseCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.number_of_notified_things {
                properties.insert(
                    "NumberOfNotifiedThings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_of_succeeded_things {
                properties.insert(
                    "NumberOfSucceededThings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-deployment-iotjobtimeoutconfig.html
    pub struct IoTJobTimeoutConfig_ {
        pub in_progress_timeout_in_minutes: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_Deployment_IoTJobTimeoutConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::Deployment.IoTJobTimeoutConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_Deployment_IoTJobTimeoutConfig as IoTJobTimeoutConfig;
    impl crate::value::ToValue for IoTJobTimeoutConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.in_progress_timeout_in_minutes {
                properties.insert(
                    "InProgressTimeoutInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-greengrassv2-deployment-systemresourcelimits.html
    pub struct SystemResourceLimits_ {
        pub cpus: Option<f64>,
        pub memory: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_greengrassv2_Deployment_SystemResourceLimits {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GreengrassV2::Deployment.SystemResourceLimits"
            $($field $value)*)
        };
    }
    pub use crate::__aws_greengrassv2_Deployment_SystemResourceLimits as SystemResourceLimits;
    impl crate::value::ToValue for SystemResourceLimits_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cpus {
                properties.insert("Cpus".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.memory {
                properties.insert("Memory".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrassv2-componentversion.html
pub struct ComponentVersion_ {
    pub inline_recipe: Option<crate::value::ExpString>,
    pub lambda_function: Option<super::greengrassv2::componentversion::LambdaFunctionRecipeSource_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrassv2_ComponentVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GreengrassV2::ComponentVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrassv2_ComponentVersion as ComponentVersion;
impl crate::template::ToResource for ComponentVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GreengrassV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ComponentVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.inline_recipe {
            properties.insert(
                "InlineRecipe".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lambda_function {
            properties.insert(
                "LambdaFunction".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-greengrassv2-deployment.html
pub struct Deployment_ {
    pub components: Option<
        std::collections::BTreeMap<
            String,
            super::greengrassv2::deployment::ComponentDeploymentSpecification_,
        >,
    >,
    pub deployment_name: Option<crate::value::ExpString>,
    pub deployment_policies: Option<super::greengrassv2::deployment::DeploymentPolicies_>,
    pub iot_job_configuration:
        Option<super::greengrassv2::deployment::DeploymentIoTJobConfiguration_>,
    pub parent_target_arn: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub target_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_greengrassv2_Deployment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GreengrassV2::Deployment"
        $($field $value)*)
    };
}
pub use crate::__aws_greengrassv2_Deployment as Deployment;
impl crate::template::ToResource for Deployment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GreengrassV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Deployment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.components {
            properties.insert(
                "Components".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deployment_name {
            properties.insert(
                "DeploymentName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deployment_policies {
            properties.insert(
                "DeploymentPolicies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iot_job_configuration {
            properties.insert(
                "IotJobConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.parent_target_arn {
            properties.insert(
                "ParentTargetArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
