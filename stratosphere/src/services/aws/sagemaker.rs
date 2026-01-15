pub mod app {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-app-resourcespec.html
    pub struct ResourceSpec_ {
        pub instance_type: Option<crate::value::ExpString>,
        pub lifecycle_config_arn: Option<crate::value::ExpString>,
        pub sage_maker_image_arn: Option<crate::value::ExpString>,
        pub sage_maker_image_version_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_App_ResourceSpec {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::App.ResourceSpec"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_App_ResourceSpec as ResourceSpec;
    impl crate::value::ToValue for ResourceSpec_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instance_type {
                properties.insert(
                    "InstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_config_arn {
                properties.insert(
                    "LifecycleConfigArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sage_maker_image_arn {
                properties.insert(
                    "SageMakerImageArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sage_maker_image_version_arn {
                properties.insert(
                    "SageMakerImageVersionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod appimageconfig {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-codeeditorappimageconfig.html
    pub struct CodeEditorAppImageConfig_ {
        pub container_config: Option<Box<ContainerConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_AppImageConfig_CodeEditorAppImageConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::AppImageConfig.CodeEditorAppImageConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_AppImageConfig_CodeEditorAppImageConfig as CodeEditorAppImageConfig;
    impl crate::value::ToValue for CodeEditorAppImageConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_config {
                properties.insert(
                    "ContainerConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-containerconfig.html
    pub struct ContainerConfig_ {
        pub container_arguments: Option<Vec<crate::value::ExpString>>,
        pub container_entrypoint: Option<Vec<crate::value::ExpString>>,
        pub container_environment_variables: Option<Vec<CustomImageContainerEnvironmentVariable_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_AppImageConfig_ContainerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::AppImageConfig.ContainerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_AppImageConfig_ContainerConfig as ContainerConfig;
    impl crate::value::ToValue for ContainerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_arguments {
                properties.insert(
                    "ContainerArguments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_entrypoint {
                properties.insert(
                    "ContainerEntrypoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_environment_variables {
                properties.insert(
                    "ContainerEnvironmentVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-customimagecontainerenvironmentvariable.html
    pub struct CustomImageContainerEnvironmentVariable_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_AppImageConfig_CustomImageContainerEnvironmentVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::AppImageConfig.CustomImageContainerEnvironmentVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_AppImageConfig_CustomImageContainerEnvironmentVariable as CustomImageContainerEnvironmentVariable;
    impl crate::value::ToValue for CustomImageContainerEnvironmentVariable_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-filesystemconfig.html
    pub struct FileSystemConfig_ {
        pub default_gid: Option<i64>,
        pub default_uid: Option<i64>,
        pub mount_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_AppImageConfig_FileSystemConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::AppImageConfig.FileSystemConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_AppImageConfig_FileSystemConfig as FileSystemConfig;
    impl crate::value::ToValue for FileSystemConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_gid {
                properties.insert(
                    "DefaultGid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_uid {
                properties.insert(
                    "DefaultUid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mount_path {
                properties.insert(
                    "MountPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-jupyterlabappimageconfig.html
    pub struct JupyterLabAppImageConfig_ {
        pub container_config: Option<Box<ContainerConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_AppImageConfig_JupyterLabAppImageConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::AppImageConfig.JupyterLabAppImageConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_AppImageConfig_JupyterLabAppImageConfig as JupyterLabAppImageConfig;
    impl crate::value::ToValue for JupyterLabAppImageConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_config {
                properties.insert(
                    "ContainerConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-kernelgatewayimageconfig.html
    pub struct KernelGatewayImageConfig_ {
        pub file_system_config: Option<Box<FileSystemConfig_>>,
        pub kernel_specs: Vec<KernelSpec_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_AppImageConfig_KernelGatewayImageConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::AppImageConfig.KernelGatewayImageConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_AppImageConfig_KernelGatewayImageConfig as KernelGatewayImageConfig;
    impl crate::value::ToValue for KernelGatewayImageConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.file_system_config {
                properties.insert(
                    "FileSystemConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "KernelSpecs".to_string(),
                crate::value::ToValue::to_value(&self.kernel_specs),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-kernelspec.html
    pub struct KernelSpec_ {
        pub display_name: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_AppImageConfig_KernelSpec {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::AppImageConfig.KernelSpec"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_AppImageConfig_KernelSpec as KernelSpec;
    impl crate::value::ToValue for KernelSpec_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.display_name {
                properties.insert(
                    "DisplayName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
}
pub mod cluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-cluster-alarmdetails.html
    pub struct AlarmDetails_ {
        pub alarm_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Cluster_AlarmDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Cluster.AlarmDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Cluster_AlarmDetails as AlarmDetails;
    impl crate::value::ToValue for AlarmDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AlarmName".to_string(),
                crate::value::ToValue::to_value(&self.alarm_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-cluster-capacitysizeconfig.html
    pub struct CapacitySizeConfig_ {
        pub r#type: crate::value::ExpString,
        pub value: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Cluster_CapacitySizeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Cluster.CapacitySizeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Cluster_CapacitySizeConfig as CapacitySizeConfig;
    impl crate::value::ToValue for CapacitySizeConfig_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-cluster-clusterautoscalingconfig.html
    pub struct ClusterAutoScalingConfig_ {
        pub auto_scaler_type: Option<crate::value::ExpString>,
        pub mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Cluster_ClusterAutoScalingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Cluster.ClusterAutoScalingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Cluster_ClusterAutoScalingConfig as ClusterAutoScalingConfig;
    impl crate::value::ToValue for ClusterAutoScalingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_scaler_type {
                properties.insert(
                    "AutoScalerType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Mode".to_string(),
                crate::value::ToValue::to_value(&self.mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-cluster-clusterebsvolumeconfig.html
    pub struct ClusterEbsVolumeConfig_ {
        pub root_volume: Option<crate::value::ExpBool>,
        pub volume_kms_key_id: Option<crate::value::ExpString>,
        pub volume_size_in_gb: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Cluster_ClusterEbsVolumeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Cluster.ClusterEbsVolumeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Cluster_ClusterEbsVolumeConfig as ClusterEbsVolumeConfig;
    impl crate::value::ToValue for ClusterEbsVolumeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.root_volume {
                properties.insert(
                    "RootVolume".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.volume_kms_key_id {
                properties.insert(
                    "VolumeKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.volume_size_in_gb {
                properties.insert(
                    "VolumeSizeInGB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-cluster-clusterinstancegroup.html
    pub struct ClusterInstanceGroup_ {
        pub current_count: Option<i64>,
        pub execution_role: crate::value::ExpString,
        pub image_id: Option<crate::value::ExpString>,
        pub instance_count: i64,
        pub instance_group_name: crate::value::ExpString,
        pub instance_storage_configs: Option<Vec<ClusterInstanceStorageConfig_>>,
        pub instance_type: crate::value::ExpString,
        pub life_cycle_config: Box<ClusterLifeCycleConfig_>,
        pub on_start_deep_health_checks: Option<Vec<crate::value::ExpString>>,
        pub override_vpc_config: Option<Box<VpcConfig_>>,
        pub scheduled_update_config: Option<Box<ScheduledUpdateConfig_>>,
        pub threads_per_core: Option<i64>,
        pub training_plan_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Cluster_ClusterInstanceGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Cluster.ClusterInstanceGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Cluster_ClusterInstanceGroup as ClusterInstanceGroup;
    impl crate::value::ToValue for ClusterInstanceGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.current_count {
                properties.insert(
                    "CurrentCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ExecutionRole".to_string(),
                crate::value::ToValue::to_value(&self.execution_role),
            );
            if let Some(ref value) = self.image_id {
                properties.insert(
                    "ImageId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InstanceCount".to_string(),
                crate::value::ToValue::to_value(&self.instance_count),
            );
            properties.insert(
                "InstanceGroupName".to_string(),
                crate::value::ToValue::to_value(&self.instance_group_name),
            );
            if let Some(ref value) = self.instance_storage_configs {
                properties.insert(
                    "InstanceStorageConfigs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            properties.insert(
                "LifeCycleConfig".to_string(),
                crate::value::ToValue::to_value(&self.life_cycle_config),
            );
            if let Some(ref value) = self.on_start_deep_health_checks {
                properties.insert(
                    "OnStartDeepHealthChecks".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.override_vpc_config {
                properties.insert(
                    "OverrideVpcConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scheduled_update_config {
                properties.insert(
                    "ScheduledUpdateConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.threads_per_core {
                properties.insert(
                    "ThreadsPerCore".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.training_plan_arn {
                properties.insert(
                    "TrainingPlanArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-cluster-clusterinstancestorageconfig.html
    pub struct ClusterInstanceStorageConfig_ {
        pub ebs_volume_config: Option<Box<ClusterEbsVolumeConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Cluster_ClusterInstanceStorageConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Cluster.ClusterInstanceStorageConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Cluster_ClusterInstanceStorageConfig as ClusterInstanceStorageConfig;
    impl crate::value::ToValue for ClusterInstanceStorageConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ebs_volume_config {
                properties.insert(
                    "EbsVolumeConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-cluster-clusterlifecycleconfig.html
    pub struct ClusterLifeCycleConfig_ {
        pub on_create: crate::value::ExpString,
        pub source_s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Cluster_ClusterLifeCycleConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Cluster.ClusterLifeCycleConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Cluster_ClusterLifeCycleConfig as ClusterLifeCycleConfig;
    impl crate::value::ToValue for ClusterLifeCycleConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OnCreate".to_string(),
                crate::value::ToValue::to_value(&self.on_create),
            );
            properties.insert(
                "SourceS3Uri".to_string(),
                crate::value::ToValue::to_value(&self.source_s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-cluster-clusterorchestratoreksconfig.html
    pub struct ClusterOrchestratorEksConfig_ {
        pub cluster_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Cluster_ClusterOrchestratorEksConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Cluster.ClusterOrchestratorEksConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Cluster_ClusterOrchestratorEksConfig as ClusterOrchestratorEksConfig;
    impl crate::value::ToValue for ClusterOrchestratorEksConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterArn".to_string(),
                crate::value::ToValue::to_value(&self.cluster_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-cluster-clusterrestrictedinstancegroup.html
    pub struct ClusterRestrictedInstanceGroup_ {
        pub current_count: Option<i64>,
        pub environment_config: Box<EnvironmentConfig_>,
        pub execution_role: crate::value::ExpString,
        pub instance_count: i64,
        pub instance_group_name: crate::value::ExpString,
        pub instance_storage_configs: Option<Vec<ClusterInstanceStorageConfig_>>,
        pub instance_type: crate::value::ExpString,
        pub on_start_deep_health_checks: Option<Vec<crate::value::ExpString>>,
        pub override_vpc_config: Option<Box<VpcConfig_>>,
        pub threads_per_core: Option<i64>,
        pub training_plan_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Cluster_ClusterRestrictedInstanceGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Cluster.ClusterRestrictedInstanceGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Cluster_ClusterRestrictedInstanceGroup as ClusterRestrictedInstanceGroup;
    impl crate::value::ToValue for ClusterRestrictedInstanceGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.current_count {
                properties.insert(
                    "CurrentCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EnvironmentConfig".to_string(),
                crate::value::ToValue::to_value(&self.environment_config),
            );
            properties.insert(
                "ExecutionRole".to_string(),
                crate::value::ToValue::to_value(&self.execution_role),
            );
            properties.insert(
                "InstanceCount".to_string(),
                crate::value::ToValue::to_value(&self.instance_count),
            );
            properties.insert(
                "InstanceGroupName".to_string(),
                crate::value::ToValue::to_value(&self.instance_group_name),
            );
            if let Some(ref value) = self.instance_storage_configs {
                properties.insert(
                    "InstanceStorageConfigs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            if let Some(ref value) = self.on_start_deep_health_checks {
                properties.insert(
                    "OnStartDeepHealthChecks".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.override_vpc_config {
                properties.insert(
                    "OverrideVpcConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.threads_per_core {
                properties.insert(
                    "ThreadsPerCore".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.training_plan_arn {
                properties.insert(
                    "TrainingPlanArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-cluster-deploymentconfig.html
    pub struct DeploymentConfig_ {
        pub auto_rollback_configuration: Option<Vec<AlarmDetails_>>,
        pub rolling_update_policy: Option<Box<RollingUpdatePolicy_>>,
        pub wait_interval_in_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Cluster_DeploymentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Cluster.DeploymentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Cluster_DeploymentConfig as DeploymentConfig;
    impl crate::value::ToValue for DeploymentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_rollback_configuration {
                properties.insert(
                    "AutoRollbackConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rolling_update_policy {
                properties.insert(
                    "RollingUpdatePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.wait_interval_in_seconds {
                properties.insert(
                    "WaitIntervalInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-cluster-environmentconfig.html
    pub struct EnvironmentConfig_ {
        pub f_sx_lustre_config: Option<Box<FSxLustreConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Cluster_EnvironmentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Cluster.EnvironmentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Cluster_EnvironmentConfig as EnvironmentConfig;
    impl crate::value::ToValue for EnvironmentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.f_sx_lustre_config {
                properties.insert(
                    "FSxLustreConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-cluster-fsxlustreconfig.html
    pub struct FSxLustreConfig_ {
        pub per_unit_storage_throughput: i64,
        pub size_in_gi_b: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Cluster_FSxLustreConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Cluster.FSxLustreConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Cluster_FSxLustreConfig as FSxLustreConfig;
    impl crate::value::ToValue for FSxLustreConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PerUnitStorageThroughput".to_string(),
                crate::value::ToValue::to_value(&self.per_unit_storage_throughput),
            );
            properties.insert(
                "SizeInGiB".to_string(),
                crate::value::ToValue::to_value(&self.size_in_gi_b),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-cluster-orchestrator.html
    pub struct Orchestrator_ {
        pub eks: Box<ClusterOrchestratorEksConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Cluster_Orchestrator {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Cluster.Orchestrator"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Cluster_Orchestrator as Orchestrator;
    impl crate::value::ToValue for Orchestrator_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Eks".to_string(),
                crate::value::ToValue::to_value(&self.eks),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-cluster-rollingupdatepolicy.html
    pub struct RollingUpdatePolicy_ {
        pub maximum_batch_size: Box<CapacitySizeConfig_>,
        pub rollback_maximum_batch_size: Option<Box<CapacitySizeConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Cluster_RollingUpdatePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Cluster.RollingUpdatePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Cluster_RollingUpdatePolicy as RollingUpdatePolicy;
    impl crate::value::ToValue for RollingUpdatePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaximumBatchSize".to_string(),
                crate::value::ToValue::to_value(&self.maximum_batch_size),
            );
            if let Some(ref value) = self.rollback_maximum_batch_size {
                properties.insert(
                    "RollbackMaximumBatchSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-cluster-scheduledupdateconfig.html
    pub struct ScheduledUpdateConfig_ {
        pub deployment_config: Option<Box<DeploymentConfig_>>,
        pub schedule_expression: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Cluster_ScheduledUpdateConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Cluster.ScheduledUpdateConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Cluster_ScheduledUpdateConfig as ScheduledUpdateConfig;
    impl crate::value::ToValue for ScheduledUpdateConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.deployment_config {
                properties.insert(
                    "DeploymentConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ScheduleExpression".to_string(),
                crate::value::ToValue::to_value(&self.schedule_expression),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-cluster-vpcconfig.html
    pub struct VpcConfig_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Cluster_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Cluster.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Cluster_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
}
pub mod coderepository {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-coderepository-gitconfig.html
    pub struct GitConfig_ {
        pub branch: Option<crate::value::ExpString>,
        pub repository_url: crate::value::ExpString,
        pub secret_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_CodeRepository_GitConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::CodeRepository.GitConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_CodeRepository_GitConfig as GitConfig;
    impl crate::value::ToValue for GitConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.branch {
                properties.insert("Branch".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "RepositoryUrl".to_string(),
                crate::value::ToValue::to_value(&self.repository_url),
            );
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod dataqualityjobdefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-batchtransforminput.html
    pub struct BatchTransformInput_ {
        pub data_captured_destination_s3_uri: crate::value::ExpString,
        pub dataset_format: Box<DatasetFormat_>,
        pub exclude_features_attribute: Option<crate::value::ExpString>,
        pub local_path: crate::value::ExpString,
        pub s3_data_distribution_type: Option<crate::value::ExpString>,
        pub s3_input_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_BatchTransformInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.BatchTransformInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_BatchTransformInput as BatchTransformInput;
    impl crate::value::ToValue for BatchTransformInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataCapturedDestinationS3Uri".to_string(),
                crate::value::ToValue::to_value(&self.data_captured_destination_s3_uri),
            );
            properties.insert(
                "DatasetFormat".to_string(),
                crate::value::ToValue::to_value(&self.dataset_format),
            );
            if let Some(ref value) = self.exclude_features_attribute {
                properties.insert(
                    "ExcludeFeaturesAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LocalPath".to_string(),
                crate::value::ToValue::to_value(&self.local_path),
            );
            if let Some(ref value) = self.s3_data_distribution_type {
                properties.insert(
                    "S3DataDistributionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_input_mode {
                properties.insert(
                    "S3InputMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-clusterconfig.html
    pub struct ClusterConfig_ {
        pub instance_count: i64,
        pub instance_type: crate::value::ExpString,
        pub volume_kms_key_id: Option<crate::value::ExpString>,
        pub volume_size_in_gb: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_ClusterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.ClusterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_ClusterConfig as ClusterConfig;
    impl crate::value::ToValue for ClusterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceCount".to_string(),
                crate::value::ToValue::to_value(&self.instance_count),
            );
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            if let Some(ref value) = self.volume_kms_key_id {
                properties.insert(
                    "VolumeKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VolumeSizeInGB".to_string(),
                crate::value::ToValue::to_value(&self.volume_size_in_gb),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-constraintsresource.html
    pub struct ConstraintsResource_ {
        pub s3_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_ConstraintsResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.ConstraintsResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_ConstraintsResource as ConstraintsResource;
    impl crate::value::ToValue for ConstraintsResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_uri {
                properties.insert("S3Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-csv.html
    pub struct Csv_ {
        pub header: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_Csv {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.Csv"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_Csv as Csv;
    impl crate::value::ToValue for Csv_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.header {
                properties.insert("Header".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-dataqualityappspecification.html
    pub struct DataQualityAppSpecification_ {
        pub container_arguments: Option<Vec<crate::value::ExpString>>,
        pub container_entrypoint: Option<Vec<crate::value::ExpString>>,
        pub environment: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub image_uri: crate::value::ExpString,
        pub post_analytics_processor_source_uri: Option<crate::value::ExpString>,
        pub record_preprocessor_source_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_DataQualityAppSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.DataQualityAppSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_DataQualityAppSpecification as DataQualityAppSpecification;
    impl crate::value::ToValue for DataQualityAppSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_arguments {
                properties.insert(
                    "ContainerArguments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_entrypoint {
                properties.insert(
                    "ContainerEntrypoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ImageUri".to_string(),
                crate::value::ToValue::to_value(&self.image_uri),
            );
            if let Some(ref value) = self.post_analytics_processor_source_uri {
                properties.insert(
                    "PostAnalyticsProcessorSourceUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.record_preprocessor_source_uri {
                properties.insert(
                    "RecordPreprocessorSourceUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-dataqualitybaselineconfig.html
    pub struct DataQualityBaselineConfig_ {
        pub baselining_job_name: Option<crate::value::ExpString>,
        pub constraints_resource: Option<Box<ConstraintsResource_>>,
        pub statistics_resource: Option<Box<StatisticsResource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_DataQualityBaselineConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.DataQualityBaselineConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_DataQualityBaselineConfig as DataQualityBaselineConfig;
    impl crate::value::ToValue for DataQualityBaselineConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.baselining_job_name {
                properties.insert(
                    "BaseliningJobName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.constraints_resource {
                properties.insert(
                    "ConstraintsResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.statistics_resource {
                properties.insert(
                    "StatisticsResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-dataqualityjobinput.html
    pub struct DataQualityJobInput_ {
        pub batch_transform_input: Option<Box<BatchTransformInput_>>,
        pub endpoint_input: Option<Box<EndpointInput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_DataQualityJobInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.DataQualityJobInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_DataQualityJobInput as DataQualityJobInput;
    impl crate::value::ToValue for DataQualityJobInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_transform_input {
                properties.insert(
                    "BatchTransformInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.endpoint_input {
                properties.insert(
                    "EndpointInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-datasetformat.html
    pub struct DatasetFormat_ {
        pub csv: Option<Box<Csv_>>,
        pub json: Option<Box<Json_>>,
        pub parquet: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_DatasetFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.DatasetFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_DatasetFormat as DatasetFormat;
    impl crate::value::ToValue for DatasetFormat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.csv {
                properties.insert("Csv".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.json {
                properties.insert("Json".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.parquet {
                properties.insert(
                    "Parquet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-endpointinput.html
    pub struct EndpointInput_ {
        pub endpoint_name: crate::value::ExpString,
        pub exclude_features_attribute: Option<crate::value::ExpString>,
        pub local_path: crate::value::ExpString,
        pub s3_data_distribution_type: Option<crate::value::ExpString>,
        pub s3_input_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_EndpointInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.EndpointInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_EndpointInput as EndpointInput;
    impl crate::value::ToValue for EndpointInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EndpointName".to_string(),
                crate::value::ToValue::to_value(&self.endpoint_name),
            );
            if let Some(ref value) = self.exclude_features_attribute {
                properties.insert(
                    "ExcludeFeaturesAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LocalPath".to_string(),
                crate::value::ToValue::to_value(&self.local_path),
            );
            if let Some(ref value) = self.s3_data_distribution_type {
                properties.insert(
                    "S3DataDistributionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_input_mode {
                properties.insert(
                    "S3InputMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-json.html
    pub struct Json_ {
        pub line: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_Json {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.Json"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_Json as Json;
    impl crate::value::ToValue for Json_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.line {
                properties.insert("Line".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-monitoringoutput.html
    pub struct MonitoringOutput_ {
        pub s3_output: Box<S3Output_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_MonitoringOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.MonitoringOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_MonitoringOutput as MonitoringOutput;
    impl crate::value::ToValue for MonitoringOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Output".to_string(),
                crate::value::ToValue::to_value(&self.s3_output),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-monitoringoutputconfig.html
    pub struct MonitoringOutputConfig_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub monitoring_outputs: Vec<MonitoringOutput_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_MonitoringOutputConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.MonitoringOutputConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_MonitoringOutputConfig as MonitoringOutputConfig;
    impl crate::value::ToValue for MonitoringOutputConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MonitoringOutputs".to_string(),
                crate::value::ToValue::to_value(&self.monitoring_outputs),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-monitoringresources.html
    pub struct MonitoringResources_ {
        pub cluster_config: Box<ClusterConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_MonitoringResources {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.MonitoringResources"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_MonitoringResources as MonitoringResources;
    impl crate::value::ToValue for MonitoringResources_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterConfig".to_string(),
                crate::value::ToValue::to_value(&self.cluster_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-networkconfig.html
    pub struct NetworkConfig_ {
        pub enable_inter_container_traffic_encryption: Option<crate::value::ExpBool>,
        pub enable_network_isolation: Option<crate::value::ExpBool>,
        pub vpc_config: Option<Box<VpcConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_NetworkConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.NetworkConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_NetworkConfig as NetworkConfig;
    impl crate::value::ToValue for NetworkConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_inter_container_traffic_encryption {
                properties.insert(
                    "EnableInterContainerTrafficEncryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_network_isolation {
                properties.insert(
                    "EnableNetworkIsolation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_config {
                properties.insert(
                    "VpcConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-s3output.html
    pub struct S3Output_ {
        pub local_path: crate::value::ExpString,
        pub s3_upload_mode: Option<crate::value::ExpString>,
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_S3Output {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.S3Output"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_S3Output as S3Output;
    impl crate::value::ToValue for S3Output_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LocalPath".to_string(),
                crate::value::ToValue::to_value(&self.local_path),
            );
            if let Some(ref value) = self.s3_upload_mode {
                properties.insert(
                    "S3UploadMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-statisticsresource.html
    pub struct StatisticsResource_ {
        pub s3_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_StatisticsResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.StatisticsResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_StatisticsResource as StatisticsResource;
    impl crate::value::ToValue for StatisticsResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_uri {
                properties.insert("S3Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-stoppingcondition.html
    pub struct StoppingCondition_ {
        pub max_runtime_in_seconds: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_StoppingCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.StoppingCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_StoppingCondition as StoppingCondition;
    impl crate::value::ToValue for StoppingCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxRuntimeInSeconds".to_string(),
                crate::value::ToValue::to_value(&self.max_runtime_in_seconds),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-vpcconfig.html
    pub struct VpcConfig_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DataQualityJobDefinition_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DataQualityJobDefinition.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DataQualityJobDefinition_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
}
pub mod device {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-device-device.html
    pub struct Device_ {
        pub description: Option<crate::value::ExpString>,
        pub device_name: crate::value::ExpString,
        pub iot_thing_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Device_Device {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Device.Device"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Device_Device as Device;
    impl crate::value::ToValue for Device_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DeviceName".to_string(),
                crate::value::ToValue::to_value(&self.device_name),
            );
            if let Some(ref value) = self.iot_thing_name {
                properties.insert(
                    "IotThingName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod devicefleet {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-devicefleet-edgeoutputconfig.html
    pub struct EdgeOutputConfig_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub s3_output_location: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_DeviceFleet_EdgeOutputConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::DeviceFleet.EdgeOutputConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_DeviceFleet_EdgeOutputConfig as EdgeOutputConfig;
    impl crate::value::ToValue for EdgeOutputConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3OutputLocation".to_string(),
                crate::value::ToValue::to_value(&self.s3_output_location),
            );
            properties.into()
        }
    }
}
pub mod domain {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-applifecyclemanagement.html
    pub struct AppLifecycleManagement_ {
        pub idle_settings: Option<Box<IdleSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_AppLifecycleManagement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.AppLifecycleManagement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_AppLifecycleManagement as AppLifecycleManagement;
    impl crate::value::ToValue for AppLifecycleManagement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idle_settings {
                properties.insert(
                    "IdleSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-codeeditorappsettings.html
    pub struct CodeEditorAppSettings_ {
        pub app_lifecycle_management: Option<Box<AppLifecycleManagement_>>,
        pub built_in_lifecycle_config_arn: Option<crate::value::ExpString>,
        pub custom_images: Option<Vec<CustomImage_>>,
        pub default_resource_spec: Option<Box<ResourceSpec_>>,
        pub lifecycle_config_arns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_CodeEditorAppSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.CodeEditorAppSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_CodeEditorAppSettings as CodeEditorAppSettings;
    impl crate::value::ToValue for CodeEditorAppSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_lifecycle_management {
                properties.insert(
                    "AppLifecycleManagement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.built_in_lifecycle_config_arn {
                properties.insert(
                    "BuiltInLifecycleConfigArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_images {
                properties.insert(
                    "CustomImages".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_resource_spec {
                properties.insert(
                    "DefaultResourceSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_config_arns {
                properties.insert(
                    "LifecycleConfigArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-coderepository.html
    pub struct CodeRepository_ {
        pub repository_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_CodeRepository {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.CodeRepository"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_CodeRepository as CodeRepository;
    impl crate::value::ToValue for CodeRepository_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RepositoryUrl".to_string(),
                crate::value::ToValue::to_value(&self.repository_url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-customfilesystemconfig.html
    pub struct CustomFileSystemConfig_ {
        pub efs_file_system_config: Option<Box<EFSFileSystemConfig_>>,
        pub f_sx_lustre_file_system_config: Option<Box<FSxLustreFileSystemConfig_>>,
        pub s3_file_system_config: Option<Box<S3FileSystemConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_CustomFileSystemConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.CustomFileSystemConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_CustomFileSystemConfig as CustomFileSystemConfig;
    impl crate::value::ToValue for CustomFileSystemConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.efs_file_system_config {
                properties.insert(
                    "EFSFileSystemConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.f_sx_lustre_file_system_config {
                properties.insert(
                    "FSxLustreFileSystemConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_file_system_config {
                properties.insert(
                    "S3FileSystemConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-customimage.html
    pub struct CustomImage_ {
        pub app_image_config_name: crate::value::ExpString,
        pub image_name: crate::value::ExpString,
        pub image_version_number: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_CustomImage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.CustomImage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_CustomImage as CustomImage;
    impl crate::value::ToValue for CustomImage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppImageConfigName".to_string(),
                crate::value::ToValue::to_value(&self.app_image_config_name),
            );
            properties.insert(
                "ImageName".to_string(),
                crate::value::ToValue::to_value(&self.image_name),
            );
            if let Some(ref value) = self.image_version_number {
                properties.insert(
                    "ImageVersionNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-customposixuserconfig.html
    pub struct CustomPosixUserConfig_ {
        pub gid: i64,
        pub uid: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_CustomPosixUserConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.CustomPosixUserConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_CustomPosixUserConfig as CustomPosixUserConfig;
    impl crate::value::ToValue for CustomPosixUserConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Gid".to_string(),
                crate::value::ToValue::to_value(&self.gid),
            );
            properties.insert(
                "Uid".to_string(),
                crate::value::ToValue::to_value(&self.uid),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-defaultebsstoragesettings.html
    pub struct DefaultEbsStorageSettings_ {
        pub default_ebs_volume_size_in_gb: i64,
        pub maximum_ebs_volume_size_in_gb: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_DefaultEbsStorageSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.DefaultEbsStorageSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_DefaultEbsStorageSettings as DefaultEbsStorageSettings;
    impl crate::value::ToValue for DefaultEbsStorageSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DefaultEbsVolumeSizeInGb".to_string(),
                crate::value::ToValue::to_value(&self.default_ebs_volume_size_in_gb),
            );
            properties.insert(
                "MaximumEbsVolumeSizeInGb".to_string(),
                crate::value::ToValue::to_value(&self.maximum_ebs_volume_size_in_gb),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-defaultspacesettings.html
    pub struct DefaultSpaceSettings_ {
        pub custom_file_system_configs: Option<Vec<CustomFileSystemConfig_>>,
        pub custom_posix_user_config: Option<Box<CustomPosixUserConfig_>>,
        pub execution_role: crate::value::ExpString,
        pub jupyter_lab_app_settings: Option<Box<JupyterLabAppSettings_>>,
        pub jupyter_server_app_settings: Option<Box<JupyterServerAppSettings_>>,
        pub kernel_gateway_app_settings: Option<Box<KernelGatewayAppSettings_>>,
        pub security_groups: Option<Vec<crate::value::ExpString>>,
        pub space_storage_settings: Option<Box<DefaultSpaceStorageSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_DefaultSpaceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.DefaultSpaceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_DefaultSpaceSettings as DefaultSpaceSettings;
    impl crate::value::ToValue for DefaultSpaceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_file_system_configs {
                properties.insert(
                    "CustomFileSystemConfigs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_posix_user_config {
                properties.insert(
                    "CustomPosixUserConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ExecutionRole".to_string(),
                crate::value::ToValue::to_value(&self.execution_role),
            );
            if let Some(ref value) = self.jupyter_lab_app_settings {
                properties.insert(
                    "JupyterLabAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.jupyter_server_app_settings {
                properties.insert(
                    "JupyterServerAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kernel_gateway_app_settings {
                properties.insert(
                    "KernelGatewayAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_groups {
                properties.insert(
                    "SecurityGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.space_storage_settings {
                properties.insert(
                    "SpaceStorageSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-defaultspacestoragesettings.html
    pub struct DefaultSpaceStorageSettings_ {
        pub default_ebs_storage_settings: Option<Box<DefaultEbsStorageSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_DefaultSpaceStorageSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.DefaultSpaceStorageSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_DefaultSpaceStorageSettings as DefaultSpaceStorageSettings;
    impl crate::value::ToValue for DefaultSpaceStorageSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_ebs_storage_settings {
                properties.insert(
                    "DefaultEbsStorageSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-dockersettings.html
    pub struct DockerSettings_ {
        pub enable_docker_access: Option<crate::value::ExpString>,
        pub vpc_only_trusted_accounts: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_DockerSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.DockerSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_DockerSettings as DockerSettings;
    impl crate::value::ToValue for DockerSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_docker_access {
                properties.insert(
                    "EnableDockerAccess".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_only_trusted_accounts {
                properties.insert(
                    "VpcOnlyTrustedAccounts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-domainsettings.html
    pub struct DomainSettings_ {
        pub docker_settings: Option<Box<DockerSettings_>>,
        pub execution_role_identity_config: Option<crate::value::ExpString>,
        pub r_studio_server_pro_domain_settings: Option<Box<RStudioServerProDomainSettings_>>,
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub unified_studio_settings: Option<Box<UnifiedStudioSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_DomainSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.DomainSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_DomainSettings as DomainSettings;
    impl crate::value::ToValue for DomainSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.docker_settings {
                properties.insert(
                    "DockerSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execution_role_identity_config {
                properties.insert(
                    "ExecutionRoleIdentityConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r_studio_server_pro_domain_settings {
                properties.insert(
                    "RStudioServerProDomainSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unified_studio_settings {
                properties.insert(
                    "UnifiedStudioSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-efsfilesystemconfig.html
    pub struct EFSFileSystemConfig_ {
        pub file_system_id: crate::value::ExpString,
        pub file_system_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_EFSFileSystemConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.EFSFileSystemConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_EFSFileSystemConfig as EFSFileSystemConfig;
    impl crate::value::ToValue for EFSFileSystemConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FileSystemId".to_string(),
                crate::value::ToValue::to_value(&self.file_system_id),
            );
            if let Some(ref value) = self.file_system_path {
                properties.insert(
                    "FileSystemPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-fsxlustrefilesystemconfig.html
    pub struct FSxLustreFileSystemConfig_ {
        pub file_system_id: crate::value::ExpString,
        pub file_system_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_FSxLustreFileSystemConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.FSxLustreFileSystemConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_FSxLustreFileSystemConfig as FSxLustreFileSystemConfig;
    impl crate::value::ToValue for FSxLustreFileSystemConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FileSystemId".to_string(),
                crate::value::ToValue::to_value(&self.file_system_id),
            );
            if let Some(ref value) = self.file_system_path {
                properties.insert(
                    "FileSystemPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-hiddensagemakerimage.html
    pub struct HiddenSageMakerImage_ {
        pub sage_maker_image_name: Option<crate::value::ExpString>,
        pub version_aliases: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_HiddenSageMakerImage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.HiddenSageMakerImage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_HiddenSageMakerImage as HiddenSageMakerImage;
    impl crate::value::ToValue for HiddenSageMakerImage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sage_maker_image_name {
                properties.insert(
                    "SageMakerImageName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.version_aliases {
                properties.insert(
                    "VersionAliases".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-idlesettings.html
    pub struct IdleSettings_ {
        pub idle_timeout_in_minutes: Option<i64>,
        pub lifecycle_management: Option<crate::value::ExpString>,
        pub max_idle_timeout_in_minutes: Option<i64>,
        pub min_idle_timeout_in_minutes: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_IdleSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.IdleSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_IdleSettings as IdleSettings;
    impl crate::value::ToValue for IdleSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idle_timeout_in_minutes {
                properties.insert(
                    "IdleTimeoutInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_management {
                properties.insert(
                    "LifecycleManagement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_idle_timeout_in_minutes {
                properties.insert(
                    "MaxIdleTimeoutInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_idle_timeout_in_minutes {
                properties.insert(
                    "MinIdleTimeoutInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-jupyterlabappsettings.html
    pub struct JupyterLabAppSettings_ {
        pub app_lifecycle_management: Option<Box<AppLifecycleManagement_>>,
        pub built_in_lifecycle_config_arn: Option<crate::value::ExpString>,
        pub code_repositories: Option<Vec<CodeRepository_>>,
        pub custom_images: Option<Vec<CustomImage_>>,
        pub default_resource_spec: Option<Box<ResourceSpec_>>,
        pub lifecycle_config_arns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_JupyterLabAppSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.JupyterLabAppSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_JupyterLabAppSettings as JupyterLabAppSettings;
    impl crate::value::ToValue for JupyterLabAppSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_lifecycle_management {
                properties.insert(
                    "AppLifecycleManagement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.built_in_lifecycle_config_arn {
                properties.insert(
                    "BuiltInLifecycleConfigArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.code_repositories {
                properties.insert(
                    "CodeRepositories".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_images {
                properties.insert(
                    "CustomImages".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_resource_spec {
                properties.insert(
                    "DefaultResourceSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_config_arns {
                properties.insert(
                    "LifecycleConfigArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-jupyterserverappsettings.html
    pub struct JupyterServerAppSettings_ {
        pub default_resource_spec: Option<Box<ResourceSpec_>>,
        pub lifecycle_config_arns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_JupyterServerAppSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.JupyterServerAppSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_JupyterServerAppSettings as JupyterServerAppSettings;
    impl crate::value::ToValue for JupyterServerAppSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_resource_spec {
                properties.insert(
                    "DefaultResourceSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_config_arns {
                properties.insert(
                    "LifecycleConfigArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-kernelgatewayappsettings.html
    pub struct KernelGatewayAppSettings_ {
        pub custom_images: Option<Vec<CustomImage_>>,
        pub default_resource_spec: Option<Box<ResourceSpec_>>,
        pub lifecycle_config_arns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_KernelGatewayAppSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.KernelGatewayAppSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_KernelGatewayAppSettings as KernelGatewayAppSettings;
    impl crate::value::ToValue for KernelGatewayAppSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_images {
                properties.insert(
                    "CustomImages".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_resource_spec {
                properties.insert(
                    "DefaultResourceSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_config_arns {
                properties.insert(
                    "LifecycleConfigArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-rsessionappsettings.html
    pub struct RSessionAppSettings_ {
        pub custom_images: Option<Vec<CustomImage_>>,
        pub default_resource_spec: Option<Box<ResourceSpec_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_RSessionAppSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.RSessionAppSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_RSessionAppSettings as RSessionAppSettings;
    impl crate::value::ToValue for RSessionAppSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_images {
                properties.insert(
                    "CustomImages".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_resource_spec {
                properties.insert(
                    "DefaultResourceSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-rstudioserverproappsettings.html
    pub struct RStudioServerProAppSettings_ {
        pub access_status: Option<crate::value::ExpString>,
        pub user_group: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_RStudioServerProAppSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.RStudioServerProAppSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_RStudioServerProAppSettings as RStudioServerProAppSettings;
    impl crate::value::ToValue for RStudioServerProAppSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_status {
                properties.insert(
                    "AccessStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_group {
                properties.insert(
                    "UserGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-rstudioserverprodomainsettings.html
    pub struct RStudioServerProDomainSettings_ {
        pub default_resource_spec: Option<Box<ResourceSpec_>>,
        pub domain_execution_role_arn: crate::value::ExpString,
        pub r_studio_connect_url: Option<crate::value::ExpString>,
        pub r_studio_package_manager_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_RStudioServerProDomainSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.RStudioServerProDomainSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_RStudioServerProDomainSettings as RStudioServerProDomainSettings;
    impl crate::value::ToValue for RStudioServerProDomainSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_resource_spec {
                properties.insert(
                    "DefaultResourceSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DomainExecutionRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.domain_execution_role_arn),
            );
            if let Some(ref value) = self.r_studio_connect_url {
                properties.insert(
                    "RStudioConnectUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r_studio_package_manager_url {
                properties.insert(
                    "RStudioPackageManagerUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-resourcespec.html
    pub struct ResourceSpec_ {
        pub instance_type: Option<crate::value::ExpString>,
        pub lifecycle_config_arn: Option<crate::value::ExpString>,
        pub sage_maker_image_arn: Option<crate::value::ExpString>,
        pub sage_maker_image_version_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_ResourceSpec {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.ResourceSpec"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_ResourceSpec as ResourceSpec;
    impl crate::value::ToValue for ResourceSpec_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instance_type {
                properties.insert(
                    "InstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_config_arn {
                properties.insert(
                    "LifecycleConfigArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sage_maker_image_arn {
                properties.insert(
                    "SageMakerImageArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sage_maker_image_version_arn {
                properties.insert(
                    "SageMakerImageVersionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-s3filesystemconfig.html
    pub struct S3FileSystemConfig_ {
        pub mount_path: Option<crate::value::ExpString>,
        pub s3_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_S3FileSystemConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.S3FileSystemConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_S3FileSystemConfig as S3FileSystemConfig;
    impl crate::value::ToValue for S3FileSystemConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mount_path {
                properties.insert(
                    "MountPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_uri {
                properties.insert("S3Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-sharingsettings.html
    pub struct SharingSettings_ {
        pub notebook_output_option: Option<crate::value::ExpString>,
        pub s3_kms_key_id: Option<crate::value::ExpString>,
        pub s3_output_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_SharingSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.SharingSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_SharingSettings as SharingSettings;
    impl crate::value::ToValue for SharingSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.notebook_output_option {
                properties.insert(
                    "NotebookOutputOption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_kms_key_id {
                properties.insert(
                    "S3KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_output_path {
                properties.insert(
                    "S3OutputPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-studiowebportalsettings.html
    pub struct StudioWebPortalSettings_ {
        pub hidden_app_types: Option<Vec<crate::value::ExpString>>,
        pub hidden_instance_types: Option<Vec<crate::value::ExpString>>,
        pub hidden_ml_tools: Option<Vec<crate::value::ExpString>>,
        pub hidden_sage_maker_image_version_aliases: Option<Vec<HiddenSageMakerImage_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_StudioWebPortalSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.StudioWebPortalSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_StudioWebPortalSettings as StudioWebPortalSettings;
    impl crate::value::ToValue for StudioWebPortalSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hidden_app_types {
                properties.insert(
                    "HiddenAppTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hidden_instance_types {
                properties.insert(
                    "HiddenInstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hidden_ml_tools {
                properties.insert(
                    "HiddenMlTools".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hidden_sage_maker_image_version_aliases {
                properties.insert(
                    "HiddenSageMakerImageVersionAliases".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-unifiedstudiosettings.html
    pub struct UnifiedStudioSettings_ {
        pub domain_account_id: Option<crate::value::ExpString>,
        pub domain_id: Option<crate::value::ExpString>,
        pub domain_region: Option<crate::value::ExpString>,
        pub environment_id: Option<crate::value::ExpString>,
        pub project_id: Option<crate::value::ExpString>,
        pub project_s3_path: Option<crate::value::ExpString>,
        pub studio_web_portal_access: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_UnifiedStudioSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.UnifiedStudioSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_UnifiedStudioSettings as UnifiedStudioSettings;
    impl crate::value::ToValue for UnifiedStudioSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.domain_account_id {
                properties.insert(
                    "DomainAccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.domain_id {
                properties.insert(
                    "DomainId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.domain_region {
                properties.insert(
                    "DomainRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment_id {
                properties.insert(
                    "EnvironmentId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.project_id {
                properties.insert(
                    "ProjectId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.project_s3_path {
                properties.insert(
                    "ProjectS3Path".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.studio_web_portal_access {
                properties.insert(
                    "StudioWebPortalAccess".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-usersettings.html
    pub struct UserSettings_ {
        pub auto_mount_home_efs: Option<crate::value::ExpString>,
        pub code_editor_app_settings: Option<Box<CodeEditorAppSettings_>>,
        pub custom_file_system_configs: Option<Vec<CustomFileSystemConfig_>>,
        pub custom_posix_user_config: Option<Box<CustomPosixUserConfig_>>,
        pub default_landing_uri: Option<crate::value::ExpString>,
        pub execution_role: crate::value::ExpString,
        pub jupyter_lab_app_settings: Option<Box<JupyterLabAppSettings_>>,
        pub jupyter_server_app_settings: Option<Box<JupyterServerAppSettings_>>,
        pub kernel_gateway_app_settings: Option<Box<KernelGatewayAppSettings_>>,
        pub r_session_app_settings: Option<Box<RSessionAppSettings_>>,
        pub r_studio_server_pro_app_settings: Option<Box<RStudioServerProAppSettings_>>,
        pub security_groups: Option<Vec<crate::value::ExpString>>,
        pub sharing_settings: Option<Box<SharingSettings_>>,
        pub space_storage_settings: Option<Box<DefaultSpaceStorageSettings_>>,
        pub studio_web_portal: Option<crate::value::ExpString>,
        pub studio_web_portal_settings: Option<Box<StudioWebPortalSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Domain_UserSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Domain.UserSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Domain_UserSettings as UserSettings;
    impl crate::value::ToValue for UserSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_mount_home_efs {
                properties.insert(
                    "AutoMountHomeEFS".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.code_editor_app_settings {
                properties.insert(
                    "CodeEditorAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_file_system_configs {
                properties.insert(
                    "CustomFileSystemConfigs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_posix_user_config {
                properties.insert(
                    "CustomPosixUserConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_landing_uri {
                properties.insert(
                    "DefaultLandingUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ExecutionRole".to_string(),
                crate::value::ToValue::to_value(&self.execution_role),
            );
            if let Some(ref value) = self.jupyter_lab_app_settings {
                properties.insert(
                    "JupyterLabAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.jupyter_server_app_settings {
                properties.insert(
                    "JupyterServerAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kernel_gateway_app_settings {
                properties.insert(
                    "KernelGatewayAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r_session_app_settings {
                properties.insert(
                    "RSessionAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r_studio_server_pro_app_settings {
                properties.insert(
                    "RStudioServerProAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_groups {
                properties.insert(
                    "SecurityGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sharing_settings {
                properties.insert(
                    "SharingSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.space_storage_settings {
                properties.insert(
                    "SpaceStorageSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.studio_web_portal {
                properties.insert(
                    "StudioWebPortal".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.studio_web_portal_settings {
                properties.insert(
                    "StudioWebPortalSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod endpoint {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-alarm.html
    pub struct Alarm_ {
        pub alarm_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Endpoint_Alarm {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Endpoint.Alarm"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Endpoint_Alarm as Alarm;
    impl crate::value::ToValue for Alarm_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AlarmName".to_string(),
                crate::value::ToValue::to_value(&self.alarm_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-autorollbackconfig.html
    pub struct AutoRollbackConfig_ {
        pub alarms: Vec<Alarm_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Endpoint_AutoRollbackConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Endpoint.AutoRollbackConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Endpoint_AutoRollbackConfig as AutoRollbackConfig;
    impl crate::value::ToValue for AutoRollbackConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Alarms".to_string(),
                crate::value::ToValue::to_value(&self.alarms),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-bluegreenupdatepolicy.html
    pub struct BlueGreenUpdatePolicy_ {
        pub maximum_execution_timeout_in_seconds: Option<i64>,
        pub termination_wait_in_seconds: Option<i64>,
        pub traffic_routing_configuration: Box<TrafficRoutingConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Endpoint_BlueGreenUpdatePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Endpoint.BlueGreenUpdatePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Endpoint_BlueGreenUpdatePolicy as BlueGreenUpdatePolicy;
    impl crate::value::ToValue for BlueGreenUpdatePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maximum_execution_timeout_in_seconds {
                properties.insert(
                    "MaximumExecutionTimeoutInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.termination_wait_in_seconds {
                properties.insert(
                    "TerminationWaitInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TrafficRoutingConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.traffic_routing_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-capacitysize.html
    pub struct CapacitySize_ {
        pub r#type: crate::value::ExpString,
        pub value: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Endpoint_CapacitySize {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Endpoint.CapacitySize"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Endpoint_CapacitySize as CapacitySize;
    impl crate::value::ToValue for CapacitySize_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-deploymentconfig.html
    pub struct DeploymentConfig_ {
        pub auto_rollback_configuration: Option<Box<AutoRollbackConfig_>>,
        pub blue_green_update_policy: Option<Box<BlueGreenUpdatePolicy_>>,
        pub rolling_update_policy: Option<Box<RollingUpdatePolicy_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Endpoint_DeploymentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Endpoint.DeploymentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Endpoint_DeploymentConfig as DeploymentConfig;
    impl crate::value::ToValue for DeploymentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_rollback_configuration {
                properties.insert(
                    "AutoRollbackConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.blue_green_update_policy {
                properties.insert(
                    "BlueGreenUpdatePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rolling_update_policy {
                properties.insert(
                    "RollingUpdatePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-rollingupdatepolicy.html
    pub struct RollingUpdatePolicy_ {
        pub maximum_batch_size: Box<CapacitySize_>,
        pub maximum_execution_timeout_in_seconds: Option<i64>,
        pub rollback_maximum_batch_size: Option<Box<CapacitySize_>>,
        pub wait_interval_in_seconds: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Endpoint_RollingUpdatePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Endpoint.RollingUpdatePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Endpoint_RollingUpdatePolicy as RollingUpdatePolicy;
    impl crate::value::ToValue for RollingUpdatePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaximumBatchSize".to_string(),
                crate::value::ToValue::to_value(&self.maximum_batch_size),
            );
            if let Some(ref value) = self.maximum_execution_timeout_in_seconds {
                properties.insert(
                    "MaximumExecutionTimeoutInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rollback_maximum_batch_size {
                properties.insert(
                    "RollbackMaximumBatchSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "WaitIntervalInSeconds".to_string(),
                crate::value::ToValue::to_value(&self.wait_interval_in_seconds),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-trafficroutingconfig.html
    pub struct TrafficRoutingConfig_ {
        pub canary_size: Option<Box<CapacitySize_>>,
        pub linear_step_size: Option<Box<CapacitySize_>>,
        pub r#type: crate::value::ExpString,
        pub wait_interval_in_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Endpoint_TrafficRoutingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Endpoint.TrafficRoutingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Endpoint_TrafficRoutingConfig as TrafficRoutingConfig;
    impl crate::value::ToValue for TrafficRoutingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.canary_size {
                properties.insert(
                    "CanarySize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.linear_step_size {
                properties.insert(
                    "LinearStepSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.wait_interval_in_seconds {
                properties.insert(
                    "WaitIntervalInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-variantproperty.html
    pub struct VariantProperty_ {
        pub variant_property_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Endpoint_VariantProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Endpoint.VariantProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Endpoint_VariantProperty as VariantProperty;
    impl crate::value::ToValue for VariantProperty_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.variant_property_type {
                properties.insert(
                    "VariantPropertyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod endpointconfig {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-asyncinferenceclientconfig.html
    pub struct AsyncInferenceClientConfig_ {
        pub max_concurrent_invocations_per_instance: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_AsyncInferenceClientConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.AsyncInferenceClientConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_AsyncInferenceClientConfig as AsyncInferenceClientConfig;
    impl crate::value::ToValue for AsyncInferenceClientConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_concurrent_invocations_per_instance {
                properties.insert(
                    "MaxConcurrentInvocationsPerInstance".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-asyncinferenceconfig.html
    pub struct AsyncInferenceConfig_ {
        pub client_config: Option<Box<AsyncInferenceClientConfig_>>,
        pub output_config: Box<AsyncInferenceOutputConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_AsyncInferenceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.AsyncInferenceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_AsyncInferenceConfig as AsyncInferenceConfig;
    impl crate::value::ToValue for AsyncInferenceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.client_config {
                properties.insert(
                    "ClientConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OutputConfig".to_string(),
                crate::value::ToValue::to_value(&self.output_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-asyncinferencenotificationconfig.html
    pub struct AsyncInferenceNotificationConfig_ {
        pub error_topic: Option<crate::value::ExpString>,
        pub include_inference_response_in: Option<Vec<crate::value::ExpString>>,
        pub success_topic: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_AsyncInferenceNotificationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.AsyncInferenceNotificationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_AsyncInferenceNotificationConfig as AsyncInferenceNotificationConfig;
    impl crate::value::ToValue for AsyncInferenceNotificationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.error_topic {
                properties.insert(
                    "ErrorTopic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_inference_response_in {
                properties.insert(
                    "IncludeInferenceResponseIn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.success_topic {
                properties.insert(
                    "SuccessTopic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-asyncinferenceoutputconfig.html
    pub struct AsyncInferenceOutputConfig_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub notification_config: Option<Box<AsyncInferenceNotificationConfig_>>,
        pub s3_failure_path: Option<crate::value::ExpString>,
        pub s3_output_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_AsyncInferenceOutputConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.AsyncInferenceOutputConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_AsyncInferenceOutputConfig as AsyncInferenceOutputConfig;
    impl crate::value::ToValue for AsyncInferenceOutputConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.notification_config {
                properties.insert(
                    "NotificationConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_failure_path {
                properties.insert(
                    "S3FailurePath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_output_path {
                properties.insert(
                    "S3OutputPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-capacityreservationconfig.html
    pub struct CapacityReservationConfig_ {
        pub capacity_reservation_preference: Option<crate::value::ExpString>,
        pub ml_reservation_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_CapacityReservationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.CapacityReservationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_CapacityReservationConfig as CapacityReservationConfig;
    impl crate::value::ToValue for CapacityReservationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity_reservation_preference {
                properties.insert(
                    "CapacityReservationPreference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ml_reservation_arn {
                properties.insert(
                    "MlReservationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-datacaptureconfig-capturecontenttypeheader.html
    pub struct CaptureContentTypeHeader_ {
        pub csv_content_types: Option<Vec<crate::value::ExpString>>,
        pub json_content_types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_CaptureContentTypeHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.CaptureContentTypeHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_CaptureContentTypeHeader as CaptureContentTypeHeader;
    impl crate::value::ToValue for CaptureContentTypeHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.csv_content_types {
                properties.insert(
                    "CsvContentTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.json_content_types {
                properties.insert(
                    "JsonContentTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-captureoption.html
    pub struct CaptureOption_ {
        pub capture_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_CaptureOption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.CaptureOption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_CaptureOption as CaptureOption;
    impl crate::value::ToValue for CaptureOption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CaptureMode".to_string(),
                crate::value::ToValue::to_value(&self.capture_mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyexplainerconfig.html
    pub struct ClarifyExplainerConfig_ {
        pub enable_explanations: Option<crate::value::ExpString>,
        pub inference_config: Option<Box<ClarifyInferenceConfig_>>,
        pub shap_config: Box<ClarifyShapConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_ClarifyExplainerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.ClarifyExplainerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_ClarifyExplainerConfig as ClarifyExplainerConfig;
    impl crate::value::ToValue for ClarifyExplainerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_explanations {
                properties.insert(
                    "EnableExplanations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_config {
                properties.insert(
                    "InferenceConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ShapConfig".to_string(),
                crate::value::ToValue::to_value(&self.shap_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyfeaturetype.html
    pub struct ClarifyFeatureType_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_ClarifyFeatureType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.ClarifyFeatureType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_ClarifyFeatureType as ClarifyFeatureType;
    impl crate::value::ToValue for ClarifyFeatureType_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyheader.html
    pub struct ClarifyHeader_ {}
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_ClarifyHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.ClarifyHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_ClarifyHeader as ClarifyHeader;
    impl crate::value::ToValue for ClarifyHeader_ {
        fn to_value(&self) -> serde_json::Value {
            serde_json::Value::Object(serde_json::Map::new())
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyinferenceconfig.html
    pub struct ClarifyInferenceConfig_ {
        pub content_template: Option<crate::value::ExpString>,
        pub feature_headers: Option<Vec<ClarifyHeader_>>,
        pub feature_types: Option<Vec<ClarifyFeatureType_>>,
        pub features_attribute: Option<crate::value::ExpString>,
        pub label_attribute: Option<crate::value::ExpString>,
        pub label_headers: Option<Vec<ClarifyHeader_>>,
        pub label_index: Option<i64>,
        pub max_payload_in_mb: Option<i64>,
        pub max_record_count: Option<i64>,
        pub probability_attribute: Option<crate::value::ExpString>,
        pub probability_index: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_ClarifyInferenceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.ClarifyInferenceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_ClarifyInferenceConfig as ClarifyInferenceConfig;
    impl crate::value::ToValue for ClarifyInferenceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content_template {
                properties.insert(
                    "ContentTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.feature_headers {
                properties.insert(
                    "FeatureHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.feature_types {
                properties.insert(
                    "FeatureTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.features_attribute {
                properties.insert(
                    "FeaturesAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.label_attribute {
                properties.insert(
                    "LabelAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.label_headers {
                properties.insert(
                    "LabelHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.label_index {
                properties.insert(
                    "LabelIndex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_payload_in_mb {
                properties.insert(
                    "MaxPayloadInMB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_record_count {
                properties.insert(
                    "MaxRecordCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.probability_attribute {
                properties.insert(
                    "ProbabilityAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.probability_index {
                properties.insert(
                    "ProbabilityIndex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyshapbaselineconfig.html
    pub struct ClarifyShapBaselineConfig_ {
        pub mime_type: Option<crate::value::ExpString>,
        pub shap_baseline: Option<crate::value::ExpString>,
        pub shap_baseline_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_ClarifyShapBaselineConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.ClarifyShapBaselineConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_ClarifyShapBaselineConfig as ClarifyShapBaselineConfig;
    impl crate::value::ToValue for ClarifyShapBaselineConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mime_type {
                properties.insert(
                    "MimeType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shap_baseline {
                properties.insert(
                    "ShapBaseline".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shap_baseline_uri {
                properties.insert(
                    "ShapBaselineUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyshapconfig.html
    pub struct ClarifyShapConfig_ {
        pub number_of_samples: Option<i64>,
        pub seed: Option<i64>,
        pub shap_baseline_config: Box<ClarifyShapBaselineConfig_>,
        pub text_config: Option<Box<ClarifyTextConfig_>>,
        pub use_logit: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_ClarifyShapConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.ClarifyShapConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_ClarifyShapConfig as ClarifyShapConfig;
    impl crate::value::ToValue for ClarifyShapConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.number_of_samples {
                properties.insert(
                    "NumberOfSamples".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.seed {
                properties.insert("Seed".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "ShapBaselineConfig".to_string(),
                crate::value::ToValue::to_value(&self.shap_baseline_config),
            );
            if let Some(ref value) = self.text_config {
                properties.insert(
                    "TextConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_logit {
                properties.insert(
                    "UseLogit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifytextconfig.html
    pub struct ClarifyTextConfig_ {
        pub granularity: crate::value::ExpString,
        pub language: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_ClarifyTextConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.ClarifyTextConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_ClarifyTextConfig as ClarifyTextConfig;
    impl crate::value::ToValue for ClarifyTextConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Granularity".to_string(),
                crate::value::ToValue::to_value(&self.granularity),
            );
            properties.insert(
                "Language".to_string(),
                crate::value::ToValue::to_value(&self.language),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-datacaptureconfig.html
    pub struct DataCaptureConfig_ {
        pub capture_content_type_header: Option<Box<CaptureContentTypeHeader_>>,
        pub capture_options: Vec<CaptureOption_>,
        pub destination_s3_uri: crate::value::ExpString,
        pub enable_capture: Option<crate::value::ExpBool>,
        pub initial_sampling_percentage: i64,
        pub kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_DataCaptureConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.DataCaptureConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_DataCaptureConfig as DataCaptureConfig;
    impl crate::value::ToValue for DataCaptureConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capture_content_type_header {
                properties.insert(
                    "CaptureContentTypeHeader".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "CaptureOptions".to_string(),
                crate::value::ToValue::to_value(&self.capture_options),
            );
            properties.insert(
                "DestinationS3Uri".to_string(),
                crate::value::ToValue::to_value(&self.destination_s3_uri),
            );
            if let Some(ref value) = self.enable_capture {
                properties.insert(
                    "EnableCapture".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InitialSamplingPercentage".to_string(),
                crate::value::ToValue::to_value(&self.initial_sampling_percentage),
            );
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-explainerconfig.html
    pub struct ExplainerConfig_ {
        pub clarify_explainer_config: Option<Box<ClarifyExplainerConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_ExplainerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.ExplainerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_ExplainerConfig as ExplainerConfig;
    impl crate::value::ToValue for ExplainerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.clarify_explainer_config {
                properties.insert(
                    "ClarifyExplainerConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant-managedinstancescaling.html
    pub struct ManagedInstanceScaling_ {
        pub max_instance_count: Option<i64>,
        pub min_instance_count: Option<i64>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_ManagedInstanceScaling {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.ManagedInstanceScaling"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_ManagedInstanceScaling as ManagedInstanceScaling;
    impl crate::value::ToValue for ManagedInstanceScaling_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_instance_count {
                properties.insert(
                    "MaxInstanceCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_instance_count {
                properties.insert(
                    "MinInstanceCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant.html
    pub struct ProductionVariant_ {
        pub capacity_reservation_config: Option<Box<CapacityReservationConfig_>>,
        pub container_startup_health_check_timeout_in_seconds: Option<i64>,
        pub enable_ssm_access: Option<crate::value::ExpBool>,
        pub inference_ami_version: Option<crate::value::ExpString>,
        pub initial_instance_count: Option<i64>,
        pub initial_variant_weight: Option<f64>,
        pub instance_type: Option<crate::value::ExpString>,
        pub managed_instance_scaling: Option<Box<ManagedInstanceScaling_>>,
        pub model_data_download_timeout_in_seconds: Option<i64>,
        pub model_name: Option<crate::value::ExpString>,
        pub routing_config: Option<Box<RoutingConfig_>>,
        pub serverless_config: Option<Box<ServerlessConfig_>>,
        pub variant_name: crate::value::ExpString,
        pub volume_size_in_gb: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_ProductionVariant {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.ProductionVariant"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_ProductionVariant as ProductionVariant;
    impl crate::value::ToValue for ProductionVariant_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity_reservation_config {
                properties.insert(
                    "CapacityReservationConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_startup_health_check_timeout_in_seconds {
                properties.insert(
                    "ContainerStartupHealthCheckTimeoutInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_ssm_access {
                properties.insert(
                    "EnableSSMAccess".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_ami_version {
                properties.insert(
                    "InferenceAmiVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.initial_instance_count {
                properties.insert(
                    "InitialInstanceCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.initial_variant_weight {
                properties.insert(
                    "InitialVariantWeight".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_type {
                properties.insert(
                    "InstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.managed_instance_scaling {
                properties.insert(
                    "ManagedInstanceScaling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_data_download_timeout_in_seconds {
                properties.insert(
                    "ModelDataDownloadTimeoutInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_name {
                properties.insert(
                    "ModelName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.routing_config {
                properties.insert(
                    "RoutingConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.serverless_config {
                properties.insert(
                    "ServerlessConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VariantName".to_string(),
                crate::value::ToValue::to_value(&self.variant_name),
            );
            if let Some(ref value) = self.volume_size_in_gb {
                properties.insert(
                    "VolumeSizeInGB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant-routingconfig.html
    pub struct RoutingConfig_ {
        pub routing_strategy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_RoutingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.RoutingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_RoutingConfig as RoutingConfig;
    impl crate::value::ToValue for RoutingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.routing_strategy {
                properties.insert(
                    "RoutingStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant-serverlessconfig.html
    pub struct ServerlessConfig_ {
        pub max_concurrency: i64,
        pub memory_size_in_mb: i64,
        pub provisioned_concurrency: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_ServerlessConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.ServerlessConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_ServerlessConfig as ServerlessConfig;
    impl crate::value::ToValue for ServerlessConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxConcurrency".to_string(),
                crate::value::ToValue::to_value(&self.max_concurrency),
            );
            properties.insert(
                "MemorySizeInMB".to_string(),
                crate::value::ToValue::to_value(&self.memory_size_in_mb),
            );
            if let Some(ref value) = self.provisioned_concurrency {
                properties.insert(
                    "ProvisionedConcurrency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-vpcconfig.html
    pub struct VpcConfig_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_EndpointConfig_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::EndpointConfig.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_EndpointConfig_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
}
pub mod featuregroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-featuregroup-datacatalogconfig.html
    pub struct DataCatalogConfig_ {
        pub catalog: crate::value::ExpString,
        pub database: crate::value::ExpString,
        pub table_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_FeatureGroup_DataCatalogConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::FeatureGroup.DataCatalogConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_FeatureGroup_DataCatalogConfig as DataCatalogConfig;
    impl crate::value::ToValue for DataCatalogConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Catalog".to_string(),
                crate::value::ToValue::to_value(&self.catalog),
            );
            properties.insert(
                "Database".to_string(),
                crate::value::ToValue::to_value(&self.database),
            );
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-featuregroup-featuredefinition.html
    pub struct FeatureDefinition_ {
        pub feature_name: crate::value::ExpString,
        pub feature_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_FeatureGroup_FeatureDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::FeatureGroup.FeatureDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_FeatureGroup_FeatureDefinition as FeatureDefinition;
    impl crate::value::ToValue for FeatureDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FeatureName".to_string(),
                crate::value::ToValue::to_value(&self.feature_name),
            );
            properties.insert(
                "FeatureType".to_string(),
                crate::value::ToValue::to_value(&self.feature_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-featuregroup-offlinestoreconfig.html
    pub struct OfflineStoreConfig_ {
        pub data_catalog_config: Option<Box<DataCatalogConfig_>>,
        pub disable_glue_table_creation: Option<crate::value::ExpBool>,
        pub s3_storage_config: Box<S3StorageConfig_>,
        pub table_format: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_FeatureGroup_OfflineStoreConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::FeatureGroup.OfflineStoreConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_FeatureGroup_OfflineStoreConfig as OfflineStoreConfig;
    impl crate::value::ToValue for OfflineStoreConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_catalog_config {
                properties.insert(
                    "DataCatalogConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.disable_glue_table_creation {
                properties.insert(
                    "DisableGlueTableCreation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3StorageConfig".to_string(),
                crate::value::ToValue::to_value(&self.s3_storage_config),
            );
            if let Some(ref value) = self.table_format {
                properties.insert(
                    "TableFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-featuregroup-onlinestoreconfig.html
    pub struct OnlineStoreConfig_ {
        pub enable_online_store: Option<crate::value::ExpBool>,
        pub security_config: Option<Box<OnlineStoreSecurityConfig_>>,
        pub storage_type: Option<crate::value::ExpString>,
        pub ttl_duration: Option<Box<TtlDuration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_FeatureGroup_OnlineStoreConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::FeatureGroup.OnlineStoreConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_FeatureGroup_OnlineStoreConfig as OnlineStoreConfig;
    impl crate::value::ToValue for OnlineStoreConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_online_store {
                properties.insert(
                    "EnableOnlineStore".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_config {
                properties.insert(
                    "SecurityConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.storage_type {
                properties.insert(
                    "StorageType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ttl_duration {
                properties.insert(
                    "TtlDuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-featuregroup-onlinestoresecurityconfig.html
    pub struct OnlineStoreSecurityConfig_ {
        pub kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_FeatureGroup_OnlineStoreSecurityConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::FeatureGroup.OnlineStoreSecurityConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_FeatureGroup_OnlineStoreSecurityConfig as OnlineStoreSecurityConfig;
    impl crate::value::ToValue for OnlineStoreSecurityConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-featuregroup-s3storageconfig.html
    pub struct S3StorageConfig_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_FeatureGroup_S3StorageConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::FeatureGroup.S3StorageConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_FeatureGroup_S3StorageConfig as S3StorageConfig;
    impl crate::value::ToValue for S3StorageConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-featuregroup-throughputconfig.html
    pub struct ThroughputConfig_ {
        pub provisioned_read_capacity_units: Option<i64>,
        pub provisioned_write_capacity_units: Option<i64>,
        pub throughput_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_FeatureGroup_ThroughputConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::FeatureGroup.ThroughputConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_FeatureGroup_ThroughputConfig as ThroughputConfig;
    impl crate::value::ToValue for ThroughputConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.provisioned_read_capacity_units {
                properties.insert(
                    "ProvisionedReadCapacityUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.provisioned_write_capacity_units {
                properties.insert(
                    "ProvisionedWriteCapacityUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ThroughputMode".to_string(),
                crate::value::ToValue::to_value(&self.throughput_mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-featuregroup-ttlduration.html
    pub struct TtlDuration_ {
        pub unit: Option<crate::value::ExpString>,
        pub value: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_FeatureGroup_TtlDuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::FeatureGroup.TtlDuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_FeatureGroup_TtlDuration as TtlDuration;
    impl crate::value::ToValue for TtlDuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod inferencecomponent {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferencecomponent-alarm.html
    pub struct Alarm_ {
        pub alarm_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceComponent_Alarm {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceComponent.Alarm"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceComponent_Alarm as Alarm;
    impl crate::value::ToValue for Alarm_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AlarmName".to_string(),
                crate::value::ToValue::to_value(&self.alarm_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferencecomponent-autorollbackconfiguration.html
    pub struct AutoRollbackConfiguration_ {
        pub alarms: Vec<Alarm_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceComponent_AutoRollbackConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceComponent.AutoRollbackConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceComponent_AutoRollbackConfiguration as AutoRollbackConfiguration;
    impl crate::value::ToValue for AutoRollbackConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Alarms".to_string(),
                crate::value::ToValue::to_value(&self.alarms),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferencecomponent-deployedimage.html
    pub struct DeployedImage_ {
        pub resolution_time: Option<crate::value::ExpString>,
        pub resolved_image: Option<crate::value::ExpString>,
        pub specified_image: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceComponent_DeployedImage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceComponent.DeployedImage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceComponent_DeployedImage as DeployedImage;
    impl crate::value::ToValue for DeployedImage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.resolution_time {
                properties.insert(
                    "ResolutionTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resolved_image {
                properties.insert(
                    "ResolvedImage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.specified_image {
                properties.insert(
                    "SpecifiedImage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferencecomponent-inferencecomponentcapacitysize.html
    pub struct InferenceComponentCapacitySize_ {
        pub r#type: crate::value::ExpString,
        pub value: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceComponent_InferenceComponentCapacitySize {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceComponent.InferenceComponentCapacitySize"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceComponent_InferenceComponentCapacitySize as InferenceComponentCapacitySize;
    impl crate::value::ToValue for InferenceComponentCapacitySize_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferencecomponent-inferencecomponentcomputeresourcerequirements.html
    pub struct InferenceComponentComputeResourceRequirements_ {
        pub max_memory_required_in_mb: Option<i64>,
        pub min_memory_required_in_mb: Option<i64>,
        pub number_of_accelerator_devices_required: Option<f64>,
        pub number_of_cpu_cores_required: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceComponent_InferenceComponentComputeResourceRequirements {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceComponent.InferenceComponentComputeResourceRequirements"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceComponent_InferenceComponentComputeResourceRequirements as InferenceComponentComputeResourceRequirements;
    impl crate::value::ToValue for InferenceComponentComputeResourceRequirements_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_memory_required_in_mb {
                properties.insert(
                    "MaxMemoryRequiredInMb".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_memory_required_in_mb {
                properties.insert(
                    "MinMemoryRequiredInMb".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_of_accelerator_devices_required {
                properties.insert(
                    "NumberOfAcceleratorDevicesRequired".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_of_cpu_cores_required {
                properties.insert(
                    "NumberOfCpuCoresRequired".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferencecomponent-inferencecomponentcontainerspecification.html
    pub struct InferenceComponentContainerSpecification_ {
        pub artifact_url: Option<crate::value::ExpString>,
        pub deployed_image: Option<Box<DeployedImage_>>,
        pub environment: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub image: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceComponent_InferenceComponentContainerSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceComponent.InferenceComponentContainerSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceComponent_InferenceComponentContainerSpecification as InferenceComponentContainerSpecification;
    impl crate::value::ToValue for InferenceComponentContainerSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.artifact_url {
                properties.insert(
                    "ArtifactUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.deployed_image {
                properties.insert(
                    "DeployedImage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image {
                properties.insert("Image".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferencecomponent-inferencecomponentdeploymentconfig.html
    pub struct InferenceComponentDeploymentConfig_ {
        pub auto_rollback_configuration: Option<Box<AutoRollbackConfiguration_>>,
        pub rolling_update_policy: Option<Box<InferenceComponentRollingUpdatePolicy_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceComponent_InferenceComponentDeploymentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceComponent.InferenceComponentDeploymentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceComponent_InferenceComponentDeploymentConfig as InferenceComponentDeploymentConfig;
    impl crate::value::ToValue for InferenceComponentDeploymentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_rollback_configuration {
                properties.insert(
                    "AutoRollbackConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rolling_update_policy {
                properties.insert(
                    "RollingUpdatePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferencecomponent-inferencecomponentrollingupdatepolicy.html
    pub struct InferenceComponentRollingUpdatePolicy_ {
        pub maximum_batch_size: Option<Box<InferenceComponentCapacitySize_>>,
        pub maximum_execution_timeout_in_seconds: Option<i64>,
        pub rollback_maximum_batch_size: Option<Box<InferenceComponentCapacitySize_>>,
        pub wait_interval_in_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceComponent_InferenceComponentRollingUpdatePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceComponent.InferenceComponentRollingUpdatePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceComponent_InferenceComponentRollingUpdatePolicy as InferenceComponentRollingUpdatePolicy;
    impl crate::value::ToValue for InferenceComponentRollingUpdatePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maximum_batch_size {
                properties.insert(
                    "MaximumBatchSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_execution_timeout_in_seconds {
                properties.insert(
                    "MaximumExecutionTimeoutInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rollback_maximum_batch_size {
                properties.insert(
                    "RollbackMaximumBatchSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.wait_interval_in_seconds {
                properties.insert(
                    "WaitIntervalInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferencecomponent-inferencecomponentruntimeconfig.html
    pub struct InferenceComponentRuntimeConfig_ {
        pub copy_count: Option<i64>,
        pub current_copy_count: Option<i64>,
        pub desired_copy_count: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceComponent_InferenceComponentRuntimeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceComponent.InferenceComponentRuntimeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceComponent_InferenceComponentRuntimeConfig as InferenceComponentRuntimeConfig;
    impl crate::value::ToValue for InferenceComponentRuntimeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.copy_count {
                properties.insert(
                    "CopyCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.current_copy_count {
                properties.insert(
                    "CurrentCopyCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.desired_copy_count {
                properties.insert(
                    "DesiredCopyCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferencecomponent-inferencecomponentspecification.html
    pub struct InferenceComponentSpecification_ {
        pub base_inference_component_name: Option<crate::value::ExpString>,
        pub compute_resource_requirements:
            Option<Box<InferenceComponentComputeResourceRequirements_>>,
        pub container: Option<Box<InferenceComponentContainerSpecification_>>,
        pub model_name: Option<crate::value::ExpString>,
        pub startup_parameters: Option<Box<InferenceComponentStartupParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceComponent_InferenceComponentSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceComponent.InferenceComponentSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceComponent_InferenceComponentSpecification as InferenceComponentSpecification;
    impl crate::value::ToValue for InferenceComponentSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.base_inference_component_name {
                properties.insert(
                    "BaseInferenceComponentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compute_resource_requirements {
                properties.insert(
                    "ComputeResourceRequirements".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container {
                properties.insert(
                    "Container".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_name {
                properties.insert(
                    "ModelName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.startup_parameters {
                properties.insert(
                    "StartupParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferencecomponent-inferencecomponentstartupparameters.html
    pub struct InferenceComponentStartupParameters_ {
        pub container_startup_health_check_timeout_in_seconds: Option<i64>,
        pub model_data_download_timeout_in_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceComponent_InferenceComponentStartupParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceComponent.InferenceComponentStartupParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceComponent_InferenceComponentStartupParameters as InferenceComponentStartupParameters;
    impl crate::value::ToValue for InferenceComponentStartupParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_startup_health_check_timeout_in_seconds {
                properties.insert(
                    "ContainerStartupHealthCheckTimeoutInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_data_download_timeout_in_seconds {
                properties.insert(
                    "ModelDataDownloadTimeoutInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod inferenceexperiment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferenceexperiment-capturecontenttypeheader.html
    pub struct CaptureContentTypeHeader_ {
        pub csv_content_types: Option<Vec<crate::value::ExpString>>,
        pub json_content_types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceExperiment_CaptureContentTypeHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceExperiment.CaptureContentTypeHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceExperiment_CaptureContentTypeHeader as CaptureContentTypeHeader;
    impl crate::value::ToValue for CaptureContentTypeHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.csv_content_types {
                properties.insert(
                    "CsvContentTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.json_content_types {
                properties.insert(
                    "JsonContentTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferenceexperiment-datastorageconfig.html
    pub struct DataStorageConfig_ {
        pub content_type: Option<Box<CaptureContentTypeHeader_>>,
        pub destination: crate::value::ExpString,
        pub kms_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceExperiment_DataStorageConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceExperiment.DataStorageConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceExperiment_DataStorageConfig as DataStorageConfig;
    impl crate::value::ToValue for DataStorageConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content_type {
                properties.insert(
                    "ContentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(&self.destination),
            );
            if let Some(ref value) = self.kms_key {
                properties.insert("KmsKey".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferenceexperiment-endpointmetadata.html
    pub struct EndpointMetadata_ {
        pub endpoint_config_name: Option<crate::value::ExpString>,
        pub endpoint_name: crate::value::ExpString,
        pub endpoint_status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceExperiment_EndpointMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceExperiment.EndpointMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceExperiment_EndpointMetadata as EndpointMetadata;
    impl crate::value::ToValue for EndpointMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.endpoint_config_name {
                properties.insert(
                    "EndpointConfigName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EndpointName".to_string(),
                crate::value::ToValue::to_value(&self.endpoint_name),
            );
            if let Some(ref value) = self.endpoint_status {
                properties.insert(
                    "EndpointStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferenceexperiment-inferenceexperimentschedule.html
    pub struct InferenceExperimentSchedule_ {
        pub end_time: Option<crate::value::ExpString>,
        pub start_time: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceExperiment_InferenceExperimentSchedule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceExperiment.InferenceExperimentSchedule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceExperiment_InferenceExperimentSchedule as InferenceExperimentSchedule;
    impl crate::value::ToValue for InferenceExperimentSchedule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.end_time {
                properties.insert(
                    "EndTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_time {
                properties.insert(
                    "StartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferenceexperiment-modelinfrastructureconfig.html
    pub struct ModelInfrastructureConfig_ {
        pub infrastructure_type: crate::value::ExpString,
        pub real_time_inference_config: Box<RealTimeInferenceConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceExperiment_ModelInfrastructureConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceExperiment.ModelInfrastructureConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceExperiment_ModelInfrastructureConfig as ModelInfrastructureConfig;
    impl crate::value::ToValue for ModelInfrastructureConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InfrastructureType".to_string(),
                crate::value::ToValue::to_value(&self.infrastructure_type),
            );
            properties.insert(
                "RealTimeInferenceConfig".to_string(),
                crate::value::ToValue::to_value(&self.real_time_inference_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferenceexperiment-modelvariantconfig.html
    pub struct ModelVariantConfig_ {
        pub infrastructure_config: Box<ModelInfrastructureConfig_>,
        pub model_name: crate::value::ExpString,
        pub variant_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceExperiment_ModelVariantConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceExperiment.ModelVariantConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceExperiment_ModelVariantConfig as ModelVariantConfig;
    impl crate::value::ToValue for ModelVariantConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InfrastructureConfig".to_string(),
                crate::value::ToValue::to_value(&self.infrastructure_config),
            );
            properties.insert(
                "ModelName".to_string(),
                crate::value::ToValue::to_value(&self.model_name),
            );
            properties.insert(
                "VariantName".to_string(),
                crate::value::ToValue::to_value(&self.variant_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferenceexperiment-realtimeinferenceconfig.html
    pub struct RealTimeInferenceConfig_ {
        pub instance_count: i64,
        pub instance_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceExperiment_RealTimeInferenceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceExperiment.RealTimeInferenceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceExperiment_RealTimeInferenceConfig as RealTimeInferenceConfig;
    impl crate::value::ToValue for RealTimeInferenceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceCount".to_string(),
                crate::value::ToValue::to_value(&self.instance_count),
            );
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferenceexperiment-shadowmodeconfig.html
    pub struct ShadowModeConfig_ {
        pub shadow_model_variants: Vec<ShadowModelVariantConfig_>,
        pub source_model_variant_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceExperiment_ShadowModeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceExperiment.ShadowModeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceExperiment_ShadowModeConfig as ShadowModeConfig;
    impl crate::value::ToValue for ShadowModeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ShadowModelVariants".to_string(),
                crate::value::ToValue::to_value(&self.shadow_model_variants),
            );
            properties.insert(
                "SourceModelVariantName".to_string(),
                crate::value::ToValue::to_value(&self.source_model_variant_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-inferenceexperiment-shadowmodelvariantconfig.html
    pub struct ShadowModelVariantConfig_ {
        pub sampling_percentage: i64,
        pub shadow_model_variant_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_InferenceExperiment_ShadowModelVariantConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::InferenceExperiment.ShadowModelVariantConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_InferenceExperiment_ShadowModelVariantConfig as ShadowModelVariantConfig;
    impl crate::value::ToValue for ShadowModelVariantConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SamplingPercentage".to_string(),
                crate::value::ToValue::to_value(&self.sampling_percentage),
            );
            properties.insert(
                "ShadowModelVariantName".to_string(),
                crate::value::ToValue::to_value(&self.shadow_model_variant_name),
            );
            properties.into()
        }
    }
}
pub mod model {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-additionalmodeldatasource.html
    pub struct AdditionalModelDataSource_ {
        pub channel_name: crate::value::ExpString,
        pub s3_data_source: Box<S3DataSource_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Model_AdditionalModelDataSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Model.AdditionalModelDataSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Model_AdditionalModelDataSource as AdditionalModelDataSource;
    impl crate::value::ToValue for AdditionalModelDataSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ChannelName".to_string(),
                crate::value::ToValue::to_value(&self.channel_name),
            );
            properties.insert(
                "S3DataSource".to_string(),
                crate::value::ToValue::to_value(&self.s3_data_source),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition.html
    pub struct ContainerDefinition_ {
        pub container_hostname: Option<crate::value::ExpString>,
        pub environment: Option<serde_json::Value>,
        pub image: Option<crate::value::ExpString>,
        pub image_config: Option<Box<ImageConfig_>>,
        pub inference_specification_name: Option<crate::value::ExpString>,
        pub mode: Option<crate::value::ExpString>,
        pub model_data_source: Option<Box<ModelDataSource_>>,
        pub model_data_url: Option<crate::value::ExpString>,
        pub model_package_name: Option<crate::value::ExpString>,
        pub multi_model_config: Option<Box<MultiModelConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Model_ContainerDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Model.ContainerDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Model_ContainerDefinition as ContainerDefinition;
    impl crate::value::ToValue for ContainerDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_hostname {
                properties.insert(
                    "ContainerHostname".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image {
                properties.insert("Image".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.image_config {
                properties.insert(
                    "ImageConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_specification_name {
                properties.insert(
                    "InferenceSpecificationName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mode {
                properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.model_data_source {
                properties.insert(
                    "ModelDataSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_data_url {
                properties.insert(
                    "ModelDataUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_package_name {
                properties.insert(
                    "ModelPackageName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.multi_model_config {
                properties.insert(
                    "MultiModelConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-s3datasource-hubaccessconfig.html
    pub struct HubAccessConfig_ {
        pub hub_content_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Model_HubAccessConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Model.HubAccessConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Model_HubAccessConfig as HubAccessConfig;
    impl crate::value::ToValue for HubAccessConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HubContentArn".to_string(),
                crate::value::ToValue::to_value(&self.hub_content_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition-imageconfig.html
    pub struct ImageConfig_ {
        pub repository_access_mode: crate::value::ExpString,
        pub repository_auth_config: Option<Box<RepositoryAuthConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Model_ImageConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Model.ImageConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Model_ImageConfig as ImageConfig;
    impl crate::value::ToValue for ImageConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RepositoryAccessMode".to_string(),
                crate::value::ToValue::to_value(&self.repository_access_mode),
            );
            if let Some(ref value) = self.repository_auth_config {
                properties.insert(
                    "RepositoryAuthConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-inferenceexecutionconfig.html
    pub struct InferenceExecutionConfig_ {
        pub mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Model_InferenceExecutionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Model.InferenceExecutionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Model_InferenceExecutionConfig as InferenceExecutionConfig;
    impl crate::value::ToValue for InferenceExecutionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Mode".to_string(),
                crate::value::ToValue::to_value(&self.mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-s3datasource-modelaccessconfig.html
    pub struct ModelAccessConfig_ {
        pub accept_eula: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Model_ModelAccessConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Model.ModelAccessConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Model_ModelAccessConfig as ModelAccessConfig;
    impl crate::value::ToValue for ModelAccessConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AcceptEula".to_string(),
                crate::value::ToValue::to_value(&self.accept_eula),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition-modeldatasource.html
    pub struct ModelDataSource_ {
        pub s3_data_source: Box<S3DataSource_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Model_ModelDataSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Model.ModelDataSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Model_ModelDataSource as ModelDataSource;
    impl crate::value::ToValue for ModelDataSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3DataSource".to_string(),
                crate::value::ToValue::to_value(&self.s3_data_source),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition-multimodelconfig.html
    pub struct MultiModelConfig_ {
        pub model_cache_setting: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Model_MultiModelConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Model.MultiModelConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Model_MultiModelConfig as MultiModelConfig;
    impl crate::value::ToValue for MultiModelConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.model_cache_setting {
                properties.insert(
                    "ModelCacheSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition-imageconfig-repositoryauthconfig.html
    pub struct RepositoryAuthConfig_ {
        pub repository_credentials_provider_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Model_RepositoryAuthConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Model.RepositoryAuthConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Model_RepositoryAuthConfig as RepositoryAuthConfig;
    impl crate::value::ToValue for RepositoryAuthConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RepositoryCredentialsProviderArn".to_string(),
                crate::value::ToValue::to_value(&self.repository_credentials_provider_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-s3datasource.html
    pub struct S3DataSource_ {
        pub compression_type: crate::value::ExpString,
        pub hub_access_config: Option<Box<HubAccessConfig_>>,
        pub model_access_config: Option<Box<ModelAccessConfig_>>,
        pub s3_data_type: crate::value::ExpString,
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Model_S3DataSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Model.S3DataSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Model_S3DataSource as S3DataSource;
    impl crate::value::ToValue for S3DataSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CompressionType".to_string(),
                crate::value::ToValue::to_value(&self.compression_type),
            );
            if let Some(ref value) = self.hub_access_config {
                properties.insert(
                    "HubAccessConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_access_config {
                properties.insert(
                    "ModelAccessConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3DataType".to_string(),
                crate::value::ToValue::to_value(&self.s3_data_type),
            );
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-vpcconfig.html
    pub struct VpcConfig_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Model_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Model.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Model_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
}
pub mod modelbiasjobdefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-batchtransforminput.html
    pub struct BatchTransformInput_ {
        pub data_captured_destination_s3_uri: crate::value::ExpString,
        pub dataset_format: Box<DatasetFormat_>,
        pub end_time_offset: Option<crate::value::ExpString>,
        pub features_attribute: Option<crate::value::ExpString>,
        pub inference_attribute: Option<crate::value::ExpString>,
        pub local_path: crate::value::ExpString,
        pub probability_attribute: Option<crate::value::ExpString>,
        pub probability_threshold_attribute: Option<f64>,
        pub s3_data_distribution_type: Option<crate::value::ExpString>,
        pub s3_input_mode: Option<crate::value::ExpString>,
        pub start_time_offset: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_BatchTransformInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.BatchTransformInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_BatchTransformInput as BatchTransformInput;
    impl crate::value::ToValue for BatchTransformInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataCapturedDestinationS3Uri".to_string(),
                crate::value::ToValue::to_value(&self.data_captured_destination_s3_uri),
            );
            properties.insert(
                "DatasetFormat".to_string(),
                crate::value::ToValue::to_value(&self.dataset_format),
            );
            if let Some(ref value) = self.end_time_offset {
                properties.insert(
                    "EndTimeOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.features_attribute {
                properties.insert(
                    "FeaturesAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_attribute {
                properties.insert(
                    "InferenceAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LocalPath".to_string(),
                crate::value::ToValue::to_value(&self.local_path),
            );
            if let Some(ref value) = self.probability_attribute {
                properties.insert(
                    "ProbabilityAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.probability_threshold_attribute {
                properties.insert(
                    "ProbabilityThresholdAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_data_distribution_type {
                properties.insert(
                    "S3DataDistributionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_input_mode {
                properties.insert(
                    "S3InputMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_time_offset {
                properties.insert(
                    "StartTimeOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-clusterconfig.html
    pub struct ClusterConfig_ {
        pub instance_count: i64,
        pub instance_type: crate::value::ExpString,
        pub volume_kms_key_id: Option<crate::value::ExpString>,
        pub volume_size_in_gb: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_ClusterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.ClusterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_ClusterConfig as ClusterConfig;
    impl crate::value::ToValue for ClusterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceCount".to_string(),
                crate::value::ToValue::to_value(&self.instance_count),
            );
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            if let Some(ref value) = self.volume_kms_key_id {
                properties.insert(
                    "VolumeKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VolumeSizeInGB".to_string(),
                crate::value::ToValue::to_value(&self.volume_size_in_gb),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-constraintsresource.html
    pub struct ConstraintsResource_ {
        pub s3_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_ConstraintsResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.ConstraintsResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_ConstraintsResource as ConstraintsResource;
    impl crate::value::ToValue for ConstraintsResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_uri {
                properties.insert("S3Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-csv.html
    pub struct Csv_ {
        pub header: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_Csv {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.Csv"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_Csv as Csv;
    impl crate::value::ToValue for Csv_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.header {
                properties.insert("Header".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-datasetformat.html
    pub struct DatasetFormat_ {
        pub csv: Option<Box<Csv_>>,
        pub json: Option<Box<Json_>>,
        pub parquet: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_DatasetFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.DatasetFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_DatasetFormat as DatasetFormat;
    impl crate::value::ToValue for DatasetFormat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.csv {
                properties.insert("Csv".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.json {
                properties.insert("Json".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.parquet {
                properties.insert(
                    "Parquet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-endpointinput.html
    pub struct EndpointInput_ {
        pub end_time_offset: Option<crate::value::ExpString>,
        pub endpoint_name: crate::value::ExpString,
        pub features_attribute: Option<crate::value::ExpString>,
        pub inference_attribute: Option<crate::value::ExpString>,
        pub local_path: crate::value::ExpString,
        pub probability_attribute: Option<crate::value::ExpString>,
        pub probability_threshold_attribute: Option<f64>,
        pub s3_data_distribution_type: Option<crate::value::ExpString>,
        pub s3_input_mode: Option<crate::value::ExpString>,
        pub start_time_offset: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_EndpointInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.EndpointInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_EndpointInput as EndpointInput;
    impl crate::value::ToValue for EndpointInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.end_time_offset {
                properties.insert(
                    "EndTimeOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EndpointName".to_string(),
                crate::value::ToValue::to_value(&self.endpoint_name),
            );
            if let Some(ref value) = self.features_attribute {
                properties.insert(
                    "FeaturesAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_attribute {
                properties.insert(
                    "InferenceAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LocalPath".to_string(),
                crate::value::ToValue::to_value(&self.local_path),
            );
            if let Some(ref value) = self.probability_attribute {
                properties.insert(
                    "ProbabilityAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.probability_threshold_attribute {
                properties.insert(
                    "ProbabilityThresholdAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_data_distribution_type {
                properties.insert(
                    "S3DataDistributionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_input_mode {
                properties.insert(
                    "S3InputMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_time_offset {
                properties.insert(
                    "StartTimeOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-json.html
    pub struct Json_ {
        pub line: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_Json {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.Json"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_Json as Json;
    impl crate::value::ToValue for Json_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.line {
                properties.insert("Line".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-modelbiasappspecification.html
    pub struct ModelBiasAppSpecification_ {
        pub config_uri: crate::value::ExpString,
        pub environment: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub image_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_ModelBiasAppSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.ModelBiasAppSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_ModelBiasAppSpecification as ModelBiasAppSpecification;
    impl crate::value::ToValue for ModelBiasAppSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConfigUri".to_string(),
                crate::value::ToValue::to_value(&self.config_uri),
            );
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ImageUri".to_string(),
                crate::value::ToValue::to_value(&self.image_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-modelbiasbaselineconfig.html
    pub struct ModelBiasBaselineConfig_ {
        pub baselining_job_name: Option<crate::value::ExpString>,
        pub constraints_resource: Option<Box<ConstraintsResource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_ModelBiasBaselineConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.ModelBiasBaselineConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_ModelBiasBaselineConfig as ModelBiasBaselineConfig;
    impl crate::value::ToValue for ModelBiasBaselineConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.baselining_job_name {
                properties.insert(
                    "BaseliningJobName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.constraints_resource {
                properties.insert(
                    "ConstraintsResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-modelbiasjobinput.html
    pub struct ModelBiasJobInput_ {
        pub batch_transform_input: Option<Box<BatchTransformInput_>>,
        pub endpoint_input: Option<Box<EndpointInput_>>,
        pub ground_truth_s3_input: Box<MonitoringGroundTruthS3Input_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_ModelBiasJobInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.ModelBiasJobInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_ModelBiasJobInput as ModelBiasJobInput;
    impl crate::value::ToValue for ModelBiasJobInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_transform_input {
                properties.insert(
                    "BatchTransformInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.endpoint_input {
                properties.insert(
                    "EndpointInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "GroundTruthS3Input".to_string(),
                crate::value::ToValue::to_value(&self.ground_truth_s3_input),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-monitoringgroundtruths3input.html
    pub struct MonitoringGroundTruthS3Input_ {
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_MonitoringGroundTruthS3Input {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.MonitoringGroundTruthS3Input"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_MonitoringGroundTruthS3Input as MonitoringGroundTruthS3Input;
    impl crate::value::ToValue for MonitoringGroundTruthS3Input_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-monitoringoutput.html
    pub struct MonitoringOutput_ {
        pub s3_output: Box<S3Output_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_MonitoringOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.MonitoringOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_MonitoringOutput as MonitoringOutput;
    impl crate::value::ToValue for MonitoringOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Output".to_string(),
                crate::value::ToValue::to_value(&self.s3_output),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-monitoringoutputconfig.html
    pub struct MonitoringOutputConfig_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub monitoring_outputs: Vec<MonitoringOutput_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_MonitoringOutputConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.MonitoringOutputConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_MonitoringOutputConfig as MonitoringOutputConfig;
    impl crate::value::ToValue for MonitoringOutputConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MonitoringOutputs".to_string(),
                crate::value::ToValue::to_value(&self.monitoring_outputs),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-monitoringresources.html
    pub struct MonitoringResources_ {
        pub cluster_config: Box<ClusterConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_MonitoringResources {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.MonitoringResources"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_MonitoringResources as MonitoringResources;
    impl crate::value::ToValue for MonitoringResources_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterConfig".to_string(),
                crate::value::ToValue::to_value(&self.cluster_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-networkconfig.html
    pub struct NetworkConfig_ {
        pub enable_inter_container_traffic_encryption: Option<crate::value::ExpBool>,
        pub enable_network_isolation: Option<crate::value::ExpBool>,
        pub vpc_config: Option<Box<VpcConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_NetworkConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.NetworkConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_NetworkConfig as NetworkConfig;
    impl crate::value::ToValue for NetworkConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_inter_container_traffic_encryption {
                properties.insert(
                    "EnableInterContainerTrafficEncryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_network_isolation {
                properties.insert(
                    "EnableNetworkIsolation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_config {
                properties.insert(
                    "VpcConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-s3output.html
    pub struct S3Output_ {
        pub local_path: crate::value::ExpString,
        pub s3_upload_mode: Option<crate::value::ExpString>,
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_S3Output {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.S3Output"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_S3Output as S3Output;
    impl crate::value::ToValue for S3Output_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LocalPath".to_string(),
                crate::value::ToValue::to_value(&self.local_path),
            );
            if let Some(ref value) = self.s3_upload_mode {
                properties.insert(
                    "S3UploadMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-stoppingcondition.html
    pub struct StoppingCondition_ {
        pub max_runtime_in_seconds: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_StoppingCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.StoppingCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_StoppingCondition as StoppingCondition;
    impl crate::value::ToValue for StoppingCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxRuntimeInSeconds".to_string(),
                crate::value::ToValue::to_value(&self.max_runtime_in_seconds),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-vpcconfig.html
    pub struct VpcConfig_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelBiasJobDefinition_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelBiasJobDefinition.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelBiasJobDefinition_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
}
pub mod modelcard {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-additionalinformation.html
    pub struct AdditionalInformation_ {
        pub caveats_and_recommendations: Option<crate::value::ExpString>,
        pub custom_details: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub ethical_considerations: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_AdditionalInformation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.AdditionalInformation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_AdditionalInformation as AdditionalInformation;
    impl crate::value::ToValue for AdditionalInformation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.caveats_and_recommendations {
                properties.insert(
                    "CaveatsAndRecommendations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_details {
                properties.insert(
                    "CustomDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ethical_considerations {
                properties.insert(
                    "EthicalConsiderations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-businessdetails.html
    pub struct BusinessDetails_ {
        pub business_problem: Option<crate::value::ExpString>,
        pub business_stakeholders: Option<crate::value::ExpString>,
        pub line_of_business: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_BusinessDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.BusinessDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_BusinessDetails as BusinessDetails;
    impl crate::value::ToValue for BusinessDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.business_problem {
                properties.insert(
                    "BusinessProblem".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.business_stakeholders {
                properties.insert(
                    "BusinessStakeholders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.line_of_business {
                properties.insert(
                    "LineOfBusiness".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-container.html
    pub struct Container_ {
        pub image: crate::value::ExpString,
        pub model_data_url: Option<crate::value::ExpString>,
        pub nearest_model_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_Container {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.Container"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_Container as Container;
    impl crate::value::ToValue for Container_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Image".to_string(),
                crate::value::ToValue::to_value(&self.image),
            );
            if let Some(ref value) = self.model_data_url {
                properties.insert(
                    "ModelDataUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nearest_model_name {
                properties.insert(
                    "NearestModelName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-content.html
    pub struct Content_ {
        pub additional_information: Option<Box<AdditionalInformation_>>,
        pub business_details: Option<Box<BusinessDetails_>>,
        pub evaluation_details: Option<Vec<EvaluationDetail_>>,
        pub intended_uses: Option<Box<IntendedUses_>>,
        pub model_overview: Option<Box<ModelOverview_>>,
        pub model_package_details: Option<Box<ModelPackageDetails_>>,
        pub training_details: Option<Box<TrainingDetails_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_Content {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.Content"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_Content as Content;
    impl crate::value::ToValue for Content_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_information {
                properties.insert(
                    "AdditionalInformation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.business_details {
                properties.insert(
                    "BusinessDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.evaluation_details {
                properties.insert(
                    "EvaluationDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.intended_uses {
                properties.insert(
                    "IntendedUses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_overview {
                properties.insert(
                    "ModelOverview".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_package_details {
                properties.insert(
                    "ModelPackageDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.training_details {
                properties.insert(
                    "TrainingDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-evaluationdetail.html
    pub struct EvaluationDetail_ {
        pub datasets: Option<Vec<crate::value::ExpString>>,
        pub evaluation_job_arn: Option<crate::value::ExpString>,
        pub evaluation_observation: Option<crate::value::ExpString>,
        pub metadata: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub metric_groups: Option<Vec<MetricGroup_>>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_EvaluationDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.EvaluationDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_EvaluationDetail as EvaluationDetail;
    impl crate::value::ToValue for EvaluationDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.datasets {
                properties.insert(
                    "Datasets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.evaluation_job_arn {
                properties.insert(
                    "EvaluationJobArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.evaluation_observation {
                properties.insert(
                    "EvaluationObservation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metadata {
                properties.insert(
                    "Metadata".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_groups {
                properties.insert(
                    "MetricGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-function.html
    pub struct Function_ {
        pub condition: Option<crate::value::ExpString>,
        pub facet: Option<crate::value::ExpString>,
        pub function: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_Function {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.Function"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_Function as Function;
    impl crate::value::ToValue for Function_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.condition {
                properties.insert(
                    "Condition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.facet {
                properties.insert("Facet".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.function {
                properties.insert(
                    "Function".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-inferenceenvironment.html
    pub struct InferenceEnvironment_ {
        pub container_image: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_InferenceEnvironment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.InferenceEnvironment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_InferenceEnvironment as InferenceEnvironment;
    impl crate::value::ToValue for InferenceEnvironment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_image {
                properties.insert(
                    "ContainerImage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-inferencespecification.html
    pub struct InferenceSpecification_ {
        pub containers: Vec<Container_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_InferenceSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.InferenceSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_InferenceSpecification as InferenceSpecification;
    impl crate::value::ToValue for InferenceSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Containers".to_string(),
                crate::value::ToValue::to_value(&self.containers),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-intendeduses.html
    pub struct IntendedUses_ {
        pub explanations_for_risk_rating: Option<crate::value::ExpString>,
        pub factors_affecting_model_efficiency: Option<crate::value::ExpString>,
        pub intended_uses: Option<crate::value::ExpString>,
        pub purpose_of_model: Option<crate::value::ExpString>,
        pub risk_rating: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_IntendedUses {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.IntendedUses"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_IntendedUses as IntendedUses;
    impl crate::value::ToValue for IntendedUses_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.explanations_for_risk_rating {
                properties.insert(
                    "ExplanationsForRiskRating".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.factors_affecting_model_efficiency {
                properties.insert(
                    "FactorsAffectingModelEfficiency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.intended_uses {
                properties.insert(
                    "IntendedUses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.purpose_of_model {
                properties.insert(
                    "PurposeOfModel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.risk_rating {
                properties.insert(
                    "RiskRating".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-metricdataitems.html
    pub struct MetricDataItems_ {
        pub name: crate::value::ExpString,
        pub notes: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
        pub value: serde_json::Value,
        pub x_axis_name: Option<Vec<crate::value::ExpString>>,
        pub y_axis_name: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_MetricDataItems {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.MetricDataItems"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_MetricDataItems as MetricDataItems;
    impl crate::value::ToValue for MetricDataItems_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.notes {
                properties.insert("Notes".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            if let Some(ref value) = self.x_axis_name {
                properties.insert(
                    "XAxisName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.y_axis_name {
                properties.insert(
                    "YAxisName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-metricgroup.html
    pub struct MetricGroup_ {
        pub metric_data: Vec<MetricDataItems_>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_MetricGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.MetricGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_MetricGroup as MetricGroup;
    impl crate::value::ToValue for MetricGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MetricData".to_string(),
                crate::value::ToValue::to_value(&self.metric_data),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-modeloverview.html
    pub struct ModelOverview_ {
        pub algorithm_type: Option<crate::value::ExpString>,
        pub inference_environment: Option<Box<InferenceEnvironment_>>,
        pub model_artifact: Option<Vec<crate::value::ExpString>>,
        pub model_creator: Option<crate::value::ExpString>,
        pub model_description: Option<crate::value::ExpString>,
        pub model_id: Option<crate::value::ExpString>,
        pub model_name: Option<crate::value::ExpString>,
        pub model_owner: Option<crate::value::ExpString>,
        pub model_version: Option<f64>,
        pub problem_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_ModelOverview {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.ModelOverview"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_ModelOverview as ModelOverview;
    impl crate::value::ToValue for ModelOverview_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.algorithm_type {
                properties.insert(
                    "AlgorithmType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_environment {
                properties.insert(
                    "InferenceEnvironment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_artifact {
                properties.insert(
                    "ModelArtifact".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_creator {
                properties.insert(
                    "ModelCreator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_description {
                properties.insert(
                    "ModelDescription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_id {
                properties.insert(
                    "ModelId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_name {
                properties.insert(
                    "ModelName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_owner {
                properties.insert(
                    "ModelOwner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_version {
                properties.insert(
                    "ModelVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.problem_type {
                properties.insert(
                    "ProblemType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-modelpackagecreator.html
    pub struct ModelPackageCreator_ {
        pub user_profile_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_ModelPackageCreator {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.ModelPackageCreator"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_ModelPackageCreator as ModelPackageCreator;
    impl crate::value::ToValue for ModelPackageCreator_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.user_profile_name {
                properties.insert(
                    "UserProfileName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-modelpackagedetails.html
    pub struct ModelPackageDetails_ {
        pub approval_description: Option<crate::value::ExpString>,
        pub created_by: Option<Box<ModelPackageCreator_>>,
        pub domain: Option<crate::value::ExpString>,
        pub inference_specification: Option<Box<InferenceSpecification_>>,
        pub model_approval_status: Option<crate::value::ExpString>,
        pub model_package_arn: Option<crate::value::ExpString>,
        pub model_package_description: Option<crate::value::ExpString>,
        pub model_package_group_name: Option<crate::value::ExpString>,
        pub model_package_name: Option<crate::value::ExpString>,
        pub model_package_status: Option<crate::value::ExpString>,
        pub model_package_version: Option<f64>,
        pub source_algorithms: Option<Vec<SourceAlgorithm_>>,
        pub task: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_ModelPackageDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.ModelPackageDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_ModelPackageDetails as ModelPackageDetails;
    impl crate::value::ToValue for ModelPackageDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.approval_description {
                properties.insert(
                    "ApprovalDescription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.created_by {
                properties.insert(
                    "CreatedBy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.domain {
                properties.insert("Domain".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.inference_specification {
                properties.insert(
                    "InferenceSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_approval_status {
                properties.insert(
                    "ModelApprovalStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_package_arn {
                properties.insert(
                    "ModelPackageArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_package_description {
                properties.insert(
                    "ModelPackageDescription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_package_group_name {
                properties.insert(
                    "ModelPackageGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_package_name {
                properties.insert(
                    "ModelPackageName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_package_status {
                properties.insert(
                    "ModelPackageStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_package_version {
                properties.insert(
                    "ModelPackageVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_algorithms {
                properties.insert(
                    "SourceAlgorithms".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.task {
                properties.insert("Task".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-objectivefunction.html
    pub struct ObjectiveFunction_ {
        pub function: Option<Box<Function_>>,
        pub notes: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_ObjectiveFunction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.ObjectiveFunction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_ObjectiveFunction as ObjectiveFunction;
    impl crate::value::ToValue for ObjectiveFunction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.function {
                properties.insert(
                    "Function".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.notes {
                properties.insert("Notes".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-securityconfig.html
    pub struct SecurityConfig_ {
        pub kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_SecurityConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.SecurityConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_SecurityConfig as SecurityConfig;
    impl crate::value::ToValue for SecurityConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-sourcealgorithm.html
    pub struct SourceAlgorithm_ {
        pub algorithm_name: crate::value::ExpString,
        pub model_data_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_SourceAlgorithm {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.SourceAlgorithm"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_SourceAlgorithm as SourceAlgorithm;
    impl crate::value::ToValue for SourceAlgorithm_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AlgorithmName".to_string(),
                crate::value::ToValue::to_value(&self.algorithm_name),
            );
            if let Some(ref value) = self.model_data_url {
                properties.insert(
                    "ModelDataUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-trainingdetails.html
    pub struct TrainingDetails_ {
        pub objective_function: Option<Box<ObjectiveFunction_>>,
        pub training_job_details: Option<Box<TrainingJobDetails_>>,
        pub training_observations: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_TrainingDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.TrainingDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_TrainingDetails as TrainingDetails;
    impl crate::value::ToValue for TrainingDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.objective_function {
                properties.insert(
                    "ObjectiveFunction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.training_job_details {
                properties.insert(
                    "TrainingJobDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.training_observations {
                properties.insert(
                    "TrainingObservations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-trainingenvironment.html
    pub struct TrainingEnvironment_ {
        pub container_image: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_TrainingEnvironment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.TrainingEnvironment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_TrainingEnvironment as TrainingEnvironment;
    impl crate::value::ToValue for TrainingEnvironment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_image {
                properties.insert(
                    "ContainerImage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-traininghyperparameter.html
    pub struct TrainingHyperParameter_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_TrainingHyperParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.TrainingHyperParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_TrainingHyperParameter as TrainingHyperParameter;
    impl crate::value::ToValue for TrainingHyperParameter_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-trainingjobdetails.html
    pub struct TrainingJobDetails_ {
        pub hyper_parameters: Option<Vec<TrainingHyperParameter_>>,
        pub training_arn: Option<crate::value::ExpString>,
        pub training_datasets: Option<Vec<crate::value::ExpString>>,
        pub training_environment: Option<Box<TrainingEnvironment_>>,
        pub training_metrics: Option<Vec<TrainingMetric_>>,
        pub user_provided_hyper_parameters: Option<Vec<TrainingHyperParameter_>>,
        pub user_provided_training_metrics: Option<Vec<TrainingMetric_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_TrainingJobDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.TrainingJobDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_TrainingJobDetails as TrainingJobDetails;
    impl crate::value::ToValue for TrainingJobDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hyper_parameters {
                properties.insert(
                    "HyperParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.training_arn {
                properties.insert(
                    "TrainingArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.training_datasets {
                properties.insert(
                    "TrainingDatasets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.training_environment {
                properties.insert(
                    "TrainingEnvironment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.training_metrics {
                properties.insert(
                    "TrainingMetrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_provided_hyper_parameters {
                properties.insert(
                    "UserProvidedHyperParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_provided_training_metrics {
                properties.insert(
                    "UserProvidedTrainingMetrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-trainingmetric.html
    pub struct TrainingMetric_ {
        pub name: crate::value::ExpString,
        pub notes: Option<crate::value::ExpString>,
        pub value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_TrainingMetric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.TrainingMetric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_TrainingMetric as TrainingMetric;
    impl crate::value::ToValue for TrainingMetric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.notes {
                properties.insert("Notes".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelcard-usercontext.html
    pub struct UserContext_ {
        pub domain_id: Option<crate::value::ExpString>,
        pub user_profile_arn: Option<crate::value::ExpString>,
        pub user_profile_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelCard_UserContext {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelCard.UserContext"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelCard_UserContext as UserContext;
    impl crate::value::ToValue for UserContext_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.domain_id {
                properties.insert(
                    "DomainId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_profile_arn {
                properties.insert(
                    "UserProfileArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_profile_name {
                properties.insert(
                    "UserProfileName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod modelexplainabilityjobdefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-batchtransforminput.html
    pub struct BatchTransformInput_ {
        pub data_captured_destination_s3_uri: crate::value::ExpString,
        pub dataset_format: Box<DatasetFormat_>,
        pub features_attribute: Option<crate::value::ExpString>,
        pub inference_attribute: Option<crate::value::ExpString>,
        pub local_path: crate::value::ExpString,
        pub probability_attribute: Option<crate::value::ExpString>,
        pub s3_data_distribution_type: Option<crate::value::ExpString>,
        pub s3_input_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_BatchTransformInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.BatchTransformInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_BatchTransformInput as BatchTransformInput;
    impl crate::value::ToValue for BatchTransformInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataCapturedDestinationS3Uri".to_string(),
                crate::value::ToValue::to_value(&self.data_captured_destination_s3_uri),
            );
            properties.insert(
                "DatasetFormat".to_string(),
                crate::value::ToValue::to_value(&self.dataset_format),
            );
            if let Some(ref value) = self.features_attribute {
                properties.insert(
                    "FeaturesAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_attribute {
                properties.insert(
                    "InferenceAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LocalPath".to_string(),
                crate::value::ToValue::to_value(&self.local_path),
            );
            if let Some(ref value) = self.probability_attribute {
                properties.insert(
                    "ProbabilityAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_data_distribution_type {
                properties.insert(
                    "S3DataDistributionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_input_mode {
                properties.insert(
                    "S3InputMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-clusterconfig.html
    pub struct ClusterConfig_ {
        pub instance_count: i64,
        pub instance_type: crate::value::ExpString,
        pub volume_kms_key_id: Option<crate::value::ExpString>,
        pub volume_size_in_gb: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_ClusterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.ClusterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_ClusterConfig as ClusterConfig;
    impl crate::value::ToValue for ClusterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceCount".to_string(),
                crate::value::ToValue::to_value(&self.instance_count),
            );
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            if let Some(ref value) = self.volume_kms_key_id {
                properties.insert(
                    "VolumeKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VolumeSizeInGB".to_string(),
                crate::value::ToValue::to_value(&self.volume_size_in_gb),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-constraintsresource.html
    pub struct ConstraintsResource_ {
        pub s3_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_ConstraintsResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.ConstraintsResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_ConstraintsResource as ConstraintsResource;
    impl crate::value::ToValue for ConstraintsResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_uri {
                properties.insert("S3Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-csv.html
    pub struct Csv_ {
        pub header: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_Csv {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.Csv"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_Csv as Csv;
    impl crate::value::ToValue for Csv_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.header {
                properties.insert("Header".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-datasetformat.html
    pub struct DatasetFormat_ {
        pub csv: Option<Box<Csv_>>,
        pub json: Option<Box<Json_>>,
        pub parquet: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_DatasetFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.DatasetFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_DatasetFormat as DatasetFormat;
    impl crate::value::ToValue for DatasetFormat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.csv {
                properties.insert("Csv".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.json {
                properties.insert("Json".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.parquet {
                properties.insert(
                    "Parquet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-endpointinput.html
    pub struct EndpointInput_ {
        pub endpoint_name: crate::value::ExpString,
        pub features_attribute: Option<crate::value::ExpString>,
        pub inference_attribute: Option<crate::value::ExpString>,
        pub local_path: crate::value::ExpString,
        pub probability_attribute: Option<crate::value::ExpString>,
        pub s3_data_distribution_type: Option<crate::value::ExpString>,
        pub s3_input_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_EndpointInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.EndpointInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_EndpointInput as EndpointInput;
    impl crate::value::ToValue for EndpointInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EndpointName".to_string(),
                crate::value::ToValue::to_value(&self.endpoint_name),
            );
            if let Some(ref value) = self.features_attribute {
                properties.insert(
                    "FeaturesAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_attribute {
                properties.insert(
                    "InferenceAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LocalPath".to_string(),
                crate::value::ToValue::to_value(&self.local_path),
            );
            if let Some(ref value) = self.probability_attribute {
                properties.insert(
                    "ProbabilityAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_data_distribution_type {
                properties.insert(
                    "S3DataDistributionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_input_mode {
                properties.insert(
                    "S3InputMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-json.html
    pub struct Json_ {
        pub line: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_Json {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.Json"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_Json as Json;
    impl crate::value::ToValue for Json_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.line {
                properties.insert("Line".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-modelexplainabilityappspecification.html
    pub struct ModelExplainabilityAppSpecification_ {
        pub config_uri: crate::value::ExpString,
        pub environment: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub image_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_ModelExplainabilityAppSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.ModelExplainabilityAppSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_ModelExplainabilityAppSpecification as ModelExplainabilityAppSpecification;
    impl crate::value::ToValue for ModelExplainabilityAppSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConfigUri".to_string(),
                crate::value::ToValue::to_value(&self.config_uri),
            );
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ImageUri".to_string(),
                crate::value::ToValue::to_value(&self.image_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-modelexplainabilitybaselineconfig.html
    pub struct ModelExplainabilityBaselineConfig_ {
        pub baselining_job_name: Option<crate::value::ExpString>,
        pub constraints_resource: Option<Box<ConstraintsResource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_ModelExplainabilityBaselineConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.ModelExplainabilityBaselineConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_ModelExplainabilityBaselineConfig as ModelExplainabilityBaselineConfig;
    impl crate::value::ToValue for ModelExplainabilityBaselineConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.baselining_job_name {
                properties.insert(
                    "BaseliningJobName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.constraints_resource {
                properties.insert(
                    "ConstraintsResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-modelexplainabilityjobinput.html
    pub struct ModelExplainabilityJobInput_ {
        pub batch_transform_input: Option<Box<BatchTransformInput_>>,
        pub endpoint_input: Option<Box<EndpointInput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_ModelExplainabilityJobInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.ModelExplainabilityJobInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_ModelExplainabilityJobInput as ModelExplainabilityJobInput;
    impl crate::value::ToValue for ModelExplainabilityJobInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_transform_input {
                properties.insert(
                    "BatchTransformInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.endpoint_input {
                properties.insert(
                    "EndpointInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-monitoringoutput.html
    pub struct MonitoringOutput_ {
        pub s3_output: Box<S3Output_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_MonitoringOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.MonitoringOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_MonitoringOutput as MonitoringOutput;
    impl crate::value::ToValue for MonitoringOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Output".to_string(),
                crate::value::ToValue::to_value(&self.s3_output),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-monitoringoutputconfig.html
    pub struct MonitoringOutputConfig_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub monitoring_outputs: Vec<MonitoringOutput_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_MonitoringOutputConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.MonitoringOutputConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_MonitoringOutputConfig as MonitoringOutputConfig;
    impl crate::value::ToValue for MonitoringOutputConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MonitoringOutputs".to_string(),
                crate::value::ToValue::to_value(&self.monitoring_outputs),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-monitoringresources.html
    pub struct MonitoringResources_ {
        pub cluster_config: Box<ClusterConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_MonitoringResources {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.MonitoringResources"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_MonitoringResources as MonitoringResources;
    impl crate::value::ToValue for MonitoringResources_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterConfig".to_string(),
                crate::value::ToValue::to_value(&self.cluster_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-networkconfig.html
    pub struct NetworkConfig_ {
        pub enable_inter_container_traffic_encryption: Option<crate::value::ExpBool>,
        pub enable_network_isolation: Option<crate::value::ExpBool>,
        pub vpc_config: Option<Box<VpcConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_NetworkConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.NetworkConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_NetworkConfig as NetworkConfig;
    impl crate::value::ToValue for NetworkConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_inter_container_traffic_encryption {
                properties.insert(
                    "EnableInterContainerTrafficEncryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_network_isolation {
                properties.insert(
                    "EnableNetworkIsolation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_config {
                properties.insert(
                    "VpcConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-s3output.html
    pub struct S3Output_ {
        pub local_path: crate::value::ExpString,
        pub s3_upload_mode: Option<crate::value::ExpString>,
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_S3Output {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.S3Output"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_S3Output as S3Output;
    impl crate::value::ToValue for S3Output_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LocalPath".to_string(),
                crate::value::ToValue::to_value(&self.local_path),
            );
            if let Some(ref value) = self.s3_upload_mode {
                properties.insert(
                    "S3UploadMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-stoppingcondition.html
    pub struct StoppingCondition_ {
        pub max_runtime_in_seconds: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_StoppingCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.StoppingCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_StoppingCondition as StoppingCondition;
    impl crate::value::ToValue for StoppingCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxRuntimeInSeconds".to_string(),
                crate::value::ToValue::to_value(&self.max_runtime_in_seconds),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-vpcconfig.html
    pub struct VpcConfig_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelExplainabilityJobDefinition.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
}
pub mod modelpackage {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-additionalinferencespecificationdefinition.html
    pub struct AdditionalInferenceSpecificationDefinition_ {
        pub containers: Vec<ModelPackageContainerDefinition_>,
        pub description: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub supported_content_types: Option<Vec<crate::value::ExpString>>,
        pub supported_realtime_inference_instance_types: Option<Vec<crate::value::ExpString>>,
        pub supported_response_mime_types: Option<Vec<crate::value::ExpString>>,
        pub supported_transform_instance_types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_AdditionalInferenceSpecificationDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.AdditionalInferenceSpecificationDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_AdditionalInferenceSpecificationDefinition as AdditionalInferenceSpecificationDefinition;
    impl crate::value::ToValue for AdditionalInferenceSpecificationDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Containers".to_string(),
                crate::value::ToValue::to_value(&self.containers),
            );
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.supported_content_types {
                properties.insert(
                    "SupportedContentTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.supported_realtime_inference_instance_types {
                properties.insert(
                    "SupportedRealtimeInferenceInstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.supported_response_mime_types {
                properties.insert(
                    "SupportedResponseMIMETypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.supported_transform_instance_types {
                properties.insert(
                    "SupportedTransformInstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-bias.html
    pub struct Bias_ {
        pub post_training_report: Option<Box<MetricsSource_>>,
        pub pre_training_report: Option<Box<MetricsSource_>>,
        pub report: Option<Box<MetricsSource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_Bias {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.Bias"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_Bias as Bias;
    impl crate::value::ToValue for Bias_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.post_training_report {
                properties.insert(
                    "PostTrainingReport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pre_training_report {
                properties.insert(
                    "PreTrainingReport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.report {
                properties.insert("Report".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-datasource.html
    pub struct DataSource_ {
        pub s3_data_source: Box<S3DataSource_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_DataSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.DataSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_DataSource as DataSource;
    impl crate::value::ToValue for DataSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3DataSource".to_string(),
                crate::value::ToValue::to_value(&self.s3_data_source),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckbaselines.html
    pub struct DriftCheckBaselines_ {
        pub bias: Option<Box<DriftCheckBias_>>,
        pub explainability: Option<Box<DriftCheckExplainability_>>,
        pub model_data_quality: Option<Box<DriftCheckModelDataQuality_>>,
        pub model_quality: Option<Box<DriftCheckModelQuality_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_DriftCheckBaselines {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.DriftCheckBaselines"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_DriftCheckBaselines as DriftCheckBaselines;
    impl crate::value::ToValue for DriftCheckBaselines_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bias {
                properties.insert("Bias".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.explainability {
                properties.insert(
                    "Explainability".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_data_quality {
                properties.insert(
                    "ModelDataQuality".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_quality {
                properties.insert(
                    "ModelQuality".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckbias.html
    pub struct DriftCheckBias_ {
        pub config_file: Option<Box<FileSource_>>,
        pub post_training_constraints: Option<Box<MetricsSource_>>,
        pub pre_training_constraints: Option<Box<MetricsSource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_DriftCheckBias {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.DriftCheckBias"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_DriftCheckBias as DriftCheckBias;
    impl crate::value::ToValue for DriftCheckBias_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.config_file {
                properties.insert(
                    "ConfigFile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.post_training_constraints {
                properties.insert(
                    "PostTrainingConstraints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pre_training_constraints {
                properties.insert(
                    "PreTrainingConstraints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckexplainability.html
    pub struct DriftCheckExplainability_ {
        pub config_file: Option<Box<FileSource_>>,
        pub constraints: Option<Box<MetricsSource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_DriftCheckExplainability {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.DriftCheckExplainability"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_DriftCheckExplainability as DriftCheckExplainability;
    impl crate::value::ToValue for DriftCheckExplainability_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.config_file {
                properties.insert(
                    "ConfigFile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.constraints {
                properties.insert(
                    "Constraints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckmodeldataquality.html
    pub struct DriftCheckModelDataQuality_ {
        pub constraints: Option<Box<MetricsSource_>>,
        pub statistics: Option<Box<MetricsSource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_DriftCheckModelDataQuality {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.DriftCheckModelDataQuality"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_DriftCheckModelDataQuality as DriftCheckModelDataQuality;
    impl crate::value::ToValue for DriftCheckModelDataQuality_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.constraints {
                properties.insert(
                    "Constraints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.statistics {
                properties.insert(
                    "Statistics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckmodelquality.html
    pub struct DriftCheckModelQuality_ {
        pub constraints: Option<Box<MetricsSource_>>,
        pub statistics: Option<Box<MetricsSource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_DriftCheckModelQuality {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.DriftCheckModelQuality"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_DriftCheckModelQuality as DriftCheckModelQuality;
    impl crate::value::ToValue for DriftCheckModelQuality_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.constraints {
                properties.insert(
                    "Constraints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.statistics {
                properties.insert(
                    "Statistics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-explainability.html
    pub struct Explainability_ {
        pub report: Option<Box<MetricsSource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_Explainability {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.Explainability"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_Explainability as Explainability;
    impl crate::value::ToValue for Explainability_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.report {
                properties.insert("Report".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-filesource.html
    pub struct FileSource_ {
        pub content_digest: Option<crate::value::ExpString>,
        pub content_type: Option<crate::value::ExpString>,
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_FileSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.FileSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_FileSource as FileSource;
    impl crate::value::ToValue for FileSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content_digest {
                properties.insert(
                    "ContentDigest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.content_type {
                properties.insert(
                    "ContentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-inferencespecification.html
    pub struct InferenceSpecification_ {
        pub containers: Vec<ModelPackageContainerDefinition_>,
        pub supported_content_types: Vec<crate::value::ExpString>,
        pub supported_realtime_inference_instance_types: Option<Vec<crate::value::ExpString>>,
        pub supported_response_mime_types: Vec<crate::value::ExpString>,
        pub supported_transform_instance_types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_InferenceSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.InferenceSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_InferenceSpecification as InferenceSpecification;
    impl crate::value::ToValue for InferenceSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Containers".to_string(),
                crate::value::ToValue::to_value(&self.containers),
            );
            properties.insert(
                "SupportedContentTypes".to_string(),
                crate::value::ToValue::to_value(&self.supported_content_types),
            );
            if let Some(ref value) = self.supported_realtime_inference_instance_types {
                properties.insert(
                    "SupportedRealtimeInferenceInstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SupportedResponseMIMETypes".to_string(),
                crate::value::ToValue::to_value(&self.supported_response_mime_types),
            );
            if let Some(ref value) = self.supported_transform_instance_types {
                properties.insert(
                    "SupportedTransformInstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-metadataproperties.html
    pub struct MetadataProperties_ {
        pub commit_id: Option<crate::value::ExpString>,
        pub generated_by: Option<crate::value::ExpString>,
        pub project_id: Option<crate::value::ExpString>,
        pub repository: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_MetadataProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.MetadataProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_MetadataProperties as MetadataProperties;
    impl crate::value::ToValue for MetadataProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.commit_id {
                properties.insert(
                    "CommitId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.generated_by {
                properties.insert(
                    "GeneratedBy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.project_id {
                properties.insert(
                    "ProjectId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.repository {
                properties.insert(
                    "Repository".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-metricssource.html
    pub struct MetricsSource_ {
        pub content_digest: Option<crate::value::ExpString>,
        pub content_type: crate::value::ExpString,
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_MetricsSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.MetricsSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_MetricsSource as MetricsSource;
    impl crate::value::ToValue for MetricsSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content_digest {
                properties.insert(
                    "ContentDigest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ContentType".to_string(),
                crate::value::ToValue::to_value(&self.content_type),
            );
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelaccessconfig.html
    pub struct ModelAccessConfig_ {
        pub accept_eula: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_ModelAccessConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.ModelAccessConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_ModelAccessConfig as ModelAccessConfig;
    impl crate::value::ToValue for ModelAccessConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AcceptEula".to_string(),
                crate::value::ToValue::to_value(&self.accept_eula),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelcard.html
    pub struct ModelCard_ {
        pub model_card_content: crate::value::ExpString,
        pub model_card_status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_ModelCard {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.ModelCard"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_ModelCard as ModelCard;
    impl crate::value::ToValue for ModelCard_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ModelCardContent".to_string(),
                crate::value::ToValue::to_value(&self.model_card_content),
            );
            properties.insert(
                "ModelCardStatus".to_string(),
                crate::value::ToValue::to_value(&self.model_card_status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modeldataquality.html
    pub struct ModelDataQuality_ {
        pub constraints: Option<Box<MetricsSource_>>,
        pub statistics: Option<Box<MetricsSource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_ModelDataQuality {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.ModelDataQuality"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_ModelDataQuality as ModelDataQuality;
    impl crate::value::ToValue for ModelDataQuality_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.constraints {
                properties.insert(
                    "Constraints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.statistics {
                properties.insert(
                    "Statistics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modeldatasource.html
    pub struct ModelDataSource_ {
        pub s3_data_source: Option<Box<S3ModelDataSource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_ModelDataSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.ModelDataSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_ModelDataSource as ModelDataSource;
    impl crate::value::ToValue for ModelDataSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_data_source {
                properties.insert(
                    "S3DataSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelinput.html
    pub struct ModelInput_ {
        pub data_input_config: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_ModelInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.ModelInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_ModelInput as ModelInput;
    impl crate::value::ToValue for ModelInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataInputConfig".to_string(),
                crate::value::ToValue::to_value(&self.data_input_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelmetrics.html
    pub struct ModelMetrics_ {
        pub bias: Option<Box<Bias_>>,
        pub explainability: Option<Box<Explainability_>>,
        pub model_data_quality: Option<Box<ModelDataQuality_>>,
        pub model_quality: Option<Box<ModelQuality_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_ModelMetrics {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.ModelMetrics"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_ModelMetrics as ModelMetrics;
    impl crate::value::ToValue for ModelMetrics_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bias {
                properties.insert("Bias".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.explainability {
                properties.insert(
                    "Explainability".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_data_quality {
                properties.insert(
                    "ModelDataQuality".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_quality {
                properties.insert(
                    "ModelQuality".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagecontainerdefinition.html
    pub struct ModelPackageContainerDefinition_ {
        pub container_hostname: Option<crate::value::ExpString>,
        pub environment: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub framework: Option<crate::value::ExpString>,
        pub framework_version: Option<crate::value::ExpString>,
        pub image: crate::value::ExpString,
        pub image_digest: Option<crate::value::ExpString>,
        pub model_data_source: Option<Box<ModelDataSource_>>,
        pub model_data_url: Option<crate::value::ExpString>,
        pub model_input: Option<Box<ModelInput_>>,
        pub nearest_model_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_ModelPackageContainerDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.ModelPackageContainerDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_ModelPackageContainerDefinition as ModelPackageContainerDefinition;
    impl crate::value::ToValue for ModelPackageContainerDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_hostname {
                properties.insert(
                    "ContainerHostname".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.framework {
                properties.insert(
                    "Framework".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.framework_version {
                properties.insert(
                    "FrameworkVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Image".to_string(),
                crate::value::ToValue::to_value(&self.image),
            );
            if let Some(ref value) = self.image_digest {
                properties.insert(
                    "ImageDigest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_data_source {
                properties.insert(
                    "ModelDataSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_data_url {
                properties.insert(
                    "ModelDataUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_input {
                properties.insert(
                    "ModelInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nearest_model_name {
                properties.insert(
                    "NearestModelName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagestatusdetails.html
    pub struct ModelPackageStatusDetails_ {
        pub validation_statuses: Option<Vec<ModelPackageStatusItem_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_ModelPackageStatusDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.ModelPackageStatusDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_ModelPackageStatusDetails as ModelPackageStatusDetails;
    impl crate::value::ToValue for ModelPackageStatusDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.validation_statuses {
                properties.insert(
                    "ValidationStatuses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagestatusitem.html
    pub struct ModelPackageStatusItem_ {
        pub failure_reason: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_ModelPackageStatusItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.ModelPackageStatusItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_ModelPackageStatusItem as ModelPackageStatusItem;
    impl crate::value::ToValue for ModelPackageStatusItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.failure_reason {
                properties.insert(
                    "FailureReason".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelquality.html
    pub struct ModelQuality_ {
        pub constraints: Option<Box<MetricsSource_>>,
        pub statistics: Option<Box<MetricsSource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_ModelQuality {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.ModelQuality"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_ModelQuality as ModelQuality;
    impl crate::value::ToValue for ModelQuality_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.constraints {
                properties.insert(
                    "Constraints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.statistics {
                properties.insert(
                    "Statistics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-s3datasource.html
    pub struct S3DataSource_ {
        pub s3_data_type: crate::value::ExpString,
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_S3DataSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.S3DataSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_S3DataSource as S3DataSource;
    impl crate::value::ToValue for S3DataSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3DataType".to_string(),
                crate::value::ToValue::to_value(&self.s3_data_type),
            );
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-s3modeldatasource.html
    pub struct S3ModelDataSource_ {
        pub compression_type: crate::value::ExpString,
        pub model_access_config: Option<Box<ModelAccessConfig_>>,
        pub s3_data_type: crate::value::ExpString,
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_S3ModelDataSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.S3ModelDataSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_S3ModelDataSource as S3ModelDataSource;
    impl crate::value::ToValue for S3ModelDataSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CompressionType".to_string(),
                crate::value::ToValue::to_value(&self.compression_type),
            );
            if let Some(ref value) = self.model_access_config {
                properties.insert(
                    "ModelAccessConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3DataType".to_string(),
                crate::value::ToValue::to_value(&self.s3_data_type),
            );
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-securityconfig.html
    pub struct SecurityConfig_ {
        pub kms_key_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_SecurityConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.SecurityConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_SecurityConfig as SecurityConfig;
    impl crate::value::ToValue for SecurityConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(&self.kms_key_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-sourcealgorithm.html
    pub struct SourceAlgorithm_ {
        pub algorithm_name: crate::value::ExpString,
        pub model_data_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_SourceAlgorithm {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.SourceAlgorithm"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_SourceAlgorithm as SourceAlgorithm;
    impl crate::value::ToValue for SourceAlgorithm_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AlgorithmName".to_string(),
                crate::value::ToValue::to_value(&self.algorithm_name),
            );
            if let Some(ref value) = self.model_data_url {
                properties.insert(
                    "ModelDataUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-sourcealgorithmspecification.html
    pub struct SourceAlgorithmSpecification_ {
        pub source_algorithms: Vec<SourceAlgorithm_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_SourceAlgorithmSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.SourceAlgorithmSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_SourceAlgorithmSpecification as SourceAlgorithmSpecification;
    impl crate::value::ToValue for SourceAlgorithmSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SourceAlgorithms".to_string(),
                crate::value::ToValue::to_value(&self.source_algorithms),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transforminput.html
    pub struct TransformInput_ {
        pub compression_type: Option<crate::value::ExpString>,
        pub content_type: Option<crate::value::ExpString>,
        pub data_source: Box<DataSource_>,
        pub split_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_TransformInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.TransformInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_TransformInput as TransformInput;
    impl crate::value::ToValue for TransformInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.compression_type {
                properties.insert(
                    "CompressionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.content_type {
                properties.insert(
                    "ContentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DataSource".to_string(),
                crate::value::ToValue::to_value(&self.data_source),
            );
            if let Some(ref value) = self.split_type {
                properties.insert(
                    "SplitType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformjobdefinition.html
    pub struct TransformJobDefinition_ {
        pub batch_strategy: Option<crate::value::ExpString>,
        pub environment: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub max_concurrent_transforms: Option<i64>,
        pub max_payload_in_mb: Option<i64>,
        pub transform_input: Box<TransformInput_>,
        pub transform_output: Box<TransformOutput_>,
        pub transform_resources: Box<TransformResources_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_TransformJobDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.TransformJobDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_TransformJobDefinition as TransformJobDefinition;
    impl crate::value::ToValue for TransformJobDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_strategy {
                properties.insert(
                    "BatchStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_concurrent_transforms {
                properties.insert(
                    "MaxConcurrentTransforms".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_payload_in_mb {
                properties.insert(
                    "MaxPayloadInMB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TransformInput".to_string(),
                crate::value::ToValue::to_value(&self.transform_input),
            );
            properties.insert(
                "TransformOutput".to_string(),
                crate::value::ToValue::to_value(&self.transform_output),
            );
            properties.insert(
                "TransformResources".to_string(),
                crate::value::ToValue::to_value(&self.transform_resources),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformoutput.html
    pub struct TransformOutput_ {
        pub accept: Option<crate::value::ExpString>,
        pub assemble_with: Option<crate::value::ExpString>,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub s3_output_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_TransformOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.TransformOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_TransformOutput as TransformOutput;
    impl crate::value::ToValue for TransformOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.accept {
                properties.insert("Accept".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.assemble_with {
                properties.insert(
                    "AssembleWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3OutputPath".to_string(),
                crate::value::ToValue::to_value(&self.s3_output_path),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformresources.html
    pub struct TransformResources_ {
        pub instance_count: i64,
        pub instance_type: crate::value::ExpString,
        pub volume_kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_TransformResources {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.TransformResources"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_TransformResources as TransformResources;
    impl crate::value::ToValue for TransformResources_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceCount".to_string(),
                crate::value::ToValue::to_value(&self.instance_count),
            );
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            if let Some(ref value) = self.volume_kms_key_id {
                properties.insert(
                    "VolumeKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-validationprofile.html
    pub struct ValidationProfile_ {
        pub profile_name: crate::value::ExpString,
        pub transform_job_definition: Box<TransformJobDefinition_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_ValidationProfile {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.ValidationProfile"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_ValidationProfile as ValidationProfile;
    impl crate::value::ToValue for ValidationProfile_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ProfileName".to_string(),
                crate::value::ToValue::to_value(&self.profile_name),
            );
            properties.insert(
                "TransformJobDefinition".to_string(),
                crate::value::ToValue::to_value(&self.transform_job_definition),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-validationspecification.html
    pub struct ValidationSpecification_ {
        pub validation_profiles: Vec<ValidationProfile_>,
        pub validation_role: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelPackage_ValidationSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelPackage.ValidationSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelPackage_ValidationSpecification as ValidationSpecification;
    impl crate::value::ToValue for ValidationSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ValidationProfiles".to_string(),
                crate::value::ToValue::to_value(&self.validation_profiles),
            );
            properties.insert(
                "ValidationRole".to_string(),
                crate::value::ToValue::to_value(&self.validation_role),
            );
            properties.into()
        }
    }
}
pub mod modelqualityjobdefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-batchtransforminput.html
    pub struct BatchTransformInput_ {
        pub data_captured_destination_s3_uri: crate::value::ExpString,
        pub dataset_format: Box<DatasetFormat_>,
        pub end_time_offset: Option<crate::value::ExpString>,
        pub inference_attribute: Option<crate::value::ExpString>,
        pub local_path: crate::value::ExpString,
        pub probability_attribute: Option<crate::value::ExpString>,
        pub probability_threshold_attribute: Option<f64>,
        pub s3_data_distribution_type: Option<crate::value::ExpString>,
        pub s3_input_mode: Option<crate::value::ExpString>,
        pub start_time_offset: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_BatchTransformInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.BatchTransformInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_BatchTransformInput as BatchTransformInput;
    impl crate::value::ToValue for BatchTransformInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataCapturedDestinationS3Uri".to_string(),
                crate::value::ToValue::to_value(&self.data_captured_destination_s3_uri),
            );
            properties.insert(
                "DatasetFormat".to_string(),
                crate::value::ToValue::to_value(&self.dataset_format),
            );
            if let Some(ref value) = self.end_time_offset {
                properties.insert(
                    "EndTimeOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_attribute {
                properties.insert(
                    "InferenceAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LocalPath".to_string(),
                crate::value::ToValue::to_value(&self.local_path),
            );
            if let Some(ref value) = self.probability_attribute {
                properties.insert(
                    "ProbabilityAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.probability_threshold_attribute {
                properties.insert(
                    "ProbabilityThresholdAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_data_distribution_type {
                properties.insert(
                    "S3DataDistributionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_input_mode {
                properties.insert(
                    "S3InputMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_time_offset {
                properties.insert(
                    "StartTimeOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-clusterconfig.html
    pub struct ClusterConfig_ {
        pub instance_count: i64,
        pub instance_type: crate::value::ExpString,
        pub volume_kms_key_id: Option<crate::value::ExpString>,
        pub volume_size_in_gb: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_ClusterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.ClusterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_ClusterConfig as ClusterConfig;
    impl crate::value::ToValue for ClusterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceCount".to_string(),
                crate::value::ToValue::to_value(&self.instance_count),
            );
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            if let Some(ref value) = self.volume_kms_key_id {
                properties.insert(
                    "VolumeKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VolumeSizeInGB".to_string(),
                crate::value::ToValue::to_value(&self.volume_size_in_gb),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-constraintsresource.html
    pub struct ConstraintsResource_ {
        pub s3_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_ConstraintsResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.ConstraintsResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_ConstraintsResource as ConstraintsResource;
    impl crate::value::ToValue for ConstraintsResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_uri {
                properties.insert("S3Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-csv.html
    pub struct Csv_ {
        pub header: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_Csv {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.Csv"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_Csv as Csv;
    impl crate::value::ToValue for Csv_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.header {
                properties.insert("Header".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-datasetformat.html
    pub struct DatasetFormat_ {
        pub csv: Option<Box<Csv_>>,
        pub json: Option<Box<Json_>>,
        pub parquet: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_DatasetFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.DatasetFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_DatasetFormat as DatasetFormat;
    impl crate::value::ToValue for DatasetFormat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.csv {
                properties.insert("Csv".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.json {
                properties.insert("Json".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.parquet {
                properties.insert(
                    "Parquet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-endpointinput.html
    pub struct EndpointInput_ {
        pub end_time_offset: Option<crate::value::ExpString>,
        pub endpoint_name: crate::value::ExpString,
        pub inference_attribute: Option<crate::value::ExpString>,
        pub local_path: crate::value::ExpString,
        pub probability_attribute: Option<crate::value::ExpString>,
        pub probability_threshold_attribute: Option<f64>,
        pub s3_data_distribution_type: Option<crate::value::ExpString>,
        pub s3_input_mode: Option<crate::value::ExpString>,
        pub start_time_offset: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_EndpointInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.EndpointInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_EndpointInput as EndpointInput;
    impl crate::value::ToValue for EndpointInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.end_time_offset {
                properties.insert(
                    "EndTimeOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EndpointName".to_string(),
                crate::value::ToValue::to_value(&self.endpoint_name),
            );
            if let Some(ref value) = self.inference_attribute {
                properties.insert(
                    "InferenceAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LocalPath".to_string(),
                crate::value::ToValue::to_value(&self.local_path),
            );
            if let Some(ref value) = self.probability_attribute {
                properties.insert(
                    "ProbabilityAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.probability_threshold_attribute {
                properties.insert(
                    "ProbabilityThresholdAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_data_distribution_type {
                properties.insert(
                    "S3DataDistributionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_input_mode {
                properties.insert(
                    "S3InputMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_time_offset {
                properties.insert(
                    "StartTimeOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-json.html
    pub struct Json_ {
        pub line: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_Json {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.Json"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_Json as Json;
    impl crate::value::ToValue for Json_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.line {
                properties.insert("Line".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualityappspecification.html
    pub struct ModelQualityAppSpecification_ {
        pub container_arguments: Option<Vec<crate::value::ExpString>>,
        pub container_entrypoint: Option<Vec<crate::value::ExpString>>,
        pub environment: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub image_uri: crate::value::ExpString,
        pub post_analytics_processor_source_uri: Option<crate::value::ExpString>,
        pub problem_type: crate::value::ExpString,
        pub record_preprocessor_source_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_ModelQualityAppSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.ModelQualityAppSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_ModelQualityAppSpecification as ModelQualityAppSpecification;
    impl crate::value::ToValue for ModelQualityAppSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_arguments {
                properties.insert(
                    "ContainerArguments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_entrypoint {
                properties.insert(
                    "ContainerEntrypoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ImageUri".to_string(),
                crate::value::ToValue::to_value(&self.image_uri),
            );
            if let Some(ref value) = self.post_analytics_processor_source_uri {
                properties.insert(
                    "PostAnalyticsProcessorSourceUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ProblemType".to_string(),
                crate::value::ToValue::to_value(&self.problem_type),
            );
            if let Some(ref value) = self.record_preprocessor_source_uri {
                properties.insert(
                    "RecordPreprocessorSourceUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualitybaselineconfig.html
    pub struct ModelQualityBaselineConfig_ {
        pub baselining_job_name: Option<crate::value::ExpString>,
        pub constraints_resource: Option<Box<ConstraintsResource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_ModelQualityBaselineConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.ModelQualityBaselineConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_ModelQualityBaselineConfig as ModelQualityBaselineConfig;
    impl crate::value::ToValue for ModelQualityBaselineConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.baselining_job_name {
                properties.insert(
                    "BaseliningJobName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.constraints_resource {
                properties.insert(
                    "ConstraintsResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualityjobinput.html
    pub struct ModelQualityJobInput_ {
        pub batch_transform_input: Option<Box<BatchTransformInput_>>,
        pub endpoint_input: Option<Box<EndpointInput_>>,
        pub ground_truth_s3_input: Box<MonitoringGroundTruthS3Input_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_ModelQualityJobInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.ModelQualityJobInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_ModelQualityJobInput as ModelQualityJobInput;
    impl crate::value::ToValue for ModelQualityJobInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_transform_input {
                properties.insert(
                    "BatchTransformInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.endpoint_input {
                properties.insert(
                    "EndpointInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "GroundTruthS3Input".to_string(),
                crate::value::ToValue::to_value(&self.ground_truth_s3_input),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-monitoringgroundtruths3input.html
    pub struct MonitoringGroundTruthS3Input_ {
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_MonitoringGroundTruthS3Input {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.MonitoringGroundTruthS3Input"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_MonitoringGroundTruthS3Input as MonitoringGroundTruthS3Input;
    impl crate::value::ToValue for MonitoringGroundTruthS3Input_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-monitoringoutput.html
    pub struct MonitoringOutput_ {
        pub s3_output: Box<S3Output_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_MonitoringOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.MonitoringOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_MonitoringOutput as MonitoringOutput;
    impl crate::value::ToValue for MonitoringOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Output".to_string(),
                crate::value::ToValue::to_value(&self.s3_output),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-monitoringoutputconfig.html
    pub struct MonitoringOutputConfig_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub monitoring_outputs: Vec<MonitoringOutput_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_MonitoringOutputConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.MonitoringOutputConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_MonitoringOutputConfig as MonitoringOutputConfig;
    impl crate::value::ToValue for MonitoringOutputConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MonitoringOutputs".to_string(),
                crate::value::ToValue::to_value(&self.monitoring_outputs),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-monitoringresources.html
    pub struct MonitoringResources_ {
        pub cluster_config: Box<ClusterConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_MonitoringResources {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.MonitoringResources"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_MonitoringResources as MonitoringResources;
    impl crate::value::ToValue for MonitoringResources_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterConfig".to_string(),
                crate::value::ToValue::to_value(&self.cluster_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-networkconfig.html
    pub struct NetworkConfig_ {
        pub enable_inter_container_traffic_encryption: Option<crate::value::ExpBool>,
        pub enable_network_isolation: Option<crate::value::ExpBool>,
        pub vpc_config: Option<Box<VpcConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_NetworkConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.NetworkConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_NetworkConfig as NetworkConfig;
    impl crate::value::ToValue for NetworkConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_inter_container_traffic_encryption {
                properties.insert(
                    "EnableInterContainerTrafficEncryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_network_isolation {
                properties.insert(
                    "EnableNetworkIsolation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_config {
                properties.insert(
                    "VpcConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-s3output.html
    pub struct S3Output_ {
        pub local_path: crate::value::ExpString,
        pub s3_upload_mode: Option<crate::value::ExpString>,
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_S3Output {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.S3Output"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_S3Output as S3Output;
    impl crate::value::ToValue for S3Output_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LocalPath".to_string(),
                crate::value::ToValue::to_value(&self.local_path),
            );
            if let Some(ref value) = self.s3_upload_mode {
                properties.insert(
                    "S3UploadMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-stoppingcondition.html
    pub struct StoppingCondition_ {
        pub max_runtime_in_seconds: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_StoppingCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.StoppingCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_StoppingCondition as StoppingCondition;
    impl crate::value::ToValue for StoppingCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxRuntimeInSeconds".to_string(),
                crate::value::ToValue::to_value(&self.max_runtime_in_seconds),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-vpcconfig.html
    pub struct VpcConfig_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ModelQualityJobDefinition_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ModelQualityJobDefinition.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ModelQualityJobDefinition_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
}
pub mod monitoringschedule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-baselineconfig.html
    pub struct BaselineConfig_ {
        pub constraints_resource: Option<Box<ConstraintsResource_>>,
        pub statistics_resource: Option<Box<StatisticsResource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_BaselineConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.BaselineConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_BaselineConfig as BaselineConfig;
    impl crate::value::ToValue for BaselineConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.constraints_resource {
                properties.insert(
                    "ConstraintsResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.statistics_resource {
                properties.insert(
                    "StatisticsResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-batchtransforminput.html
    pub struct BatchTransformInput_ {
        pub data_captured_destination_s3_uri: crate::value::ExpString,
        pub dataset_format: Box<DatasetFormat_>,
        pub exclude_features_attribute: Option<crate::value::ExpString>,
        pub local_path: crate::value::ExpString,
        pub s3_data_distribution_type: Option<crate::value::ExpString>,
        pub s3_input_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_BatchTransformInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.BatchTransformInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_BatchTransformInput as BatchTransformInput;
    impl crate::value::ToValue for BatchTransformInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataCapturedDestinationS3Uri".to_string(),
                crate::value::ToValue::to_value(&self.data_captured_destination_s3_uri),
            );
            properties.insert(
                "DatasetFormat".to_string(),
                crate::value::ToValue::to_value(&self.dataset_format),
            );
            if let Some(ref value) = self.exclude_features_attribute {
                properties.insert(
                    "ExcludeFeaturesAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LocalPath".to_string(),
                crate::value::ToValue::to_value(&self.local_path),
            );
            if let Some(ref value) = self.s3_data_distribution_type {
                properties.insert(
                    "S3DataDistributionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_input_mode {
                properties.insert(
                    "S3InputMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-clusterconfig.html
    pub struct ClusterConfig_ {
        pub instance_count: i64,
        pub instance_type: crate::value::ExpString,
        pub volume_kms_key_id: Option<crate::value::ExpString>,
        pub volume_size_in_gb: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_ClusterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.ClusterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_ClusterConfig as ClusterConfig;
    impl crate::value::ToValue for ClusterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceCount".to_string(),
                crate::value::ToValue::to_value(&self.instance_count),
            );
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            if let Some(ref value) = self.volume_kms_key_id {
                properties.insert(
                    "VolumeKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VolumeSizeInGB".to_string(),
                crate::value::ToValue::to_value(&self.volume_size_in_gb),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-constraintsresource.html
    pub struct ConstraintsResource_ {
        pub s3_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_ConstraintsResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.ConstraintsResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_ConstraintsResource as ConstraintsResource;
    impl crate::value::ToValue for ConstraintsResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_uri {
                properties.insert("S3Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-csv.html
    pub struct Csv_ {
        pub header: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_Csv {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.Csv"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_Csv as Csv;
    impl crate::value::ToValue for Csv_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.header {
                properties.insert("Header".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-datasetformat.html
    pub struct DatasetFormat_ {
        pub csv: Option<Box<Csv_>>,
        pub json: Option<Box<Json_>>,
        pub parquet: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_DatasetFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.DatasetFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_DatasetFormat as DatasetFormat;
    impl crate::value::ToValue for DatasetFormat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.csv {
                properties.insert("Csv".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.json {
                properties.insert("Json".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.parquet {
                properties.insert(
                    "Parquet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-endpointinput.html
    pub struct EndpointInput_ {
        pub endpoint_name: crate::value::ExpString,
        pub exclude_features_attribute: Option<crate::value::ExpString>,
        pub local_path: crate::value::ExpString,
        pub s3_data_distribution_type: Option<crate::value::ExpString>,
        pub s3_input_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_EndpointInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.EndpointInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_EndpointInput as EndpointInput;
    impl crate::value::ToValue for EndpointInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EndpointName".to_string(),
                crate::value::ToValue::to_value(&self.endpoint_name),
            );
            if let Some(ref value) = self.exclude_features_attribute {
                properties.insert(
                    "ExcludeFeaturesAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LocalPath".to_string(),
                crate::value::ToValue::to_value(&self.local_path),
            );
            if let Some(ref value) = self.s3_data_distribution_type {
                properties.insert(
                    "S3DataDistributionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_input_mode {
                properties.insert(
                    "S3InputMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-json.html
    pub struct Json_ {
        pub line: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_Json {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.Json"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_Json as Json;
    impl crate::value::ToValue for Json_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.line {
                properties.insert("Line".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringappspecification.html
    pub struct MonitoringAppSpecification_ {
        pub container_arguments: Option<Vec<crate::value::ExpString>>,
        pub container_entrypoint: Option<Vec<crate::value::ExpString>>,
        pub image_uri: crate::value::ExpString,
        pub post_analytics_processor_source_uri: Option<crate::value::ExpString>,
        pub record_preprocessor_source_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_MonitoringAppSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.MonitoringAppSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_MonitoringAppSpecification as MonitoringAppSpecification;
    impl crate::value::ToValue for MonitoringAppSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_arguments {
                properties.insert(
                    "ContainerArguments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_entrypoint {
                properties.insert(
                    "ContainerEntrypoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ImageUri".to_string(),
                crate::value::ToValue::to_value(&self.image_uri),
            );
            if let Some(ref value) = self.post_analytics_processor_source_uri {
                properties.insert(
                    "PostAnalyticsProcessorSourceUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.record_preprocessor_source_uri {
                properties.insert(
                    "RecordPreprocessorSourceUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringexecutionsummary.html
    pub struct MonitoringExecutionSummary_ {
        pub creation_time: crate::value::ExpString,
        pub endpoint_name: Option<crate::value::ExpString>,
        pub failure_reason: Option<crate::value::ExpString>,
        pub last_modified_time: crate::value::ExpString,
        pub monitoring_execution_status: crate::value::ExpString,
        pub monitoring_schedule_name: crate::value::ExpString,
        pub processing_job_arn: Option<crate::value::ExpString>,
        pub scheduled_time: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_MonitoringExecutionSummary {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.MonitoringExecutionSummary"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_MonitoringExecutionSummary as MonitoringExecutionSummary;
    impl crate::value::ToValue for MonitoringExecutionSummary_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CreationTime".to_string(),
                crate::value::ToValue::to_value(&self.creation_time),
            );
            if let Some(ref value) = self.endpoint_name {
                properties.insert(
                    "EndpointName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failure_reason {
                properties.insert(
                    "FailureReason".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LastModifiedTime".to_string(),
                crate::value::ToValue::to_value(&self.last_modified_time),
            );
            properties.insert(
                "MonitoringExecutionStatus".to_string(),
                crate::value::ToValue::to_value(&self.monitoring_execution_status),
            );
            properties.insert(
                "MonitoringScheduleName".to_string(),
                crate::value::ToValue::to_value(&self.monitoring_schedule_name),
            );
            if let Some(ref value) = self.processing_job_arn {
                properties.insert(
                    "ProcessingJobArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ScheduledTime".to_string(),
                crate::value::ToValue::to_value(&self.scheduled_time),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringinput.html
    pub struct MonitoringInput_ {
        pub batch_transform_input: Option<Box<BatchTransformInput_>>,
        pub endpoint_input: Option<Box<EndpointInput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_MonitoringInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.MonitoringInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_MonitoringInput as MonitoringInput;
    impl crate::value::ToValue for MonitoringInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_transform_input {
                properties.insert(
                    "BatchTransformInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.endpoint_input {
                properties.insert(
                    "EndpointInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringjobdefinition.html
    pub struct MonitoringJobDefinition_ {
        pub baseline_config: Option<Box<BaselineConfig_>>,
        pub environment: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub monitoring_app_specification: Box<MonitoringAppSpecification_>,
        pub monitoring_inputs: Vec<MonitoringInput_>,
        pub monitoring_output_config: Box<MonitoringOutputConfig_>,
        pub monitoring_resources: Box<MonitoringResources_>,
        pub network_config: Option<Box<NetworkConfig_>>,
        pub role_arn: crate::value::ExpString,
        pub stopping_condition: Option<Box<StoppingCondition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_MonitoringJobDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.MonitoringJobDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_MonitoringJobDefinition as MonitoringJobDefinition;
    impl crate::value::ToValue for MonitoringJobDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.baseline_config {
                properties.insert(
                    "BaselineConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MonitoringAppSpecification".to_string(),
                crate::value::ToValue::to_value(&self.monitoring_app_specification),
            );
            properties.insert(
                "MonitoringInputs".to_string(),
                crate::value::ToValue::to_value(&self.monitoring_inputs),
            );
            properties.insert(
                "MonitoringOutputConfig".to_string(),
                crate::value::ToValue::to_value(&self.monitoring_output_config),
            );
            properties.insert(
                "MonitoringResources".to_string(),
                crate::value::ToValue::to_value(&self.monitoring_resources),
            );
            if let Some(ref value) = self.network_config {
                properties.insert(
                    "NetworkConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.stopping_condition {
                properties.insert(
                    "StoppingCondition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringoutput.html
    pub struct MonitoringOutput_ {
        pub s3_output: Box<S3Output_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_MonitoringOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.MonitoringOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_MonitoringOutput as MonitoringOutput;
    impl crate::value::ToValue for MonitoringOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Output".to_string(),
                crate::value::ToValue::to_value(&self.s3_output),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringoutputconfig.html
    pub struct MonitoringOutputConfig_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub monitoring_outputs: Vec<MonitoringOutput_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_MonitoringOutputConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.MonitoringOutputConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_MonitoringOutputConfig as MonitoringOutputConfig;
    impl crate::value::ToValue for MonitoringOutputConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MonitoringOutputs".to_string(),
                crate::value::ToValue::to_value(&self.monitoring_outputs),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringresources.html
    pub struct MonitoringResources_ {
        pub cluster_config: Box<ClusterConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_MonitoringResources {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.MonitoringResources"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_MonitoringResources as MonitoringResources;
    impl crate::value::ToValue for MonitoringResources_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterConfig".to_string(),
                crate::value::ToValue::to_value(&self.cluster_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringscheduleconfig.html
    pub struct MonitoringScheduleConfig_ {
        pub monitoring_job_definition: Option<Box<MonitoringJobDefinition_>>,
        pub monitoring_job_definition_name: Option<crate::value::ExpString>,
        pub monitoring_type: Option<crate::value::ExpString>,
        pub schedule_config: Option<Box<ScheduleConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_MonitoringScheduleConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.MonitoringScheduleConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_MonitoringScheduleConfig as MonitoringScheduleConfig;
    impl crate::value::ToValue for MonitoringScheduleConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.monitoring_job_definition {
                properties.insert(
                    "MonitoringJobDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.monitoring_job_definition_name {
                properties.insert(
                    "MonitoringJobDefinitionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.monitoring_type {
                properties.insert(
                    "MonitoringType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schedule_config {
                properties.insert(
                    "ScheduleConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-networkconfig.html
    pub struct NetworkConfig_ {
        pub enable_inter_container_traffic_encryption: Option<crate::value::ExpBool>,
        pub enable_network_isolation: Option<crate::value::ExpBool>,
        pub vpc_config: Option<Box<VpcConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_NetworkConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.NetworkConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_NetworkConfig as NetworkConfig;
    impl crate::value::ToValue for NetworkConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_inter_container_traffic_encryption {
                properties.insert(
                    "EnableInterContainerTrafficEncryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_network_isolation {
                properties.insert(
                    "EnableNetworkIsolation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_config {
                properties.insert(
                    "VpcConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-s3output.html
    pub struct S3Output_ {
        pub local_path: crate::value::ExpString,
        pub s3_upload_mode: Option<crate::value::ExpString>,
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_S3Output {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.S3Output"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_S3Output as S3Output;
    impl crate::value::ToValue for S3Output_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LocalPath".to_string(),
                crate::value::ToValue::to_value(&self.local_path),
            );
            if let Some(ref value) = self.s3_upload_mode {
                properties.insert(
                    "S3UploadMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-scheduleconfig.html
    pub struct ScheduleConfig_ {
        pub data_analysis_end_time: Option<crate::value::ExpString>,
        pub data_analysis_start_time: Option<crate::value::ExpString>,
        pub schedule_expression: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_ScheduleConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.ScheduleConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_ScheduleConfig as ScheduleConfig;
    impl crate::value::ToValue for ScheduleConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_analysis_end_time {
                properties.insert(
                    "DataAnalysisEndTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_analysis_start_time {
                properties.insert(
                    "DataAnalysisStartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ScheduleExpression".to_string(),
                crate::value::ToValue::to_value(&self.schedule_expression),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-statisticsresource.html
    pub struct StatisticsResource_ {
        pub s3_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_StatisticsResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.StatisticsResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_StatisticsResource as StatisticsResource;
    impl crate::value::ToValue for StatisticsResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_uri {
                properties.insert("S3Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-stoppingcondition.html
    pub struct StoppingCondition_ {
        pub max_runtime_in_seconds: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_StoppingCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.StoppingCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_StoppingCondition as StoppingCondition;
    impl crate::value::ToValue for StoppingCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxRuntimeInSeconds".to_string(),
                crate::value::ToValue::to_value(&self.max_runtime_in_seconds),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-vpcconfig.html
    pub struct VpcConfig_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_MonitoringSchedule_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::MonitoringSchedule.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_MonitoringSchedule_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
}
pub mod notebookinstance {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-notebookinstance-instancemetadataserviceconfiguration.html
    pub struct InstanceMetadataServiceConfiguration_ {
        pub minimum_instance_metadata_service_version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_NotebookInstance_InstanceMetadataServiceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::NotebookInstance.InstanceMetadataServiceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_NotebookInstance_InstanceMetadataServiceConfiguration as InstanceMetadataServiceConfiguration;
    impl crate::value::ToValue for InstanceMetadataServiceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MinimumInstanceMetadataServiceVersion".to_string(),
                crate::value::ToValue::to_value(&self.minimum_instance_metadata_service_version),
            );
            properties.into()
        }
    }
}
pub mod notebookinstancelifecycleconfig {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-notebookinstancelifecycleconfig-notebookinstancelifecyclehook.html
    pub struct NotebookInstanceLifecycleHook_ {
        pub content: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_NotebookInstanceLifecycleConfig_NotebookInstanceLifecycleHook {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::NotebookInstanceLifecycleConfig.NotebookInstanceLifecycleHook"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_NotebookInstanceLifecycleConfig_NotebookInstanceLifecycleHook as NotebookInstanceLifecycleHook;
    impl crate::value::ToValue for NotebookInstanceLifecycleHook_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content {
                properties.insert(
                    "Content".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod partnerapp {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-partnerapp-partnerappconfig.html
    pub struct PartnerAppConfig_ {
        pub admin_users: Option<Vec<crate::value::ExpString>>,
        pub arguments: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_PartnerApp_PartnerAppConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::PartnerApp.PartnerAppConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_PartnerApp_PartnerAppConfig as PartnerAppConfig;
    impl crate::value::ToValue for PartnerAppConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.admin_users {
                properties.insert(
                    "AdminUsers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.arguments {
                properties.insert(
                    "Arguments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-partnerapp-partnerappmaintenanceconfig.html
    pub struct PartnerAppMaintenanceConfig_ {
        pub maintenance_window_start: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_PartnerApp_PartnerAppMaintenanceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::PartnerApp.PartnerAppMaintenanceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_PartnerApp_PartnerAppMaintenanceConfig as PartnerAppMaintenanceConfig;
    impl crate::value::ToValue for PartnerAppMaintenanceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaintenanceWindowStart".to_string(),
                crate::value::ToValue::to_value(&self.maintenance_window_start),
            );
            properties.into()
        }
    }
}
pub mod pipeline {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-pipeline-parallelismconfiguration.html
    pub struct ParallelismConfiguration_ {
        pub max_parallel_execution_steps: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Pipeline_ParallelismConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Pipeline.ParallelismConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Pipeline_ParallelismConfiguration as ParallelismConfiguration;
    impl crate::value::ToValue for ParallelismConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxParallelExecutionSteps".to_string(),
                crate::value::ToValue::to_value(&self.max_parallel_execution_steps),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-pipeline-pipelinedefinition.html
    pub struct PipelineDefinition_ {
        pub pipeline_definition_body: Option<crate::value::ExpString>,
        pub pipeline_definition_s3_location: Option<Box<S3Location_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Pipeline_PipelineDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Pipeline.PipelineDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Pipeline_PipelineDefinition as PipelineDefinition;
    impl crate::value::ToValue for PipelineDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.pipeline_definition_body {
                properties.insert(
                    "PipelineDefinitionBody".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pipeline_definition_s3_location {
                properties.insert(
                    "PipelineDefinitionS3Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-pipeline-s3location.html
    pub struct S3Location_ {
        pub bucket: crate::value::ExpString,
        pub e_tag: Option<crate::value::ExpString>,
        pub key: crate::value::ExpString,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Pipeline_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Pipeline.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Pipeline_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
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
}
pub mod processingjob {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-processingjob-appspecification.html
    pub struct AppSpecification_ {
        pub container_arguments: Option<Vec<crate::value::ExpString>>,
        pub container_entrypoint: Option<Vec<crate::value::ExpString>>,
        pub image_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ProcessingJob_AppSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ProcessingJob.AppSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ProcessingJob_AppSpecification as AppSpecification;
    impl crate::value::ToValue for AppSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_arguments {
                properties.insert(
                    "ContainerArguments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_entrypoint {
                properties.insert(
                    "ContainerEntrypoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ImageUri".to_string(),
                crate::value::ToValue::to_value(&self.image_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-processingjob-athenadatasetdefinition.html
    pub struct AthenaDatasetDefinition_ {
        pub catalog: crate::value::ExpString,
        pub database: crate::value::ExpString,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub output_compression: Option<crate::value::ExpString>,
        pub output_format: crate::value::ExpString,
        pub output_s3_uri: crate::value::ExpString,
        pub query_string: crate::value::ExpString,
        pub work_group: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ProcessingJob_AthenaDatasetDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ProcessingJob.AthenaDatasetDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ProcessingJob_AthenaDatasetDefinition as AthenaDatasetDefinition;
    impl crate::value::ToValue for AthenaDatasetDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Catalog".to_string(),
                crate::value::ToValue::to_value(&self.catalog),
            );
            properties.insert(
                "Database".to_string(),
                crate::value::ToValue::to_value(&self.database),
            );
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_compression {
                properties.insert(
                    "OutputCompression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OutputFormat".to_string(),
                crate::value::ToValue::to_value(&self.output_format),
            );
            properties.insert(
                "OutputS3Uri".to_string(),
                crate::value::ToValue::to_value(&self.output_s3_uri),
            );
            properties.insert(
                "QueryString".to_string(),
                crate::value::ToValue::to_value(&self.query_string),
            );
            if let Some(ref value) = self.work_group {
                properties.insert(
                    "WorkGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-processingjob-clusterconfig.html
    pub struct ClusterConfig_ {
        pub instance_count: i64,
        pub instance_type: crate::value::ExpString,
        pub volume_kms_key_id: Option<crate::value::ExpString>,
        pub volume_size_in_gb: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ProcessingJob_ClusterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ProcessingJob.ClusterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ProcessingJob_ClusterConfig as ClusterConfig;
    impl crate::value::ToValue for ClusterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceCount".to_string(),
                crate::value::ToValue::to_value(&self.instance_count),
            );
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            if let Some(ref value) = self.volume_kms_key_id {
                properties.insert(
                    "VolumeKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VolumeSizeInGB".to_string(),
                crate::value::ToValue::to_value(&self.volume_size_in_gb),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-processingjob-datasetdefinition.html
    pub struct DatasetDefinition_ {
        pub athena_dataset_definition: Option<Box<AthenaDatasetDefinition_>>,
        pub data_distribution_type: Option<crate::value::ExpString>,
        pub input_mode: Option<crate::value::ExpString>,
        pub local_path: Option<crate::value::ExpString>,
        pub redshift_dataset_definition: Option<Box<RedshiftDatasetDefinition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ProcessingJob_DatasetDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ProcessingJob.DatasetDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ProcessingJob_DatasetDefinition as DatasetDefinition;
    impl crate::value::ToValue for DatasetDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.athena_dataset_definition {
                properties.insert(
                    "AthenaDatasetDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_distribution_type {
                properties.insert(
                    "DataDistributionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_mode {
                properties.insert(
                    "InputMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.local_path {
                properties.insert(
                    "LocalPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redshift_dataset_definition {
                properties.insert(
                    "RedshiftDatasetDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-processingjob-experimentconfig.html
    pub struct ExperimentConfig_ {
        pub experiment_name: Option<crate::value::ExpString>,
        pub run_name: Option<crate::value::ExpString>,
        pub trial_component_display_name: Option<crate::value::ExpString>,
        pub trial_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ProcessingJob_ExperimentConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ProcessingJob.ExperimentConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ProcessingJob_ExperimentConfig as ExperimentConfig;
    impl crate::value::ToValue for ExperimentConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.experiment_name {
                properties.insert(
                    "ExperimentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.run_name {
                properties.insert(
                    "RunName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.trial_component_display_name {
                properties.insert(
                    "TrialComponentDisplayName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.trial_name {
                properties.insert(
                    "TrialName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-processingjob-featurestoreoutput.html
    pub struct FeatureStoreOutput_ {
        pub feature_group_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ProcessingJob_FeatureStoreOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ProcessingJob.FeatureStoreOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ProcessingJob_FeatureStoreOutput as FeatureStoreOutput;
    impl crate::value::ToValue for FeatureStoreOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FeatureGroupName".to_string(),
                crate::value::ToValue::to_value(&self.feature_group_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-processingjob-networkconfig.html
    pub struct NetworkConfig_ {
        pub enable_inter_container_traffic_encryption: Option<crate::value::ExpBool>,
        pub enable_network_isolation: Option<crate::value::ExpBool>,
        pub vpc_config: Option<Box<VpcConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ProcessingJob_NetworkConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ProcessingJob.NetworkConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ProcessingJob_NetworkConfig as NetworkConfig;
    impl crate::value::ToValue for NetworkConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_inter_container_traffic_encryption {
                properties.insert(
                    "EnableInterContainerTrafficEncryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_network_isolation {
                properties.insert(
                    "EnableNetworkIsolation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_config {
                properties.insert(
                    "VpcConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-processingjob-processinginputsobject.html
    pub struct ProcessingInputsObject_ {
        pub app_managed: Option<crate::value::ExpBool>,
        pub dataset_definition: Option<Box<DatasetDefinition_>>,
        pub input_name: crate::value::ExpString,
        pub s3_input: Option<Box<S3Input_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ProcessingJob_ProcessingInputsObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ProcessingJob.ProcessingInputsObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ProcessingJob_ProcessingInputsObject as ProcessingInputsObject;
    impl crate::value::ToValue for ProcessingInputsObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_managed {
                properties.insert(
                    "AppManaged".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dataset_definition {
                properties.insert(
                    "DatasetDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InputName".to_string(),
                crate::value::ToValue::to_value(&self.input_name),
            );
            if let Some(ref value) = self.s3_input {
                properties.insert(
                    "S3Input".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-processingjob-processingoutputconfig.html
    pub struct ProcessingOutputConfig_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub outputs: Vec<ProcessingOutputsObject_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ProcessingJob_ProcessingOutputConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ProcessingJob.ProcessingOutputConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ProcessingJob_ProcessingOutputConfig as ProcessingOutputConfig;
    impl crate::value::ToValue for ProcessingOutputConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Outputs".to_string(),
                crate::value::ToValue::to_value(&self.outputs),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-processingjob-processingoutputsobject.html
    pub struct ProcessingOutputsObject_ {
        pub app_managed: Option<crate::value::ExpBool>,
        pub feature_store_output: Option<Box<FeatureStoreOutput_>>,
        pub output_name: crate::value::ExpString,
        pub s3_output: Option<Box<S3Output_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ProcessingJob_ProcessingOutputsObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ProcessingJob.ProcessingOutputsObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ProcessingJob_ProcessingOutputsObject as ProcessingOutputsObject;
    impl crate::value::ToValue for ProcessingOutputsObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_managed {
                properties.insert(
                    "AppManaged".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.feature_store_output {
                properties.insert(
                    "FeatureStoreOutput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OutputName".to_string(),
                crate::value::ToValue::to_value(&self.output_name),
            );
            if let Some(ref value) = self.s3_output {
                properties.insert(
                    "S3Output".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-processingjob-processingresources.html
    pub struct ProcessingResources_ {
        pub cluster_config: Box<ClusterConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ProcessingJob_ProcessingResources {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ProcessingJob.ProcessingResources"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ProcessingJob_ProcessingResources as ProcessingResources;
    impl crate::value::ToValue for ProcessingResources_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterConfig".to_string(),
                crate::value::ToValue::to_value(&self.cluster_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-processingjob-redshiftdatasetdefinition.html
    pub struct RedshiftDatasetDefinition_ {
        pub cluster_id: crate::value::ExpString,
        pub cluster_role_arn: crate::value::ExpString,
        pub database: crate::value::ExpString,
        pub db_user: crate::value::ExpString,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub output_compression: Option<crate::value::ExpString>,
        pub output_format: crate::value::ExpString,
        pub output_s3_uri: crate::value::ExpString,
        pub query_string: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ProcessingJob_RedshiftDatasetDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ProcessingJob.RedshiftDatasetDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ProcessingJob_RedshiftDatasetDefinition as RedshiftDatasetDefinition;
    impl crate::value::ToValue for RedshiftDatasetDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterId".to_string(),
                crate::value::ToValue::to_value(&self.cluster_id),
            );
            properties.insert(
                "ClusterRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.cluster_role_arn),
            );
            properties.insert(
                "Database".to_string(),
                crate::value::ToValue::to_value(&self.database),
            );
            properties.insert(
                "DbUser".to_string(),
                crate::value::ToValue::to_value(&self.db_user),
            );
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_compression {
                properties.insert(
                    "OutputCompression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OutputFormat".to_string(),
                crate::value::ToValue::to_value(&self.output_format),
            );
            properties.insert(
                "OutputS3Uri".to_string(),
                crate::value::ToValue::to_value(&self.output_s3_uri),
            );
            properties.insert(
                "QueryString".to_string(),
                crate::value::ToValue::to_value(&self.query_string),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-processingjob-s3input.html
    pub struct S3Input_ {
        pub local_path: Option<crate::value::ExpString>,
        pub s3_compression_type: Option<crate::value::ExpString>,
        pub s3_data_distribution_type: Option<crate::value::ExpString>,
        pub s3_data_type: crate::value::ExpString,
        pub s3_input_mode: Option<crate::value::ExpString>,
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ProcessingJob_S3Input {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ProcessingJob.S3Input"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ProcessingJob_S3Input as S3Input;
    impl crate::value::ToValue for S3Input_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.local_path {
                properties.insert(
                    "LocalPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_compression_type {
                properties.insert(
                    "S3CompressionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_data_distribution_type {
                properties.insert(
                    "S3DataDistributionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3DataType".to_string(),
                crate::value::ToValue::to_value(&self.s3_data_type),
            );
            if let Some(ref value) = self.s3_input_mode {
                properties.insert(
                    "S3InputMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-processingjob-s3output.html
    pub struct S3Output_ {
        pub local_path: Option<crate::value::ExpString>,
        pub s3_upload_mode: crate::value::ExpString,
        pub s3_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ProcessingJob_S3Output {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ProcessingJob.S3Output"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ProcessingJob_S3Output as S3Output;
    impl crate::value::ToValue for S3Output_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.local_path {
                properties.insert(
                    "LocalPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3UploadMode".to_string(),
                crate::value::ToValue::to_value(&self.s3_upload_mode),
            );
            properties.insert(
                "S3Uri".to_string(),
                crate::value::ToValue::to_value(&self.s3_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-processingjob-stoppingcondition.html
    pub struct StoppingCondition_ {
        pub max_runtime_in_seconds: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ProcessingJob_StoppingCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ProcessingJob.StoppingCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ProcessingJob_StoppingCondition as StoppingCondition;
    impl crate::value::ToValue for StoppingCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxRuntimeInSeconds".to_string(),
                crate::value::ToValue::to_value(&self.max_runtime_in_seconds),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-processingjob-vpcconfig.html
    pub struct VpcConfig_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_ProcessingJob_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::ProcessingJob.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_ProcessingJob_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
}
pub mod project {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-project-cfnstackparameter.html
    pub struct CfnStackParameter_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Project_CfnStackParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Project.CfnStackParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Project_CfnStackParameter as CfnStackParameter;
    impl crate::value::ToValue for CfnStackParameter_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-project-cfntemplateproviderdetail.html
    pub struct CfnTemplateProviderDetail_ {
        pub parameters: Option<Vec<CfnStackParameter_>>,
        pub role_arn: Option<crate::value::ExpString>,
        pub template_name: crate::value::ExpString,
        pub template_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Project_CfnTemplateProviderDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Project.CfnTemplateProviderDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Project_CfnTemplateProviderDetail as CfnTemplateProviderDetail;
    impl crate::value::ToValue for CfnTemplateProviderDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TemplateName".to_string(),
                crate::value::ToValue::to_value(&self.template_name),
            );
            properties.insert(
                "TemplateURL".to_string(),
                crate::value::ToValue::to_value(&self.template_url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-project-provisioningparameter.html
    pub struct ProvisioningParameter_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Project_ProvisioningParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Project.ProvisioningParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Project_ProvisioningParameter as ProvisioningParameter;
    impl crate::value::ToValue for ProvisioningParameter_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-project-servicecatalogprovisionedproductdetails.html
    pub struct ServiceCatalogProvisionedProductDetails_ {
        pub provisioned_product_id: Option<crate::value::ExpString>,
        pub provisioned_product_status_message: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Project_ServiceCatalogProvisionedProductDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Project.ServiceCatalogProvisionedProductDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Project_ServiceCatalogProvisionedProductDetails as ServiceCatalogProvisionedProductDetails;
    impl crate::value::ToValue for ServiceCatalogProvisionedProductDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.provisioned_product_id {
                properties.insert(
                    "ProvisionedProductId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.provisioned_product_status_message {
                properties.insert(
                    "ProvisionedProductStatusMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-project-servicecatalogprovisioningdetails.html
    pub struct ServiceCatalogProvisioningDetails_ {
        pub path_id: Option<crate::value::ExpString>,
        pub product_id: crate::value::ExpString,
        pub provisioning_artifact_id: Option<crate::value::ExpString>,
        pub provisioning_parameters: Option<Vec<ProvisioningParameter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Project_ServiceCatalogProvisioningDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Project.ServiceCatalogProvisioningDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Project_ServiceCatalogProvisioningDetails as ServiceCatalogProvisioningDetails;
    impl crate::value::ToValue for ServiceCatalogProvisioningDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.path_id {
                properties.insert("PathId".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "ProductId".to_string(),
                crate::value::ToValue::to_value(&self.product_id),
            );
            if let Some(ref value) = self.provisioning_artifact_id {
                properties.insert(
                    "ProvisioningArtifactId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.provisioning_parameters {
                properties.insert(
                    "ProvisioningParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-project-templateproviderdetail.html
    pub struct TemplateProviderDetail_ {
        pub cfn_template_provider_detail: Box<CfnTemplateProviderDetail_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Project_TemplateProviderDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Project.TemplateProviderDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Project_TemplateProviderDetail as TemplateProviderDetail;
    impl crate::value::ToValue for TemplateProviderDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CfnTemplateProviderDetail".to_string(),
                crate::value::ToValue::to_value(&self.cfn_template_provider_detail),
            );
            properties.into()
        }
    }
}
pub mod space {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-coderepository.html
    pub struct CodeRepository_ {
        pub repository_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_CodeRepository {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.CodeRepository"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_CodeRepository as CodeRepository;
    impl crate::value::ToValue for CodeRepository_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RepositoryUrl".to_string(),
                crate::value::ToValue::to_value(&self.repository_url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-customfilesystem.html
    pub struct CustomFileSystem_ {
        pub efs_file_system: Option<Box<EFSFileSystem_>>,
        pub f_sx_lustre_file_system: Option<Box<FSxLustreFileSystem_>>,
        pub s3_file_system: Option<Box<S3FileSystem_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_CustomFileSystem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.CustomFileSystem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_CustomFileSystem as CustomFileSystem;
    impl crate::value::ToValue for CustomFileSystem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.efs_file_system {
                properties.insert(
                    "EFSFileSystem".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.f_sx_lustre_file_system {
                properties.insert(
                    "FSxLustreFileSystem".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_file_system {
                properties.insert(
                    "S3FileSystem".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-customimage.html
    pub struct CustomImage_ {
        pub app_image_config_name: crate::value::ExpString,
        pub image_name: crate::value::ExpString,
        pub image_version_number: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_CustomImage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.CustomImage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_CustomImage as CustomImage;
    impl crate::value::ToValue for CustomImage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppImageConfigName".to_string(),
                crate::value::ToValue::to_value(&self.app_image_config_name),
            );
            properties.insert(
                "ImageName".to_string(),
                crate::value::ToValue::to_value(&self.image_name),
            );
            if let Some(ref value) = self.image_version_number {
                properties.insert(
                    "ImageVersionNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-efsfilesystem.html
    pub struct EFSFileSystem_ {
        pub file_system_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_EFSFileSystem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.EFSFileSystem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_EFSFileSystem as EFSFileSystem;
    impl crate::value::ToValue for EFSFileSystem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FileSystemId".to_string(),
                crate::value::ToValue::to_value(&self.file_system_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-ebsstoragesettings.html
    pub struct EbsStorageSettings_ {
        pub ebs_volume_size_in_gb: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_EbsStorageSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.EbsStorageSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_EbsStorageSettings as EbsStorageSettings;
    impl crate::value::ToValue for EbsStorageSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EbsVolumeSizeInGb".to_string(),
                crate::value::ToValue::to_value(&self.ebs_volume_size_in_gb),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-fsxlustrefilesystem.html
    pub struct FSxLustreFileSystem_ {
        pub file_system_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_FSxLustreFileSystem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.FSxLustreFileSystem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_FSxLustreFileSystem as FSxLustreFileSystem;
    impl crate::value::ToValue for FSxLustreFileSystem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FileSystemId".to_string(),
                crate::value::ToValue::to_value(&self.file_system_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-jupyterserverappsettings.html
    pub struct JupyterServerAppSettings_ {
        pub default_resource_spec: Option<Box<ResourceSpec_>>,
        pub lifecycle_config_arns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_JupyterServerAppSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.JupyterServerAppSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_JupyterServerAppSettings as JupyterServerAppSettings;
    impl crate::value::ToValue for JupyterServerAppSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_resource_spec {
                properties.insert(
                    "DefaultResourceSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_config_arns {
                properties.insert(
                    "LifecycleConfigArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-kernelgatewayappsettings.html
    pub struct KernelGatewayAppSettings_ {
        pub custom_images: Option<Vec<CustomImage_>>,
        pub default_resource_spec: Option<Box<ResourceSpec_>>,
        pub lifecycle_config_arns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_KernelGatewayAppSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.KernelGatewayAppSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_KernelGatewayAppSettings as KernelGatewayAppSettings;
    impl crate::value::ToValue for KernelGatewayAppSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_images {
                properties.insert(
                    "CustomImages".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_resource_spec {
                properties.insert(
                    "DefaultResourceSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_config_arns {
                properties.insert(
                    "LifecycleConfigArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-ownershipsettings.html
    pub struct OwnershipSettings_ {
        pub owner_user_profile_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_OwnershipSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.OwnershipSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_OwnershipSettings as OwnershipSettings;
    impl crate::value::ToValue for OwnershipSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OwnerUserProfileName".to_string(),
                crate::value::ToValue::to_value(&self.owner_user_profile_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-resourcespec.html
    pub struct ResourceSpec_ {
        pub instance_type: Option<crate::value::ExpString>,
        pub lifecycle_config_arn: Option<crate::value::ExpString>,
        pub sage_maker_image_arn: Option<crate::value::ExpString>,
        pub sage_maker_image_version_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_ResourceSpec {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.ResourceSpec"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_ResourceSpec as ResourceSpec;
    impl crate::value::ToValue for ResourceSpec_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instance_type {
                properties.insert(
                    "InstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_config_arn {
                properties.insert(
                    "LifecycleConfigArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sage_maker_image_arn {
                properties.insert(
                    "SageMakerImageArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sage_maker_image_version_arn {
                properties.insert(
                    "SageMakerImageVersionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-s3filesystem.html
    pub struct S3FileSystem_ {
        pub s3_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_S3FileSystem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.S3FileSystem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_S3FileSystem as S3FileSystem;
    impl crate::value::ToValue for S3FileSystem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_uri {
                properties.insert("S3Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-spaceapplifecyclemanagement.html
    pub struct SpaceAppLifecycleManagement_ {
        pub idle_settings: Option<Box<SpaceIdleSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_SpaceAppLifecycleManagement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.SpaceAppLifecycleManagement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_SpaceAppLifecycleManagement as SpaceAppLifecycleManagement;
    impl crate::value::ToValue for SpaceAppLifecycleManagement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idle_settings {
                properties.insert(
                    "IdleSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-spacecodeeditorappsettings.html
    pub struct SpaceCodeEditorAppSettings_ {
        pub app_lifecycle_management: Option<Box<SpaceAppLifecycleManagement_>>,
        pub default_resource_spec: Option<Box<ResourceSpec_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_SpaceCodeEditorAppSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.SpaceCodeEditorAppSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_SpaceCodeEditorAppSettings as SpaceCodeEditorAppSettings;
    impl crate::value::ToValue for SpaceCodeEditorAppSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_lifecycle_management {
                properties.insert(
                    "AppLifecycleManagement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_resource_spec {
                properties.insert(
                    "DefaultResourceSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-spaceidlesettings.html
    pub struct SpaceIdleSettings_ {
        pub idle_timeout_in_minutes: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_SpaceIdleSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.SpaceIdleSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_SpaceIdleSettings as SpaceIdleSettings;
    impl crate::value::ToValue for SpaceIdleSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idle_timeout_in_minutes {
                properties.insert(
                    "IdleTimeoutInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-spacejupyterlabappsettings.html
    pub struct SpaceJupyterLabAppSettings_ {
        pub app_lifecycle_management: Option<Box<SpaceAppLifecycleManagement_>>,
        pub code_repositories: Option<Vec<CodeRepository_>>,
        pub default_resource_spec: Option<Box<ResourceSpec_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_SpaceJupyterLabAppSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.SpaceJupyterLabAppSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_SpaceJupyterLabAppSettings as SpaceJupyterLabAppSettings;
    impl crate::value::ToValue for SpaceJupyterLabAppSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_lifecycle_management {
                properties.insert(
                    "AppLifecycleManagement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.code_repositories {
                properties.insert(
                    "CodeRepositories".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_resource_spec {
                properties.insert(
                    "DefaultResourceSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-spacesettings.html
    pub struct SpaceSettings_ {
        pub app_type: Option<crate::value::ExpString>,
        pub code_editor_app_settings: Option<Box<SpaceCodeEditorAppSettings_>>,
        pub custom_file_systems: Option<Vec<CustomFileSystem_>>,
        pub jupyter_lab_app_settings: Option<Box<SpaceJupyterLabAppSettings_>>,
        pub jupyter_server_app_settings: Option<Box<JupyterServerAppSettings_>>,
        pub kernel_gateway_app_settings: Option<Box<KernelGatewayAppSettings_>>,
        pub remote_access: Option<crate::value::ExpString>,
        pub space_managed_resources: Option<crate::value::ExpString>,
        pub space_storage_settings: Option<Box<SpaceStorageSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_SpaceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.SpaceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_SpaceSettings as SpaceSettings;
    impl crate::value::ToValue for SpaceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_type {
                properties.insert(
                    "AppType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.code_editor_app_settings {
                properties.insert(
                    "CodeEditorAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_file_systems {
                properties.insert(
                    "CustomFileSystems".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.jupyter_lab_app_settings {
                properties.insert(
                    "JupyterLabAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.jupyter_server_app_settings {
                properties.insert(
                    "JupyterServerAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kernel_gateway_app_settings {
                properties.insert(
                    "KernelGatewayAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remote_access {
                properties.insert(
                    "RemoteAccess".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.space_managed_resources {
                properties.insert(
                    "SpaceManagedResources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.space_storage_settings {
                properties.insert(
                    "SpaceStorageSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-spacesharingsettings.html
    pub struct SpaceSharingSettings_ {
        pub sharing_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_SpaceSharingSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.SpaceSharingSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_SpaceSharingSettings as SpaceSharingSettings;
    impl crate::value::ToValue for SpaceSharingSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SharingType".to_string(),
                crate::value::ToValue::to_value(&self.sharing_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-space-spacestoragesettings.html
    pub struct SpaceStorageSettings_ {
        pub ebs_storage_settings: Option<Box<EbsStorageSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Space_SpaceStorageSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Space.SpaceStorageSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Space_SpaceStorageSettings as SpaceStorageSettings;
    impl crate::value::ToValue for SpaceStorageSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ebs_storage_settings {
                properties.insert(
                    "EbsStorageSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod userprofile {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-applifecyclemanagement.html
    pub struct AppLifecycleManagement_ {
        pub idle_settings: Option<Box<IdleSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_AppLifecycleManagement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.AppLifecycleManagement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_AppLifecycleManagement as AppLifecycleManagement;
    impl crate::value::ToValue for AppLifecycleManagement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idle_settings {
                properties.insert(
                    "IdleSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-codeeditorappsettings.html
    pub struct CodeEditorAppSettings_ {
        pub app_lifecycle_management: Option<Box<AppLifecycleManagement_>>,
        pub built_in_lifecycle_config_arn: Option<crate::value::ExpString>,
        pub custom_images: Option<Vec<CustomImage_>>,
        pub default_resource_spec: Option<Box<ResourceSpec_>>,
        pub lifecycle_config_arns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_CodeEditorAppSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.CodeEditorAppSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_CodeEditorAppSettings as CodeEditorAppSettings;
    impl crate::value::ToValue for CodeEditorAppSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_lifecycle_management {
                properties.insert(
                    "AppLifecycleManagement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.built_in_lifecycle_config_arn {
                properties.insert(
                    "BuiltInLifecycleConfigArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_images {
                properties.insert(
                    "CustomImages".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_resource_spec {
                properties.insert(
                    "DefaultResourceSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_config_arns {
                properties.insert(
                    "LifecycleConfigArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-coderepository.html
    pub struct CodeRepository_ {
        pub repository_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_CodeRepository {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.CodeRepository"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_CodeRepository as CodeRepository;
    impl crate::value::ToValue for CodeRepository_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RepositoryUrl".to_string(),
                crate::value::ToValue::to_value(&self.repository_url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-customfilesystemconfig.html
    pub struct CustomFileSystemConfig_ {
        pub efs_file_system_config: Option<Box<EFSFileSystemConfig_>>,
        pub f_sx_lustre_file_system_config: Option<Box<FSxLustreFileSystemConfig_>>,
        pub s3_file_system_config: Option<Box<S3FileSystemConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_CustomFileSystemConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.CustomFileSystemConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_CustomFileSystemConfig as CustomFileSystemConfig;
    impl crate::value::ToValue for CustomFileSystemConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.efs_file_system_config {
                properties.insert(
                    "EFSFileSystemConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.f_sx_lustre_file_system_config {
                properties.insert(
                    "FSxLustreFileSystemConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_file_system_config {
                properties.insert(
                    "S3FileSystemConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-customimage.html
    pub struct CustomImage_ {
        pub app_image_config_name: crate::value::ExpString,
        pub image_name: crate::value::ExpString,
        pub image_version_number: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_CustomImage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.CustomImage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_CustomImage as CustomImage;
    impl crate::value::ToValue for CustomImage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppImageConfigName".to_string(),
                crate::value::ToValue::to_value(&self.app_image_config_name),
            );
            properties.insert(
                "ImageName".to_string(),
                crate::value::ToValue::to_value(&self.image_name),
            );
            if let Some(ref value) = self.image_version_number {
                properties.insert(
                    "ImageVersionNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-customposixuserconfig.html
    pub struct CustomPosixUserConfig_ {
        pub gid: i64,
        pub uid: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_CustomPosixUserConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.CustomPosixUserConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_CustomPosixUserConfig as CustomPosixUserConfig;
    impl crate::value::ToValue for CustomPosixUserConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Gid".to_string(),
                crate::value::ToValue::to_value(&self.gid),
            );
            properties.insert(
                "Uid".to_string(),
                crate::value::ToValue::to_value(&self.uid),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-defaultebsstoragesettings.html
    pub struct DefaultEbsStorageSettings_ {
        pub default_ebs_volume_size_in_gb: i64,
        pub maximum_ebs_volume_size_in_gb: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_DefaultEbsStorageSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.DefaultEbsStorageSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_DefaultEbsStorageSettings as DefaultEbsStorageSettings;
    impl crate::value::ToValue for DefaultEbsStorageSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DefaultEbsVolumeSizeInGb".to_string(),
                crate::value::ToValue::to_value(&self.default_ebs_volume_size_in_gb),
            );
            properties.insert(
                "MaximumEbsVolumeSizeInGb".to_string(),
                crate::value::ToValue::to_value(&self.maximum_ebs_volume_size_in_gb),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-defaultspacestoragesettings.html
    pub struct DefaultSpaceStorageSettings_ {
        pub default_ebs_storage_settings: Option<Box<DefaultEbsStorageSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_DefaultSpaceStorageSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.DefaultSpaceStorageSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_DefaultSpaceStorageSettings as DefaultSpaceStorageSettings;
    impl crate::value::ToValue for DefaultSpaceStorageSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_ebs_storage_settings {
                properties.insert(
                    "DefaultEbsStorageSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-efsfilesystemconfig.html
    pub struct EFSFileSystemConfig_ {
        pub file_system_id: crate::value::ExpString,
        pub file_system_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_EFSFileSystemConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.EFSFileSystemConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_EFSFileSystemConfig as EFSFileSystemConfig;
    impl crate::value::ToValue for EFSFileSystemConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FileSystemId".to_string(),
                crate::value::ToValue::to_value(&self.file_system_id),
            );
            if let Some(ref value) = self.file_system_path {
                properties.insert(
                    "FileSystemPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-fsxlustrefilesystemconfig.html
    pub struct FSxLustreFileSystemConfig_ {
        pub file_system_id: crate::value::ExpString,
        pub file_system_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_FSxLustreFileSystemConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.FSxLustreFileSystemConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_FSxLustreFileSystemConfig as FSxLustreFileSystemConfig;
    impl crate::value::ToValue for FSxLustreFileSystemConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FileSystemId".to_string(),
                crate::value::ToValue::to_value(&self.file_system_id),
            );
            if let Some(ref value) = self.file_system_path {
                properties.insert(
                    "FileSystemPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-hiddensagemakerimage.html
    pub struct HiddenSageMakerImage_ {
        pub sage_maker_image_name: Option<crate::value::ExpString>,
        pub version_aliases: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_HiddenSageMakerImage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.HiddenSageMakerImage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_HiddenSageMakerImage as HiddenSageMakerImage;
    impl crate::value::ToValue for HiddenSageMakerImage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sage_maker_image_name {
                properties.insert(
                    "SageMakerImageName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.version_aliases {
                properties.insert(
                    "VersionAliases".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-idlesettings.html
    pub struct IdleSettings_ {
        pub idle_timeout_in_minutes: Option<i64>,
        pub lifecycle_management: Option<crate::value::ExpString>,
        pub max_idle_timeout_in_minutes: Option<i64>,
        pub min_idle_timeout_in_minutes: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_IdleSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.IdleSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_IdleSettings as IdleSettings;
    impl crate::value::ToValue for IdleSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idle_timeout_in_minutes {
                properties.insert(
                    "IdleTimeoutInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_management {
                properties.insert(
                    "LifecycleManagement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_idle_timeout_in_minutes {
                properties.insert(
                    "MaxIdleTimeoutInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_idle_timeout_in_minutes {
                properties.insert(
                    "MinIdleTimeoutInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-jupyterlabappsettings.html
    pub struct JupyterLabAppSettings_ {
        pub app_lifecycle_management: Option<Box<AppLifecycleManagement_>>,
        pub built_in_lifecycle_config_arn: Option<crate::value::ExpString>,
        pub code_repositories: Option<Vec<CodeRepository_>>,
        pub custom_images: Option<Vec<CustomImage_>>,
        pub default_resource_spec: Option<Box<ResourceSpec_>>,
        pub lifecycle_config_arns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_JupyterLabAppSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.JupyterLabAppSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_JupyterLabAppSettings as JupyterLabAppSettings;
    impl crate::value::ToValue for JupyterLabAppSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_lifecycle_management {
                properties.insert(
                    "AppLifecycleManagement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.built_in_lifecycle_config_arn {
                properties.insert(
                    "BuiltInLifecycleConfigArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.code_repositories {
                properties.insert(
                    "CodeRepositories".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_images {
                properties.insert(
                    "CustomImages".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_resource_spec {
                properties.insert(
                    "DefaultResourceSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_config_arns {
                properties.insert(
                    "LifecycleConfigArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-jupyterserverappsettings.html
    pub struct JupyterServerAppSettings_ {
        pub default_resource_spec: Option<Box<ResourceSpec_>>,
        pub lifecycle_config_arns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_JupyterServerAppSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.JupyterServerAppSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_JupyterServerAppSettings as JupyterServerAppSettings;
    impl crate::value::ToValue for JupyterServerAppSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_resource_spec {
                properties.insert(
                    "DefaultResourceSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_config_arns {
                properties.insert(
                    "LifecycleConfigArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-kernelgatewayappsettings.html
    pub struct KernelGatewayAppSettings_ {
        pub custom_images: Option<Vec<CustomImage_>>,
        pub default_resource_spec: Option<Box<ResourceSpec_>>,
        pub lifecycle_config_arns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_KernelGatewayAppSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.KernelGatewayAppSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_KernelGatewayAppSettings as KernelGatewayAppSettings;
    impl crate::value::ToValue for KernelGatewayAppSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_images {
                properties.insert(
                    "CustomImages".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_resource_spec {
                properties.insert(
                    "DefaultResourceSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_config_arns {
                properties.insert(
                    "LifecycleConfigArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-rstudioserverproappsettings.html
    pub struct RStudioServerProAppSettings_ {
        pub access_status: Option<crate::value::ExpString>,
        pub user_group: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_RStudioServerProAppSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.RStudioServerProAppSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_RStudioServerProAppSettings as RStudioServerProAppSettings;
    impl crate::value::ToValue for RStudioServerProAppSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_status {
                properties.insert(
                    "AccessStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_group {
                properties.insert(
                    "UserGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-resourcespec.html
    pub struct ResourceSpec_ {
        pub instance_type: Option<crate::value::ExpString>,
        pub lifecycle_config_arn: Option<crate::value::ExpString>,
        pub sage_maker_image_arn: Option<crate::value::ExpString>,
        pub sage_maker_image_version_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_ResourceSpec {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.ResourceSpec"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_ResourceSpec as ResourceSpec;
    impl crate::value::ToValue for ResourceSpec_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instance_type {
                properties.insert(
                    "InstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_config_arn {
                properties.insert(
                    "LifecycleConfigArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sage_maker_image_arn {
                properties.insert(
                    "SageMakerImageArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sage_maker_image_version_arn {
                properties.insert(
                    "SageMakerImageVersionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-s3filesystemconfig.html
    pub struct S3FileSystemConfig_ {
        pub mount_path: Option<crate::value::ExpString>,
        pub s3_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_S3FileSystemConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.S3FileSystemConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_S3FileSystemConfig as S3FileSystemConfig;
    impl crate::value::ToValue for S3FileSystemConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mount_path {
                properties.insert(
                    "MountPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_uri {
                properties.insert("S3Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-sharingsettings.html
    pub struct SharingSettings_ {
        pub notebook_output_option: Option<crate::value::ExpString>,
        pub s3_kms_key_id: Option<crate::value::ExpString>,
        pub s3_output_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_SharingSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.SharingSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_SharingSettings as SharingSettings;
    impl crate::value::ToValue for SharingSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.notebook_output_option {
                properties.insert(
                    "NotebookOutputOption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_kms_key_id {
                properties.insert(
                    "S3KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_output_path {
                properties.insert(
                    "S3OutputPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-studiowebportalsettings.html
    pub struct StudioWebPortalSettings_ {
        pub hidden_app_types: Option<Vec<crate::value::ExpString>>,
        pub hidden_instance_types: Option<Vec<crate::value::ExpString>>,
        pub hidden_ml_tools: Option<Vec<crate::value::ExpString>>,
        pub hidden_sage_maker_image_version_aliases: Option<Vec<HiddenSageMakerImage_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_StudioWebPortalSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.StudioWebPortalSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_StudioWebPortalSettings as StudioWebPortalSettings;
    impl crate::value::ToValue for StudioWebPortalSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hidden_app_types {
                properties.insert(
                    "HiddenAppTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hidden_instance_types {
                properties.insert(
                    "HiddenInstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hidden_ml_tools {
                properties.insert(
                    "HiddenMlTools".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hidden_sage_maker_image_version_aliases {
                properties.insert(
                    "HiddenSageMakerImageVersionAliases".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-usersettings.html
    pub struct UserSettings_ {
        pub auto_mount_home_efs: Option<crate::value::ExpString>,
        pub code_editor_app_settings: Option<Box<CodeEditorAppSettings_>>,
        pub custom_file_system_configs: Option<Vec<CustomFileSystemConfig_>>,
        pub custom_posix_user_config: Option<Box<CustomPosixUserConfig_>>,
        pub default_landing_uri: Option<crate::value::ExpString>,
        pub execution_role: Option<crate::value::ExpString>,
        pub jupyter_lab_app_settings: Option<Box<JupyterLabAppSettings_>>,
        pub jupyter_server_app_settings: Option<Box<JupyterServerAppSettings_>>,
        pub kernel_gateway_app_settings: Option<Box<KernelGatewayAppSettings_>>,
        pub r_studio_server_pro_app_settings: Option<Box<RStudioServerProAppSettings_>>,
        pub security_groups: Option<Vec<crate::value::ExpString>>,
        pub sharing_settings: Option<Box<SharingSettings_>>,
        pub space_storage_settings: Option<Box<DefaultSpaceStorageSettings_>>,
        pub studio_web_portal: Option<crate::value::ExpString>,
        pub studio_web_portal_settings: Option<Box<StudioWebPortalSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_UserProfile_UserSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::UserProfile.UserSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_UserProfile_UserSettings as UserSettings;
    impl crate::value::ToValue for UserSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_mount_home_efs {
                properties.insert(
                    "AutoMountHomeEFS".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.code_editor_app_settings {
                properties.insert(
                    "CodeEditorAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_file_system_configs {
                properties.insert(
                    "CustomFileSystemConfigs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_posix_user_config {
                properties.insert(
                    "CustomPosixUserConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_landing_uri {
                properties.insert(
                    "DefaultLandingUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execution_role {
                properties.insert(
                    "ExecutionRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.jupyter_lab_app_settings {
                properties.insert(
                    "JupyterLabAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.jupyter_server_app_settings {
                properties.insert(
                    "JupyterServerAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kernel_gateway_app_settings {
                properties.insert(
                    "KernelGatewayAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r_studio_server_pro_app_settings {
                properties.insert(
                    "RStudioServerProAppSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_groups {
                properties.insert(
                    "SecurityGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sharing_settings {
                properties.insert(
                    "SharingSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.space_storage_settings {
                properties.insert(
                    "SpaceStorageSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.studio_web_portal {
                properties.insert(
                    "StudioWebPortal".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.studio_web_portal_settings {
                properties.insert(
                    "StudioWebPortalSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod workteam {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-workteam-cognitomemberdefinition.html
    pub struct CognitoMemberDefinition_ {
        pub cognito_client_id: crate::value::ExpString,
        pub cognito_user_group: crate::value::ExpString,
        pub cognito_user_pool: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Workteam_CognitoMemberDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Workteam.CognitoMemberDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Workteam_CognitoMemberDefinition as CognitoMemberDefinition;
    impl crate::value::ToValue for CognitoMemberDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CognitoClientId".to_string(),
                crate::value::ToValue::to_value(&self.cognito_client_id),
            );
            properties.insert(
                "CognitoUserGroup".to_string(),
                crate::value::ToValue::to_value(&self.cognito_user_group),
            );
            properties.insert(
                "CognitoUserPool".to_string(),
                crate::value::ToValue::to_value(&self.cognito_user_pool),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-workteam-memberdefinition.html
    pub struct MemberDefinition_ {
        pub cognito_member_definition: Option<Box<CognitoMemberDefinition_>>,
        pub oidc_member_definition: Option<Box<OidcMemberDefinition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Workteam_MemberDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Workteam.MemberDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Workteam_MemberDefinition as MemberDefinition;
    impl crate::value::ToValue for MemberDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cognito_member_definition {
                properties.insert(
                    "CognitoMemberDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.oidc_member_definition {
                properties.insert(
                    "OidcMemberDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-workteam-notificationconfiguration.html
    pub struct NotificationConfiguration_ {
        pub notification_topic_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Workteam_NotificationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Workteam.NotificationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Workteam_NotificationConfiguration as NotificationConfiguration;
    impl crate::value::ToValue for NotificationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "NotificationTopicArn".to_string(),
                crate::value::ToValue::to_value(&self.notification_topic_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-workteam-oidcmemberdefinition.html
    pub struct OidcMemberDefinition_ {
        pub oidc_groups: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sagemaker_Workteam_OidcMemberDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SageMaker::Workteam.OidcMemberDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sagemaker_Workteam_OidcMemberDefinition as OidcMemberDefinition;
    impl crate::value::ToValue for OidcMemberDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OidcGroups".to_string(),
                crate::value::ToValue::to_value(&self.oidc_groups),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-app.html
pub struct App_ {
    pub app_name: crate::value::ExpString,
    pub app_type: crate::value::ExpString,
    pub domain_id: crate::value::ExpString,
    pub recovery_mode: Option<crate::value::ExpBool>,
    pub resource_spec: Option<super::sagemaker::app::ResourceSpec_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_profile_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_App {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::App" $($field
        $value)*)
    };
}
pub use crate::__aws_sagemaker_App as App;
impl crate::template::ToResource for App_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("App"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AppName".to_string(),
            crate::value::ToValue::to_value(&self.app_name),
        );
        properties.insert(
            "AppType".to_string(),
            crate::value::ToValue::to_value(&self.app_type),
        );
        properties.insert(
            "DomainId".to_string(),
            crate::value::ToValue::to_value(&self.domain_id),
        );
        if let Some(ref value) = self.recovery_mode {
            properties.insert(
                "RecoveryMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_spec {
            properties.insert(
                "ResourceSpec".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "UserProfileName".to_string(),
            crate::value::ToValue::to_value(&self.user_profile_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-appimageconfig.html
pub struct AppImageConfig_ {
    pub app_image_config_name: crate::value::ExpString,
    pub code_editor_app_image_config:
        Option<super::sagemaker::appimageconfig::CodeEditorAppImageConfig_>,
    pub jupyter_lab_app_image_config:
        Option<super::sagemaker::appimageconfig::JupyterLabAppImageConfig_>,
    pub kernel_gateway_image_config:
        Option<super::sagemaker::appimageconfig::KernelGatewayImageConfig_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_AppImageConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::AppImageConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_AppImageConfig as AppImageConfig;
impl crate::template::ToResource for AppImageConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AppImageConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AppImageConfigName".to_string(),
            crate::value::ToValue::to_value(&self.app_image_config_name),
        );
        if let Some(ref value) = self.code_editor_app_image_config {
            properties.insert(
                "CodeEditorAppImageConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.jupyter_lab_app_image_config {
            properties.insert(
                "JupyterLabAppImageConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kernel_gateway_image_config {
            properties.insert(
                "KernelGatewayImageConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-cluster.html
pub struct Cluster_ {
    pub auto_scaling: Option<super::sagemaker::cluster::ClusterAutoScalingConfig_>,
    pub cluster_name: Option<crate::value::ExpString>,
    pub cluster_role: Option<crate::value::ExpString>,
    pub instance_groups: Option<Vec<super::sagemaker::cluster::ClusterInstanceGroup_>>,
    pub node_provisioning_mode: Option<crate::value::ExpString>,
    pub node_recovery: Option<crate::value::ExpString>,
    pub orchestrator: Option<super::sagemaker::cluster::Orchestrator_>,
    pub restricted_instance_groups:
        Option<Vec<super::sagemaker::cluster::ClusterRestrictedInstanceGroup_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_config: Option<super::sagemaker::cluster::VpcConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_Cluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::Cluster"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_Cluster as Cluster;
impl crate::template::ToResource for Cluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Cluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_scaling {
            properties.insert(
                "AutoScaling".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cluster_name {
            properties.insert(
                "ClusterName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cluster_role {
            properties.insert(
                "ClusterRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_groups {
            properties.insert(
                "InstanceGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.node_provisioning_mode {
            properties.insert(
                "NodeProvisioningMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.node_recovery {
            properties.insert(
                "NodeRecovery".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.orchestrator {
            properties.insert(
                "Orchestrator".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.restricted_instance_groups {
            properties.insert(
                "RestrictedInstanceGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-coderepository.html
pub struct CodeRepository_ {
    pub code_repository_name: Option<crate::value::ExpString>,
    pub git_config: super::sagemaker::coderepository::GitConfig_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_CodeRepository {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::CodeRepository"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_CodeRepository as CodeRepository;
impl crate::template::ToResource for CodeRepository_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CodeRepository"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.code_repository_name {
            properties.insert(
                "CodeRepositoryName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "GitConfig".to_string(),
            crate::value::ToValue::to_value(&self.git_config),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-dataqualityjobdefinition.html
pub struct DataQualityJobDefinition_ {
    pub data_quality_app_specification:
        super::sagemaker::dataqualityjobdefinition::DataQualityAppSpecification_,
    pub data_quality_baseline_config:
        Option<super::sagemaker::dataqualityjobdefinition::DataQualityBaselineConfig_>,
    pub data_quality_job_input: super::sagemaker::dataqualityjobdefinition::DataQualityJobInput_,
    pub data_quality_job_output_config:
        super::sagemaker::dataqualityjobdefinition::MonitoringOutputConfig_,
    pub endpoint_name: Option<crate::value::ExpString>,
    pub job_definition_name: Option<crate::value::ExpString>,
    pub job_resources: super::sagemaker::dataqualityjobdefinition::MonitoringResources_,
    pub network_config: Option<super::sagemaker::dataqualityjobdefinition::NetworkConfig_>,
    pub role_arn: crate::value::ExpString,
    pub stopping_condition: Option<super::sagemaker::dataqualityjobdefinition::StoppingCondition_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_DataQualityJobDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::DataQualityJobDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_DataQualityJobDefinition as DataQualityJobDefinition;
impl crate::template::ToResource for DataQualityJobDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataQualityJobDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DataQualityAppSpecification".to_string(),
            crate::value::ToValue::to_value(&self.data_quality_app_specification),
        );
        if let Some(ref value) = self.data_quality_baseline_config {
            properties.insert(
                "DataQualityBaselineConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DataQualityJobInput".to_string(),
            crate::value::ToValue::to_value(&self.data_quality_job_input),
        );
        properties.insert(
            "DataQualityJobOutputConfig".to_string(),
            crate::value::ToValue::to_value(&self.data_quality_job_output_config),
        );
        if let Some(ref value) = self.endpoint_name {
            properties.insert(
                "EndpointName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_definition_name {
            properties.insert(
                "JobDefinitionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "JobResources".to_string(),
            crate::value::ToValue::to_value(&self.job_resources),
        );
        if let Some(ref value) = self.network_config {
            properties.insert(
                "NetworkConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.stopping_condition {
            properties.insert(
                "StoppingCondition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-device.html
pub struct Device_ {
    pub device: Option<super::sagemaker::device::Device_>,
    pub device_fleet_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_Device {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::Device"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_Device as Device;
impl crate::template::ToResource for Device_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Device"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.device {
            properties.insert("Device".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "DeviceFleetName".to_string(),
            crate::value::ToValue::to_value(&self.device_fleet_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-devicefleet.html
pub struct DeviceFleet_ {
    pub description: Option<crate::value::ExpString>,
    pub device_fleet_name: crate::value::ExpString,
    pub output_config: super::sagemaker::devicefleet::EdgeOutputConfig_,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_DeviceFleet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::DeviceFleet"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_DeviceFleet as DeviceFleet;
impl crate::template::ToResource for DeviceFleet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DeviceFleet"),
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
            "DeviceFleetName".to_string(),
            crate::value::ToValue::to_value(&self.device_fleet_name),
        );
        properties.insert(
            "OutputConfig".to_string(),
            crate::value::ToValue::to_value(&self.output_config),
        );
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-domain.html
pub struct Domain_ {
    pub app_network_access_type: Option<crate::value::ExpString>,
    pub app_security_group_management: Option<crate::value::ExpString>,
    pub auth_mode: crate::value::ExpString,
    pub default_space_settings: Option<super::sagemaker::domain::DefaultSpaceSettings_>,
    pub default_user_settings: super::sagemaker::domain::UserSettings_,
    pub domain_name: crate::value::ExpString,
    pub domain_settings: Option<super::sagemaker::domain::DomainSettings_>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    pub tag_propagation: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_Domain {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::Domain"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_Domain as Domain;
impl crate::template::ToResource for Domain_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Domain"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.app_network_access_type {
            properties.insert(
                "AppNetworkAccessType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.app_security_group_management {
            properties.insert(
                "AppSecurityGroupManagement".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AuthMode".to_string(),
            crate::value::ToValue::to_value(&self.auth_mode),
        );
        if let Some(ref value) = self.default_space_settings {
            properties.insert(
                "DefaultSpaceSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DefaultUserSettings".to_string(),
            crate::value::ToValue::to_value(&self.default_user_settings),
        );
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        if let Some(ref value) = self.domain_settings {
            properties.insert(
                "DomainSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_ids {
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tag_propagation {
            properties.insert(
                "TagPropagation".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpoint.html
pub struct Endpoint_ {
    pub deployment_config: Option<super::sagemaker::endpoint::DeploymentConfig_>,
    pub endpoint_config_name: crate::value::ExpString,
    pub endpoint_name: Option<crate::value::ExpString>,
    pub exclude_retained_variant_properties:
        Option<Vec<super::sagemaker::endpoint::VariantProperty_>>,
    pub retain_all_variant_properties: Option<crate::value::ExpBool>,
    pub retain_deployment_config: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_Endpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::Endpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_Endpoint as Endpoint;
impl crate::template::ToResource for Endpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Endpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.deployment_config {
            properties.insert(
                "DeploymentConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EndpointConfigName".to_string(),
            crate::value::ToValue::to_value(&self.endpoint_config_name),
        );
        if let Some(ref value) = self.endpoint_name {
            properties.insert(
                "EndpointName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.exclude_retained_variant_properties {
            properties.insert(
                "ExcludeRetainedVariantProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.retain_all_variant_properties {
            properties.insert(
                "RetainAllVariantProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.retain_deployment_config {
            properties.insert(
                "RetainDeploymentConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpointconfig.html
pub struct EndpointConfig_ {
    pub async_inference_config: Option<super::sagemaker::endpointconfig::AsyncInferenceConfig_>,
    pub data_capture_config: Option<super::sagemaker::endpointconfig::DataCaptureConfig_>,
    pub enable_network_isolation: Option<crate::value::ExpBool>,
    pub endpoint_config_name: Option<crate::value::ExpString>,
    pub execution_role_arn: Option<crate::value::ExpString>,
    pub explainer_config: Option<super::sagemaker::endpointconfig::ExplainerConfig_>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub production_variants: Vec<super::sagemaker::endpointconfig::ProductionVariant_>,
    pub shadow_production_variants:
        Option<Vec<super::sagemaker::endpointconfig::ProductionVariant_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_config: Option<super::sagemaker::endpointconfig::VpcConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_EndpointConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::EndpointConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_EndpointConfig as EndpointConfig;
impl crate::template::ToResource for EndpointConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EndpointConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.async_inference_config {
            properties.insert(
                "AsyncInferenceConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_capture_config {
            properties.insert(
                "DataCaptureConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_network_isolation {
            properties.insert(
                "EnableNetworkIsolation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint_config_name {
            properties.insert(
                "EndpointConfigName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.execution_role_arn {
            properties.insert(
                "ExecutionRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.explainer_config {
            properties.insert(
                "ExplainerConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ProductionVariants".to_string(),
            crate::value::ToValue::to_value(&self.production_variants),
        );
        if let Some(ref value) = self.shadow_production_variants {
            properties.insert(
                "ShadowProductionVariants".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-featuregroup.html
pub struct FeatureGroup_ {
    pub description: Option<crate::value::ExpString>,
    pub event_time_feature_name: crate::value::ExpString,
    pub feature_definitions: Vec<super::sagemaker::featuregroup::FeatureDefinition_>,
    pub feature_group_name: crate::value::ExpString,
    pub offline_store_config: Option<super::sagemaker::featuregroup::OfflineStoreConfig_>,
    pub online_store_config: Option<super::sagemaker::featuregroup::OnlineStoreConfig_>,
    pub record_identifier_feature_name: crate::value::ExpString,
    pub role_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub throughput_config: Option<super::sagemaker::featuregroup::ThroughputConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_FeatureGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::FeatureGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_FeatureGroup as FeatureGroup;
impl crate::template::ToResource for FeatureGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FeatureGroup"),
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
            "EventTimeFeatureName".to_string(),
            crate::value::ToValue::to_value(&self.event_time_feature_name),
        );
        properties.insert(
            "FeatureDefinitions".to_string(),
            crate::value::ToValue::to_value(&self.feature_definitions),
        );
        properties.insert(
            "FeatureGroupName".to_string(),
            crate::value::ToValue::to_value(&self.feature_group_name),
        );
        if let Some(ref value) = self.offline_store_config {
            properties.insert(
                "OfflineStoreConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.online_store_config {
            properties.insert(
                "OnlineStoreConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RecordIdentifierFeatureName".to_string(),
            crate::value::ToValue::to_value(&self.record_identifier_feature_name),
        );
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.throughput_config {
            properties.insert(
                "ThroughputConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-image.html
pub struct Image_ {
    pub image_description: Option<crate::value::ExpString>,
    pub image_display_name: Option<crate::value::ExpString>,
    pub image_name: crate::value::ExpString,
    pub image_role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_Image {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::Image" $($field
        $value)*)
    };
}
pub use crate::__aws_sagemaker_Image as Image;
impl crate::template::ToResource for Image_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Image"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.image_description {
            properties.insert(
                "ImageDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_display_name {
            properties.insert(
                "ImageDisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ImageName".to_string(),
            crate::value::ToValue::to_value(&self.image_name),
        );
        properties.insert(
            "ImageRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.image_role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-imageversion.html
pub struct ImageVersion_ {
    pub alias: Option<crate::value::ExpString>,
    pub aliases: Option<Vec<crate::value::ExpString>>,
    pub base_image: crate::value::ExpString,
    pub horovod: Option<crate::value::ExpBool>,
    pub image_name: crate::value::ExpString,
    pub job_type: Option<crate::value::ExpString>,
    pub ml_framework: Option<crate::value::ExpString>,
    pub processor: Option<crate::value::ExpString>,
    pub programming_lang: Option<crate::value::ExpString>,
    pub release_notes: Option<crate::value::ExpString>,
    pub vendor_guidance: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_ImageVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::ImageVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_ImageVersion as ImageVersion;
impl crate::template::ToResource for ImageVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ImageVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.alias {
            properties.insert("Alias".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.aliases {
            properties.insert(
                "Aliases".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "BaseImage".to_string(),
            crate::value::ToValue::to_value(&self.base_image),
        );
        if let Some(ref value) = self.horovod {
            properties.insert(
                "Horovod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ImageName".to_string(),
            crate::value::ToValue::to_value(&self.image_name),
        );
        if let Some(ref value) = self.job_type {
            properties.insert(
                "JobType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ml_framework {
            properties.insert(
                "MLFramework".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.processor {
            properties.insert(
                "Processor".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.programming_lang {
            properties.insert(
                "ProgrammingLang".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.release_notes {
            properties.insert(
                "ReleaseNotes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vendor_guidance {
            properties.insert(
                "VendorGuidance".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-inferencecomponent.html
pub struct InferenceComponent_ {
    pub deployment_config:
        Option<super::sagemaker::inferencecomponent::InferenceComponentDeploymentConfig_>,
    pub endpoint_arn: Option<crate::value::ExpString>,
    pub endpoint_name: crate::value::ExpString,
    pub inference_component_name: Option<crate::value::ExpString>,
    pub runtime_config:
        Option<super::sagemaker::inferencecomponent::InferenceComponentRuntimeConfig_>,
    pub specification: super::sagemaker::inferencecomponent::InferenceComponentSpecification_,
    pub tags: Option<Vec<crate::Tag_>>,
    pub variant_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_InferenceComponent {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::InferenceComponent"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_InferenceComponent as InferenceComponent;
impl crate::template::ToResource for InferenceComponent_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InferenceComponent"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.deployment_config {
            properties.insert(
                "DeploymentConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint_arn {
            properties.insert(
                "EndpointArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EndpointName".to_string(),
            crate::value::ToValue::to_value(&self.endpoint_name),
        );
        if let Some(ref value) = self.inference_component_name {
            properties.insert(
                "InferenceComponentName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.runtime_config {
            properties.insert(
                "RuntimeConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Specification".to_string(),
            crate::value::ToValue::to_value(&self.specification),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.variant_name {
            properties.insert(
                "VariantName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-inferenceexperiment.html
pub struct InferenceExperiment_ {
    pub data_storage_config: Option<super::sagemaker::inferenceexperiment::DataStorageConfig_>,
    pub description: Option<crate::value::ExpString>,
    pub desired_state: Option<crate::value::ExpString>,
    pub endpoint_name: crate::value::ExpString,
    pub kms_key: Option<crate::value::ExpString>,
    pub model_variants: Vec<super::sagemaker::inferenceexperiment::ModelVariantConfig_>,
    pub name: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub schedule: Option<super::sagemaker::inferenceexperiment::InferenceExperimentSchedule_>,
    pub shadow_mode_config: Option<super::sagemaker::inferenceexperiment::ShadowModeConfig_>,
    pub status_reason: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_InferenceExperiment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::InferenceExperiment"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_InferenceExperiment as InferenceExperiment;
impl crate::template::ToResource for InferenceExperiment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InferenceExperiment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.data_storage_config {
            properties.insert(
                "DataStorageConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.desired_state {
            properties.insert(
                "DesiredState".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EndpointName".to_string(),
            crate::value::ToValue::to_value(&self.endpoint_name),
        );
        if let Some(ref value) = self.kms_key {
            properties.insert("KmsKey".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "ModelVariants".to_string(),
            crate::value::ToValue::to_value(&self.model_variants),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.schedule {
            properties.insert(
                "Schedule".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.shadow_mode_config {
            properties.insert(
                "ShadowModeConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status_reason {
            properties.insert(
                "StatusReason".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-mlflowtrackingserver.html
pub struct MlflowTrackingServer_ {
    pub artifact_store_uri: crate::value::ExpString,
    pub automatic_model_registration: Option<crate::value::ExpBool>,
    pub mlflow_version: Option<crate::value::ExpString>,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub tracking_server_name: crate::value::ExpString,
    pub tracking_server_size: Option<crate::value::ExpString>,
    pub weekly_maintenance_window_start: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_MlflowTrackingServer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::MlflowTrackingServer"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_MlflowTrackingServer as MlflowTrackingServer;
impl crate::template::ToResource for MlflowTrackingServer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MlflowTrackingServer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ArtifactStoreUri".to_string(),
            crate::value::ToValue::to_value(&self.artifact_store_uri),
        );
        if let Some(ref value) = self.automatic_model_registration {
            properties.insert(
                "AutomaticModelRegistration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.mlflow_version {
            properties.insert(
                "MlflowVersion".to_string(),
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
        properties.insert(
            "TrackingServerName".to_string(),
            crate::value::ToValue::to_value(&self.tracking_server_name),
        );
        if let Some(ref value) = self.tracking_server_size {
            properties.insert(
                "TrackingServerSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.weekly_maintenance_window_start {
            properties.insert(
                "WeeklyMaintenanceWindowStart".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-model.html
pub struct Model_ {
    pub containers: Option<Vec<super::sagemaker::model::ContainerDefinition_>>,
    pub enable_network_isolation: Option<crate::value::ExpBool>,
    pub execution_role_arn: Option<crate::value::ExpString>,
    pub inference_execution_config: Option<super::sagemaker::model::InferenceExecutionConfig_>,
    pub model_name: Option<crate::value::ExpString>,
    pub primary_container: Option<super::sagemaker::model::ContainerDefinition_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_config: Option<super::sagemaker::model::VpcConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_Model {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::Model" $($field
        $value)*)
    };
}
pub use crate::__aws_sagemaker_Model as Model;
impl crate::template::ToResource for Model_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Model"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.containers {
            properties.insert(
                "Containers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_network_isolation {
            properties.insert(
                "EnableNetworkIsolation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.execution_role_arn {
            properties.insert(
                "ExecutionRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.inference_execution_config {
            properties.insert(
                "InferenceExecutionConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.model_name {
            properties.insert(
                "ModelName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.primary_container {
            properties.insert(
                "PrimaryContainer".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelbiasjobdefinition.html
pub struct ModelBiasJobDefinition_ {
    pub endpoint_name: Option<crate::value::ExpString>,
    pub job_definition_name: Option<crate::value::ExpString>,
    pub job_resources: super::sagemaker::modelbiasjobdefinition::MonitoringResources_,
    pub model_bias_app_specification:
        super::sagemaker::modelbiasjobdefinition::ModelBiasAppSpecification_,
    pub model_bias_baseline_config:
        Option<super::sagemaker::modelbiasjobdefinition::ModelBiasBaselineConfig_>,
    pub model_bias_job_input: super::sagemaker::modelbiasjobdefinition::ModelBiasJobInput_,
    pub model_bias_job_output_config:
        super::sagemaker::modelbiasjobdefinition::MonitoringOutputConfig_,
    pub network_config: Option<super::sagemaker::modelbiasjobdefinition::NetworkConfig_>,
    pub role_arn: crate::value::ExpString,
    pub stopping_condition: Option<super::sagemaker::modelbiasjobdefinition::StoppingCondition_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_ModelBiasJobDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::ModelBiasJobDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_ModelBiasJobDefinition as ModelBiasJobDefinition;
impl crate::template::ToResource for ModelBiasJobDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ModelBiasJobDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.endpoint_name {
            properties.insert(
                "EndpointName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_definition_name {
            properties.insert(
                "JobDefinitionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "JobResources".to_string(),
            crate::value::ToValue::to_value(&self.job_resources),
        );
        properties.insert(
            "ModelBiasAppSpecification".to_string(),
            crate::value::ToValue::to_value(&self.model_bias_app_specification),
        );
        if let Some(ref value) = self.model_bias_baseline_config {
            properties.insert(
                "ModelBiasBaselineConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ModelBiasJobInput".to_string(),
            crate::value::ToValue::to_value(&self.model_bias_job_input),
        );
        properties.insert(
            "ModelBiasJobOutputConfig".to_string(),
            crate::value::ToValue::to_value(&self.model_bias_job_output_config),
        );
        if let Some(ref value) = self.network_config {
            properties.insert(
                "NetworkConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.stopping_condition {
            properties.insert(
                "StoppingCondition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelcard.html
pub struct ModelCard_ {
    pub content: super::sagemaker::modelcard::Content_,
    pub created_by: Option<super::sagemaker::modelcard::UserContext_>,
    pub last_modified_by: Option<super::sagemaker::modelcard::UserContext_>,
    pub model_card_name: crate::value::ExpString,
    pub model_card_status: crate::value::ExpString,
    pub security_config: Option<super::sagemaker::modelcard::SecurityConfig_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_ModelCard {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::ModelCard"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_ModelCard as ModelCard;
impl crate::template::ToResource for ModelCard_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ModelCard"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Content".to_string(),
            crate::value::ToValue::to_value(&self.content),
        );
        if let Some(ref value) = self.created_by {
            properties.insert(
                "CreatedBy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.last_modified_by {
            properties.insert(
                "LastModifiedBy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ModelCardName".to_string(),
            crate::value::ToValue::to_value(&self.model_card_name),
        );
        properties.insert(
            "ModelCardStatus".to_string(),
            crate::value::ToValue::to_value(&self.model_card_status),
        );
        if let Some(ref value) = self.security_config {
            properties.insert(
                "SecurityConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelexplainabilityjobdefinition.html
pub struct ModelExplainabilityJobDefinition_ {
    pub endpoint_name: Option<crate::value::ExpString>,
    pub job_definition_name: Option<crate::value::ExpString>,
    pub job_resources: super::sagemaker::modelexplainabilityjobdefinition::MonitoringResources_,
    pub model_explainability_app_specification:
        super::sagemaker::modelexplainabilityjobdefinition::ModelExplainabilityAppSpecification_,
    pub model_explainability_baseline_config: Option<
        super::sagemaker::modelexplainabilityjobdefinition::ModelExplainabilityBaselineConfig_,
    >,
    pub model_explainability_job_input:
        super::sagemaker::modelexplainabilityjobdefinition::ModelExplainabilityJobInput_,
    pub model_explainability_job_output_config:
        super::sagemaker::modelexplainabilityjobdefinition::MonitoringOutputConfig_,
    pub network_config: Option<super::sagemaker::modelexplainabilityjobdefinition::NetworkConfig_>,
    pub role_arn: crate::value::ExpString,
    pub stopping_condition:
        Option<super::sagemaker::modelexplainabilityjobdefinition::StoppingCondition_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_ModelExplainabilityJobDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::ModelExplainabilityJobDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_ModelExplainabilityJobDefinition as ModelExplainabilityJobDefinition;
impl crate::template::ToResource for ModelExplainabilityJobDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ModelExplainabilityJobDefinition",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.endpoint_name {
            properties.insert(
                "EndpointName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_definition_name {
            properties.insert(
                "JobDefinitionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "JobResources".to_string(),
            crate::value::ToValue::to_value(&self.job_resources),
        );
        properties.insert(
            "ModelExplainabilityAppSpecification".to_string(),
            crate::value::ToValue::to_value(&self.model_explainability_app_specification),
        );
        if let Some(ref value) = self.model_explainability_baseline_config {
            properties.insert(
                "ModelExplainabilityBaselineConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ModelExplainabilityJobInput".to_string(),
            crate::value::ToValue::to_value(&self.model_explainability_job_input),
        );
        properties.insert(
            "ModelExplainabilityJobOutputConfig".to_string(),
            crate::value::ToValue::to_value(&self.model_explainability_job_output_config),
        );
        if let Some(ref value) = self.network_config {
            properties.insert(
                "NetworkConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.stopping_condition {
            properties.insert(
                "StoppingCondition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html
pub struct ModelPackage_ {
    pub additional_inference_specifications:
        Option<Vec<super::sagemaker::modelpackage::AdditionalInferenceSpecificationDefinition_>>,
    pub additional_inference_specifications_to_add:
        Option<Vec<super::sagemaker::modelpackage::AdditionalInferenceSpecificationDefinition_>>,
    pub approval_description: Option<crate::value::ExpString>,
    pub certify_for_marketplace: Option<crate::value::ExpBool>,
    pub client_token: Option<crate::value::ExpString>,
    pub customer_metadata_properties:
        Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub domain: Option<crate::value::ExpString>,
    pub drift_check_baselines: Option<super::sagemaker::modelpackage::DriftCheckBaselines_>,
    pub inference_specification: Option<super::sagemaker::modelpackage::InferenceSpecification_>,
    pub last_modified_time: Option<crate::value::ExpString>,
    pub metadata_properties: Option<super::sagemaker::modelpackage::MetadataProperties_>,
    pub model_approval_status: Option<crate::value::ExpString>,
    pub model_card: Option<super::sagemaker::modelpackage::ModelCard_>,
    pub model_metrics: Option<super::sagemaker::modelpackage::ModelMetrics_>,
    pub model_package_description: Option<crate::value::ExpString>,
    pub model_package_group_name: Option<crate::value::ExpString>,
    pub model_package_name: Option<crate::value::ExpString>,
    pub model_package_status_details:
        Option<super::sagemaker::modelpackage::ModelPackageStatusDetails_>,
    pub model_package_version: Option<i64>,
    pub sample_payload_url: Option<crate::value::ExpString>,
    pub security_config: Option<super::sagemaker::modelpackage::SecurityConfig_>,
    pub skip_model_validation: Option<crate::value::ExpString>,
    pub source_algorithm_specification:
        Option<super::sagemaker::modelpackage::SourceAlgorithmSpecification_>,
    pub source_uri: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub task: Option<crate::value::ExpString>,
    pub validation_specification: Option<super::sagemaker::modelpackage::ValidationSpecification_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_ModelPackage {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::ModelPackage"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_ModelPackage as ModelPackage;
impl crate::template::ToResource for ModelPackage_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ModelPackage"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_inference_specifications {
            properties.insert(
                "AdditionalInferenceSpecifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.additional_inference_specifications_to_add {
            properties.insert(
                "AdditionalInferenceSpecificationsToAdd".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.approval_description {
            properties.insert(
                "ApprovalDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certify_for_marketplace {
            properties.insert(
                "CertifyForMarketplace".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.client_token {
            properties.insert(
                "ClientToken".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.customer_metadata_properties {
            properties.insert(
                "CustomerMetadataProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain {
            properties.insert("Domain".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.drift_check_baselines {
            properties.insert(
                "DriftCheckBaselines".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.inference_specification {
            properties.insert(
                "InferenceSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.last_modified_time {
            properties.insert(
                "LastModifiedTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metadata_properties {
            properties.insert(
                "MetadataProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.model_approval_status {
            properties.insert(
                "ModelApprovalStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.model_card {
            properties.insert(
                "ModelCard".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.model_metrics {
            properties.insert(
                "ModelMetrics".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.model_package_description {
            properties.insert(
                "ModelPackageDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.model_package_group_name {
            properties.insert(
                "ModelPackageGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.model_package_name {
            properties.insert(
                "ModelPackageName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.model_package_status_details {
            properties.insert(
                "ModelPackageStatusDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.model_package_version {
            properties.insert(
                "ModelPackageVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sample_payload_url {
            properties.insert(
                "SamplePayloadUrl".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_config {
            properties.insert(
                "SecurityConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.skip_model_validation {
            properties.insert(
                "SkipModelValidation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_algorithm_specification {
            properties.insert(
                "SourceAlgorithmSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_uri {
            properties.insert(
                "SourceUri".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.task {
            properties.insert("Task".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.validation_specification {
            properties.insert(
                "ValidationSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackagegroup.html
pub struct ModelPackageGroup_ {
    pub model_package_group_description: Option<crate::value::ExpString>,
    pub model_package_group_name: crate::value::ExpString,
    pub model_package_group_policy: Option<serde_json::Value>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_ModelPackageGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::ModelPackageGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_ModelPackageGroup as ModelPackageGroup;
impl crate::template::ToResource for ModelPackageGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ModelPackageGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.model_package_group_description {
            properties.insert(
                "ModelPackageGroupDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ModelPackageGroupName".to_string(),
            crate::value::ToValue::to_value(&self.model_package_group_name),
        );
        if let Some(ref value) = self.model_package_group_policy {
            properties.insert(
                "ModelPackageGroupPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelqualityjobdefinition.html
pub struct ModelQualityJobDefinition_ {
    pub endpoint_name: Option<crate::value::ExpString>,
    pub job_definition_name: Option<crate::value::ExpString>,
    pub job_resources: super::sagemaker::modelqualityjobdefinition::MonitoringResources_,
    pub model_quality_app_specification:
        super::sagemaker::modelqualityjobdefinition::ModelQualityAppSpecification_,
    pub model_quality_baseline_config:
        Option<super::sagemaker::modelqualityjobdefinition::ModelQualityBaselineConfig_>,
    pub model_quality_job_input: super::sagemaker::modelqualityjobdefinition::ModelQualityJobInput_,
    pub model_quality_job_output_config:
        super::sagemaker::modelqualityjobdefinition::MonitoringOutputConfig_,
    pub network_config: Option<super::sagemaker::modelqualityjobdefinition::NetworkConfig_>,
    pub role_arn: crate::value::ExpString,
    pub stopping_condition: Option<super::sagemaker::modelqualityjobdefinition::StoppingCondition_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_ModelQualityJobDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::ModelQualityJobDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_ModelQualityJobDefinition as ModelQualityJobDefinition;
impl crate::template::ToResource for ModelQualityJobDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ModelQualityJobDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.endpoint_name {
            properties.insert(
                "EndpointName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_definition_name {
            properties.insert(
                "JobDefinitionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "JobResources".to_string(),
            crate::value::ToValue::to_value(&self.job_resources),
        );
        properties.insert(
            "ModelQualityAppSpecification".to_string(),
            crate::value::ToValue::to_value(&self.model_quality_app_specification),
        );
        if let Some(ref value) = self.model_quality_baseline_config {
            properties.insert(
                "ModelQualityBaselineConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ModelQualityJobInput".to_string(),
            crate::value::ToValue::to_value(&self.model_quality_job_input),
        );
        properties.insert(
            "ModelQualityJobOutputConfig".to_string(),
            crate::value::ToValue::to_value(&self.model_quality_job_output_config),
        );
        if let Some(ref value) = self.network_config {
            properties.insert(
                "NetworkConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.stopping_condition {
            properties.insert(
                "StoppingCondition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-monitoringschedule.html
pub struct MonitoringSchedule_ {
    pub endpoint_name: Option<crate::value::ExpString>,
    pub failure_reason: Option<crate::value::ExpString>,
    pub last_monitoring_execution_summary:
        Option<super::sagemaker::monitoringschedule::MonitoringExecutionSummary_>,
    pub monitoring_schedule_config: super::sagemaker::monitoringschedule::MonitoringScheduleConfig_,
    pub monitoring_schedule_name: crate::value::ExpString,
    pub monitoring_schedule_status: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_MonitoringSchedule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::MonitoringSchedule"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_MonitoringSchedule as MonitoringSchedule;
impl crate::template::ToResource for MonitoringSchedule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MonitoringSchedule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.endpoint_name {
            properties.insert(
                "EndpointName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.failure_reason {
            properties.insert(
                "FailureReason".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.last_monitoring_execution_summary {
            properties.insert(
                "LastMonitoringExecutionSummary".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MonitoringScheduleConfig".to_string(),
            crate::value::ToValue::to_value(&self.monitoring_schedule_config),
        );
        properties.insert(
            "MonitoringScheduleName".to_string(),
            crate::value::ToValue::to_value(&self.monitoring_schedule_name),
        );
        if let Some(ref value) = self.monitoring_schedule_status {
            properties.insert(
                "MonitoringScheduleStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html
pub struct NotebookInstance_ {
    pub accelerator_types: Option<Vec<crate::value::ExpString>>,
    pub additional_code_repositories: Option<Vec<crate::value::ExpString>>,
    pub default_code_repository: Option<crate::value::ExpString>,
    pub direct_internet_access: Option<crate::value::ExpString>,
    pub instance_metadata_service_configuration:
        Option<super::sagemaker::notebookinstance::InstanceMetadataServiceConfiguration_>,
    pub instance_type: crate::value::ExpString,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub lifecycle_config_name: Option<crate::value::ExpString>,
    pub notebook_instance_name: Option<crate::value::ExpString>,
    pub platform_identifier: Option<crate::value::ExpString>,
    pub role_arn: crate::value::ExpString,
    pub root_access: Option<crate::value::ExpString>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub subnet_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub volume_size_in_gb: Option<i64>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_NotebookInstance {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::NotebookInstance"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_NotebookInstance as NotebookInstance;
impl crate::template::ToResource for NotebookInstance_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("NotebookInstance"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accelerator_types {
            properties.insert(
                "AcceleratorTypes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.additional_code_repositories {
            properties.insert(
                "AdditionalCodeRepositories".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_code_repository {
            properties.insert(
                "DefaultCodeRepository".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.direct_internet_access {
            properties.insert(
                "DirectInternetAccess".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_metadata_service_configuration {
            properties.insert(
                "InstanceMetadataServiceConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceType".to_string(),
            crate::value::ToValue::to_value(&self.instance_type),
        );
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lifecycle_config_name {
            properties.insert(
                "LifecycleConfigName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.notebook_instance_name {
            properties.insert(
                "NotebookInstanceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.platform_identifier {
            properties.insert(
                "PlatformIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.root_access {
            properties.insert(
                "RootAccess".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
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
        if let Some(ref value) = self.volume_size_in_gb {
            properties.insert(
                "VolumeSizeInGB".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstancelifecycleconfig.html
pub struct NotebookInstanceLifecycleConfig_ {
    pub notebook_instance_lifecycle_config_name: Option<crate::value::ExpString>,
    pub on_create: Option<
        Vec<super::sagemaker::notebookinstancelifecycleconfig::NotebookInstanceLifecycleHook_>,
    >,
    pub on_start: Option<
        Vec<super::sagemaker::notebookinstancelifecycleconfig::NotebookInstanceLifecycleHook_>,
    >,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_NotebookInstanceLifecycleConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::NotebookInstanceLifecycleConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_NotebookInstanceLifecycleConfig as NotebookInstanceLifecycleConfig;
impl crate::template::ToResource for NotebookInstanceLifecycleConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "NotebookInstanceLifecycleConfig",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.notebook_instance_lifecycle_config_name {
            properties.insert(
                "NotebookInstanceLifecycleConfigName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.on_create {
            properties.insert(
                "OnCreate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.on_start {
            properties.insert(
                "OnStart".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-partnerapp.html
pub struct PartnerApp_ {
    pub application_config: Option<super::sagemaker::partnerapp::PartnerAppConfig_>,
    pub auth_type: crate::value::ExpString,
    pub enable_iam_session_based_identity: Option<crate::value::ExpBool>,
    pub execution_role_arn: crate::value::ExpString,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub maintenance_config: Option<super::sagemaker::partnerapp::PartnerAppMaintenanceConfig_>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub tier: crate::value::ExpString,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_PartnerApp {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::PartnerApp"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_PartnerApp as PartnerApp;
impl crate::template::ToResource for PartnerApp_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PartnerApp"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.application_config {
            properties.insert(
                "ApplicationConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AuthType".to_string(),
            crate::value::ToValue::to_value(&self.auth_type),
        );
        if let Some(ref value) = self.enable_iam_session_based_identity {
            properties.insert(
                "EnableIamSessionBasedIdentity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ExecutionRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.execution_role_arn),
        );
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maintenance_config {
            properties.insert(
                "MaintenanceConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Tier".to_string(),
            crate::value::ToValue::to_value(&self.tier),
        );
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-pipeline.html
pub struct Pipeline_ {
    pub parallelism_configuration: Option<super::sagemaker::pipeline::ParallelismConfiguration_>,
    pub pipeline_definition: super::sagemaker::pipeline::PipelineDefinition_,
    pub pipeline_description: Option<crate::value::ExpString>,
    pub pipeline_display_name: Option<crate::value::ExpString>,
    pub pipeline_name: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_Pipeline {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::Pipeline"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_Pipeline as Pipeline;
impl crate::template::ToResource for Pipeline_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Pipeline"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.parallelism_configuration {
            properties.insert(
                "ParallelismConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PipelineDefinition".to_string(),
            crate::value::ToValue::to_value(&self.pipeline_definition),
        );
        if let Some(ref value) = self.pipeline_description {
            properties.insert(
                "PipelineDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.pipeline_display_name {
            properties.insert(
                "PipelineDisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PipelineName".to_string(),
            crate::value::ToValue::to_value(&self.pipeline_name),
        );
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-processingjob.html
pub struct ProcessingJob_ {
    pub app_specification: super::sagemaker::processingjob::AppSpecification_,
    pub environment: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub experiment_config: Option<super::sagemaker::processingjob::ExperimentConfig_>,
    pub network_config: Option<super::sagemaker::processingjob::NetworkConfig_>,
    pub processing_inputs: Option<Vec<super::sagemaker::processingjob::ProcessingInputsObject_>>,
    pub processing_job_name: Option<crate::value::ExpString>,
    pub processing_output_config: Option<super::sagemaker::processingjob::ProcessingOutputConfig_>,
    pub processing_resources: super::sagemaker::processingjob::ProcessingResources_,
    pub role_arn: crate::value::ExpString,
    pub stopping_condition: Option<super::sagemaker::processingjob::StoppingCondition_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_ProcessingJob {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::ProcessingJob"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_ProcessingJob as ProcessingJob;
impl crate::template::ToResource for ProcessingJob_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ProcessingJob"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AppSpecification".to_string(),
            crate::value::ToValue::to_value(&self.app_specification),
        );
        if let Some(ref value) = self.environment {
            properties.insert(
                "Environment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.experiment_config {
            properties.insert(
                "ExperimentConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_config {
            properties.insert(
                "NetworkConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.processing_inputs {
            properties.insert(
                "ProcessingInputs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.processing_job_name {
            properties.insert(
                "ProcessingJobName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.processing_output_config {
            properties.insert(
                "ProcessingOutputConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ProcessingResources".to_string(),
            crate::value::ToValue::to_value(&self.processing_resources),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.stopping_condition {
            properties.insert(
                "StoppingCondition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-project.html
pub struct Project_ {
    pub project_description: Option<crate::value::ExpString>,
    pub project_name: crate::value::ExpString,
    pub service_catalog_provisioned_product_details:
        Option<super::sagemaker::project::ServiceCatalogProvisionedProductDetails_>,
    pub service_catalog_provisioning_details:
        Option<super::sagemaker::project::ServiceCatalogProvisioningDetails_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub template_provider_details: Option<Vec<super::sagemaker::project::TemplateProviderDetail_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_Project {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::Project"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_Project as Project;
impl crate::template::ToResource for Project_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Project"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.project_description {
            properties.insert(
                "ProjectDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ProjectName".to_string(),
            crate::value::ToValue::to_value(&self.project_name),
        );
        if let Some(ref value) = self.service_catalog_provisioned_product_details {
            properties.insert(
                "ServiceCatalogProvisionedProductDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_catalog_provisioning_details {
            properties.insert(
                "ServiceCatalogProvisioningDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.template_provider_details {
            properties.insert(
                "TemplateProviderDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-space.html
pub struct Space_ {
    pub domain_id: crate::value::ExpString,
    pub ownership_settings: Option<super::sagemaker::space::OwnershipSettings_>,
    pub space_display_name: Option<crate::value::ExpString>,
    pub space_name: crate::value::ExpString,
    pub space_settings: Option<super::sagemaker::space::SpaceSettings_>,
    pub space_sharing_settings: Option<super::sagemaker::space::SpaceSharingSettings_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_Space {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::Space" $($field
        $value)*)
    };
}
pub use crate::__aws_sagemaker_Space as Space;
impl crate::template::ToResource for Space_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Space"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DomainId".to_string(),
            crate::value::ToValue::to_value(&self.domain_id),
        );
        if let Some(ref value) = self.ownership_settings {
            properties.insert(
                "OwnershipSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.space_display_name {
            properties.insert(
                "SpaceDisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SpaceName".to_string(),
            crate::value::ToValue::to_value(&self.space_name),
        );
        if let Some(ref value) = self.space_settings {
            properties.insert(
                "SpaceSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.space_sharing_settings {
            properties.insert(
                "SpaceSharingSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-studiolifecycleconfig.html
pub struct StudioLifecycleConfig_ {
    pub studio_lifecycle_config_app_type: crate::value::ExpString,
    pub studio_lifecycle_config_content: crate::value::ExpString,
    pub studio_lifecycle_config_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_StudioLifecycleConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::StudioLifecycleConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_StudioLifecycleConfig as StudioLifecycleConfig;
impl crate::template::ToResource for StudioLifecycleConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StudioLifecycleConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "StudioLifecycleConfigAppType".to_string(),
            crate::value::ToValue::to_value(&self.studio_lifecycle_config_app_type),
        );
        properties.insert(
            "StudioLifecycleConfigContent".to_string(),
            crate::value::ToValue::to_value(&self.studio_lifecycle_config_content),
        );
        properties.insert(
            "StudioLifecycleConfigName".to_string(),
            crate::value::ToValue::to_value(&self.studio_lifecycle_config_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-userprofile.html
pub struct UserProfile_ {
    pub domain_id: crate::value::ExpString,
    pub single_sign_on_user_identifier: Option<crate::value::ExpString>,
    pub single_sign_on_user_value: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_profile_name: crate::value::ExpString,
    pub user_settings: Option<super::sagemaker::userprofile::UserSettings_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_UserProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::UserProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_UserProfile as UserProfile;
impl crate::template::ToResource for UserProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DomainId".to_string(),
            crate::value::ToValue::to_value(&self.domain_id),
        );
        if let Some(ref value) = self.single_sign_on_user_identifier {
            properties.insert(
                "SingleSignOnUserIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.single_sign_on_user_value {
            properties.insert(
                "SingleSignOnUserValue".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "UserProfileName".to_string(),
            crate::value::ToValue::to_value(&self.user_profile_name),
        );
        if let Some(ref value) = self.user_settings {
            properties.insert(
                "UserSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-workteam.html
pub struct Workteam_ {
    pub description: Option<crate::value::ExpString>,
    pub member_definitions: Option<Vec<super::sagemaker::workteam::MemberDefinition_>>,
    pub notification_configuration: Option<super::sagemaker::workteam::NotificationConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub workforce_name: Option<crate::value::ExpString>,
    pub workteam_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sagemaker_Workteam {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SageMaker::Workteam"
        $($field $value)*)
    };
}
pub use crate::__aws_sagemaker_Workteam as Workteam;
impl crate::template::ToResource for Workteam_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SageMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Workteam"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.member_definitions {
            properties.insert(
                "MemberDefinitions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.notification_configuration {
            properties.insert(
                "NotificationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.workforce_name {
            properties.insert(
                "WorkforceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.workteam_name {
            properties.insert(
                "WorkteamName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
