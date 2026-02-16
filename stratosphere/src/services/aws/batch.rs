pub mod computeenvironment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html
    pub struct ComputeResources_ {
        pub allocation_strategy: Option<crate::value::ExpString>,
        pub bid_percentage: Option<i32>,
        pub desiredv_cpus: Option<i32>,
        pub ec2_configuration: Option<Vec<Ec2ConfigurationObject_>>,
        pub ec2_key_pair: Option<crate::value::ExpString>,
        pub image_id: Option<crate::value::ExpString>,
        pub instance_role: Option<crate::value::ExpString>,
        pub instance_types: Option<Vec<crate::value::ExpString>>,
        pub launch_template: Option<Box<LaunchTemplateSpecification_>>,
        pub maxv_cpus: i32,
        pub minv_cpus: Option<i32>,
        pub placement_group: Option<crate::value::ExpString>,
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub spot_iam_fleet_role: Option<crate::value::ExpString>,
        pub subnets: Vec<crate::value::ExpString>,
        pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub r#type: crate::value::ExpString,
        pub update_to_latest_image_version: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_ComputeEnvironment_ComputeResources {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::ComputeEnvironment.ComputeResources"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_ComputeEnvironment_ComputeResources as ComputeResources;
    impl crate::value::ToValue for ComputeResources_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allocation_strategy {
                properties.insert(
                    "AllocationStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bid_percentage {
                properties.insert(
                    "BidPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.desiredv_cpus {
                properties.insert(
                    "DesiredvCpus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ec2_configuration {
                properties.insert(
                    "Ec2Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ec2_key_pair {
                properties.insert(
                    "Ec2KeyPair".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image_id {
                properties.insert(
                    "ImageId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_role {
                properties.insert(
                    "InstanceRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_types {
                properties.insert(
                    "InstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.launch_template {
                properties.insert(
                    "LaunchTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MaxvCpus".to_string(),
                crate::value::ToValue::to_value(&self.maxv_cpus),
            );
            if let Some(ref value) = self.minv_cpus {
                properties.insert(
                    "MinvCpus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.placement_group {
                properties.insert(
                    "PlacementGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_iam_fleet_role {
                properties.insert(
                    "SpotIamFleetRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.update_to_latest_image_version {
                properties.insert(
                    "UpdateToLatestImageVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-ec2configurationobject.html
    pub struct Ec2ConfigurationObject_ {
        pub image_id_override: Option<crate::value::ExpString>,
        pub image_kubernetes_version: Option<crate::value::ExpString>,
        pub image_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_ComputeEnvironment_Ec2ConfigurationObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::ComputeEnvironment.Ec2ConfigurationObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_ComputeEnvironment_Ec2ConfigurationObject as Ec2ConfigurationObject;
    impl crate::value::ToValue for Ec2ConfigurationObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.image_id_override {
                properties.insert(
                    "ImageIdOverride".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image_kubernetes_version {
                properties.insert(
                    "ImageKubernetesVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ImageType".to_string(),
                crate::value::ToValue::to_value(&self.image_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-eksconfiguration.html
    pub struct EksConfiguration_ {
        pub eks_cluster_arn: crate::value::ExpString,
        pub kubernetes_namespace: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_ComputeEnvironment_EksConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::ComputeEnvironment.EksConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_ComputeEnvironment_EksConfiguration as EksConfiguration;
    impl crate::value::ToValue for EksConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EksClusterArn".to_string(),
                crate::value::ToValue::to_value(&self.eks_cluster_arn),
            );
            properties.insert(
                "KubernetesNamespace".to_string(),
                crate::value::ToValue::to_value(&self.kubernetes_namespace),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-launchtemplatespecification.html
    pub struct LaunchTemplateSpecification_ {
        pub launch_template_id: Option<crate::value::ExpString>,
        pub launch_template_name: Option<crate::value::ExpString>,
        pub overrides: Option<Vec<LaunchTemplateSpecificationOverride_>>,
        pub userdata_type: Option<crate::value::ExpString>,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_ComputeEnvironment_LaunchTemplateSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::ComputeEnvironment.LaunchTemplateSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_ComputeEnvironment_LaunchTemplateSpecification as LaunchTemplateSpecification;
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
            if let Some(ref value) = self.overrides {
                properties.insert(
                    "Overrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.userdata_type {
                properties.insert(
                    "UserdataType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-launchtemplatespecificationoverride.html
    pub struct LaunchTemplateSpecificationOverride_ {
        pub launch_template_id: Option<crate::value::ExpString>,
        pub launch_template_name: Option<crate::value::ExpString>,
        pub target_instance_types: Option<Vec<crate::value::ExpString>>,
        pub userdata_type: Option<crate::value::ExpString>,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_ComputeEnvironment_LaunchTemplateSpecificationOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::ComputeEnvironment.LaunchTemplateSpecificationOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_ComputeEnvironment_LaunchTemplateSpecificationOverride as LaunchTemplateSpecificationOverride;
    impl crate::value::ToValue for LaunchTemplateSpecificationOverride_ {
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
            if let Some(ref value) = self.target_instance_types {
                properties.insert(
                    "TargetInstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.userdata_type {
                properties.insert(
                    "UserdataType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-updatepolicy.html
    pub struct UpdatePolicy_ {
        pub job_execution_timeout_minutes: Option<i32>,
        pub terminate_jobs_on_update: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_ComputeEnvironment_UpdatePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::ComputeEnvironment.UpdatePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_ComputeEnvironment_UpdatePolicy as UpdatePolicy;
    impl crate::value::ToValue for UpdatePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.job_execution_timeout_minutes {
                properties.insert(
                    "JobExecutionTimeoutMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.terminate_jobs_on_update {
                properties.insert(
                    "TerminateJobsOnUpdate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod jobdefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-consumableresourceproperties.html
    pub struct ConsumableResourceProperties_ {
        pub consumable_resource_list: Vec<ConsumableResourceRequirement_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_ConsumableResourceProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.ConsumableResourceProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_ConsumableResourceProperties as ConsumableResourceProperties;
    impl crate::value::ToValue for ConsumableResourceProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConsumableResourceList".to_string(),
                crate::value::ToValue::to_value(&self.consumable_resource_list),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-consumableresourcerequirement.html
    pub struct ConsumableResourceRequirement_ {
        pub consumable_resource: crate::value::ExpString,
        pub quantity: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_ConsumableResourceRequirement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.ConsumableResourceRequirement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_ConsumableResourceRequirement as ConsumableResourceRequirement;
    impl crate::value::ToValue for ConsumableResourceRequirement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConsumableResource".to_string(),
                crate::value::ToValue::to_value(&self.consumable_resource),
            );
            properties.insert(
                "Quantity".to_string(),
                crate::value::ToValue::to_value(&self.quantity),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html
    pub struct ContainerProperties_ {
        pub command: Option<Vec<crate::value::ExpString>>,
        pub enable_execute_command: Option<crate::value::ExpBool>,
        pub environment: Option<Vec<Environment_>>,
        pub ephemeral_storage: Option<Box<EphemeralStorage_>>,
        pub execution_role_arn: Option<crate::value::ExpString>,
        pub fargate_platform_configuration: Option<Box<FargatePlatformConfiguration_>>,
        pub image: crate::value::ExpString,
        pub job_role_arn: Option<crate::value::ExpString>,
        pub linux_parameters: Option<Box<LinuxParameters_>>,
        pub log_configuration: Option<Box<LogConfiguration_>>,
        pub memory: Option<i32>,
        pub mount_points: Option<Vec<MountPoint_>>,
        pub network_configuration: Option<Box<NetworkConfiguration_>>,
        pub privileged: Option<crate::value::ExpBool>,
        pub readonly_root_filesystem: Option<crate::value::ExpBool>,
        pub repository_credentials: Option<Box<RepositoryCredentials_>>,
        pub resource_requirements: Option<Vec<ResourceRequirement_>>,
        pub runtime_platform: Option<Box<RuntimePlatform_>>,
        pub secrets: Option<Vec<Secret_>>,
        pub ulimits: Option<Vec<Ulimit_>>,
        pub user: Option<crate::value::ExpString>,
        pub vcpus: Option<i32>,
        pub volumes: Option<Vec<Volume_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_ContainerProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.ContainerProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_ContainerProperties as ContainerProperties;
    impl crate::value::ToValue for ContainerProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.command {
                properties.insert(
                    "Command".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_execute_command {
                properties.insert(
                    "EnableExecuteCommand".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ephemeral_storage {
                properties.insert(
                    "EphemeralStorage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execution_role_arn {
                properties.insert(
                    "ExecutionRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fargate_platform_configuration {
                properties.insert(
                    "FargatePlatformConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Image".to_string(),
                crate::value::ToValue::to_value(&self.image),
            );
            if let Some(ref value) = self.job_role_arn {
                properties.insert(
                    "JobRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.linux_parameters {
                properties.insert(
                    "LinuxParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_configuration {
                properties.insert(
                    "LogConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory {
                properties.insert("Memory".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.mount_points {
                properties.insert(
                    "MountPoints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_configuration {
                properties.insert(
                    "NetworkConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.privileged {
                properties.insert(
                    "Privileged".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.readonly_root_filesystem {
                properties.insert(
                    "ReadonlyRootFilesystem".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.repository_credentials {
                properties.insert(
                    "RepositoryCredentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_requirements {
                properties.insert(
                    "ResourceRequirements".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.runtime_platform {
                properties.insert(
                    "RuntimePlatform".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets {
                properties.insert(
                    "Secrets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ulimits {
                properties.insert(
                    "Ulimits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user {
                properties.insert("User".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.vcpus {
                properties.insert("Vcpus".to_string(), crate::value::ToValue::to_value(value));
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-device.html
    pub struct Device_ {
        pub container_path: Option<crate::value::ExpString>,
        pub host_path: Option<crate::value::ExpString>,
        pub permissions: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_Device {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.Device"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_Device as Device;
    impl crate::value::ToValue for Device_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_path {
                properties.insert(
                    "ContainerPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.host_path {
                properties.insert(
                    "HostPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.permissions {
                properties.insert(
                    "Permissions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-efsauthorizationconfig.html
    pub struct EFSAuthorizationConfig_ {
        pub access_point_id: Option<crate::value::ExpString>,
        pub iam: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EFSAuthorizationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EFSAuthorizationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EFSAuthorizationConfig as EFSAuthorizationConfig;
    impl crate::value::ToValue for EFSAuthorizationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_point_id {
                properties.insert(
                    "AccessPointId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iam {
                properties.insert("Iam".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-efsvolumeconfiguration.html
    pub struct EFSVolumeConfiguration_ {
        pub authorization_config: Option<Box<EFSAuthorizationConfig_>>,
        pub file_system_id: crate::value::ExpString,
        pub root_directory: Option<crate::value::ExpString>,
        pub transit_encryption: Option<crate::value::ExpString>,
        pub transit_encryption_port: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EFSVolumeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EFSVolumeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EFSVolumeConfiguration as EFSVolumeConfiguration;
    impl crate::value::ToValue for EFSVolumeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authorization_config {
                properties.insert(
                    "AuthorizationConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FileSystemId".to_string(),
                crate::value::ToValue::to_value(&self.file_system_id),
            );
            if let Some(ref value) = self.root_directory {
                properties.insert(
                    "RootDirectory".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transit_encryption {
                properties.insert(
                    "TransitEncryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transit_encryption_port {
                properties.insert(
                    "TransitEncryptionPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ecsproperties.html
    pub struct EcsProperties_ {
        pub task_properties: Vec<EcsTaskProperties_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EcsProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EcsProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EcsProperties as EcsProperties;
    impl crate::value::ToValue for EcsProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TaskProperties".to_string(),
                crate::value::ToValue::to_value(&self.task_properties),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ecstaskproperties.html
    pub struct EcsTaskProperties_ {
        pub containers: Option<Vec<TaskContainerProperties_>>,
        pub enable_execute_command: Option<crate::value::ExpBool>,
        pub ephemeral_storage: Option<Box<EphemeralStorage_>>,
        pub execution_role_arn: Option<crate::value::ExpString>,
        pub ipc_mode: Option<crate::value::ExpString>,
        pub network_configuration: Option<Box<NetworkConfiguration_>>,
        pub pid_mode: Option<crate::value::ExpString>,
        pub platform_version: Option<crate::value::ExpString>,
        pub runtime_platform: Option<Box<RuntimePlatform_>>,
        pub task_role_arn: Option<crate::value::ExpString>,
        pub volumes: Option<Vec<Volume_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EcsTaskProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EcsTaskProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EcsTaskProperties as EcsTaskProperties;
    impl crate::value::ToValue for EcsTaskProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.containers {
                properties.insert(
                    "Containers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_execute_command {
                properties.insert(
                    "EnableExecuteCommand".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ephemeral_storage {
                properties.insert(
                    "EphemeralStorage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execution_role_arn {
                properties.insert(
                    "ExecutionRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipc_mode {
                properties.insert(
                    "IpcMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_configuration {
                properties.insert(
                    "NetworkConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pid_mode {
                properties.insert(
                    "PidMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.platform_version {
                properties.insert(
                    "PlatformVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.runtime_platform {
                properties.insert(
                    "RuntimePlatform".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.task_role_arn {
                properties.insert(
                    "TaskRoleArn".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ekscontainer.html
    pub struct EksContainer_ {
        pub args: Option<Vec<crate::value::ExpString>>,
        pub command: Option<Vec<crate::value::ExpString>>,
        pub env: Option<Vec<EksContainerEnvironmentVariable_>>,
        pub image: crate::value::ExpString,
        pub image_pull_policy: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub resources: Option<Box<EksContainerResourceRequirements_>>,
        pub security_context: Option<Box<EksContainerSecurityContext_>>,
        pub volume_mounts: Option<Vec<EksContainerVolumeMount_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EksContainer {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EksContainer"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EksContainer as EksContainer;
    impl crate::value::ToValue for EksContainer_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.args {
                properties.insert("Args".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.command {
                properties.insert(
                    "Command".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.env {
                properties.insert("Env".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Image".to_string(),
                crate::value::ToValue::to_value(&self.image),
            );
            if let Some(ref value) = self.image_pull_policy {
                properties.insert(
                    "ImagePullPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.resources {
                properties.insert(
                    "Resources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_context {
                properties.insert(
                    "SecurityContext".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.volume_mounts {
                properties.insert(
                    "VolumeMounts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ekscontainerenvironmentvariable.html
    pub struct EksContainerEnvironmentVariable_ {
        pub name: crate::value::ExpString,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EksContainerEnvironmentVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EksContainerEnvironmentVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EksContainerEnvironmentVariable as EksContainerEnvironmentVariable;
    impl crate::value::ToValue for EksContainerEnvironmentVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ekscontainerresourcerequirements.html
    pub struct EksContainerResourceRequirements_ {
        pub limits: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub requests: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EksContainerResourceRequirements {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EksContainerResourceRequirements"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EksContainerResourceRequirements as EksContainerResourceRequirements;
    impl crate::value::ToValue for EksContainerResourceRequirements_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.limits {
                properties.insert("Limits".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.requests {
                properties.insert(
                    "Requests".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ekscontainersecuritycontext.html
    pub struct EksContainerSecurityContext_ {
        pub allow_privilege_escalation: Option<crate::value::ExpBool>,
        pub privileged: Option<crate::value::ExpBool>,
        pub read_only_root_filesystem: Option<crate::value::ExpBool>,
        pub run_as_group: Option<i32>,
        pub run_as_non_root: Option<crate::value::ExpBool>,
        pub run_as_user: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EksContainerSecurityContext {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EksContainerSecurityContext"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EksContainerSecurityContext as EksContainerSecurityContext;
    impl crate::value::ToValue for EksContainerSecurityContext_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_privilege_escalation {
                properties.insert(
                    "AllowPrivilegeEscalation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.privileged {
                properties.insert(
                    "Privileged".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_only_root_filesystem {
                properties.insert(
                    "ReadOnlyRootFilesystem".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.run_as_group {
                properties.insert(
                    "RunAsGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.run_as_non_root {
                properties.insert(
                    "RunAsNonRoot".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.run_as_user {
                properties.insert(
                    "RunAsUser".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ekscontainervolumemount.html
    pub struct EksContainerVolumeMount_ {
        pub mount_path: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub read_only: Option<crate::value::ExpBool>,
        pub sub_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EksContainerVolumeMount {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EksContainerVolumeMount"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EksContainerVolumeMount as EksContainerVolumeMount;
    impl crate::value::ToValue for EksContainerVolumeMount_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mount_path {
                properties.insert(
                    "MountPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.read_only {
                properties.insert(
                    "ReadOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sub_path {
                properties.insert(
                    "SubPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-eksemptydir.html
    pub struct EksEmptyDir_ {
        pub medium: Option<crate::value::ExpString>,
        pub size_limit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EksEmptyDir {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EksEmptyDir"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EksEmptyDir as EksEmptyDir;
    impl crate::value::ToValue for EksEmptyDir_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.medium {
                properties.insert("Medium".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.size_limit {
                properties.insert(
                    "SizeLimit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ekshostpath.html
    pub struct EksHostPath_ {
        pub path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EksHostPath {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EksHostPath"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EksHostPath as EksHostPath;
    impl crate::value::ToValue for EksHostPath_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-eksmetadata.html
    pub struct EksMetadata_ {
        pub annotations: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub labels: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub namespace: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EksMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EksMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EksMetadata as EksMetadata;
    impl crate::value::ToValue for EksMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.annotations {
                properties.insert(
                    "Annotations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.labels {
                properties.insert("Labels".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.namespace {
                properties.insert(
                    "Namespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ekspersistentvolumeclaim.html
    pub struct EksPersistentVolumeClaim_ {
        pub claim_name: crate::value::ExpString,
        pub read_only: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EksPersistentVolumeClaim {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EksPersistentVolumeClaim"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EksPersistentVolumeClaim as EksPersistentVolumeClaim;
    impl crate::value::ToValue for EksPersistentVolumeClaim_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClaimName".to_string(),
                crate::value::ToValue::to_value(&self.claim_name),
            );
            if let Some(ref value) = self.read_only {
                properties.insert(
                    "ReadOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ekspodproperties.html
    pub struct EksPodProperties_ {
        pub containers: Option<Vec<EksContainer_>>,
        pub dns_policy: Option<crate::value::ExpString>,
        pub host_network: Option<crate::value::ExpBool>,
        pub image_pull_secrets: Option<Vec<ImagePullSecret_>>,
        pub init_containers: Option<Vec<EksContainer_>>,
        pub metadata: Option<Box<EksMetadata_>>,
        pub service_account_name: Option<crate::value::ExpString>,
        pub share_process_namespace: Option<crate::value::ExpBool>,
        pub volumes: Option<Vec<EksVolume_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EksPodProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EksPodProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EksPodProperties as EksPodProperties;
    impl crate::value::ToValue for EksPodProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.containers {
                properties.insert(
                    "Containers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dns_policy {
                properties.insert(
                    "DnsPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.host_network {
                properties.insert(
                    "HostNetwork".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image_pull_secrets {
                properties.insert(
                    "ImagePullSecrets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.init_containers {
                properties.insert(
                    "InitContainers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metadata {
                properties.insert(
                    "Metadata".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_account_name {
                properties.insert(
                    "ServiceAccountName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.share_process_namespace {
                properties.insert(
                    "ShareProcessNamespace".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-eksproperties.html
    pub struct EksProperties_ {
        pub pod_properties: Option<Box<EksPodProperties_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EksProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EksProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EksProperties as EksProperties;
    impl crate::value::ToValue for EksProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.pod_properties {
                properties.insert(
                    "PodProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ekssecret.html
    pub struct EksSecret_ {
        pub optional: Option<crate::value::ExpBool>,
        pub secret_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EksSecret {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EksSecret"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EksSecret as EksSecret;
    impl crate::value::ToValue for EksSecret_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.optional {
                properties.insert(
                    "Optional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SecretName".to_string(),
                crate::value::ToValue::to_value(&self.secret_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-eksvolume.html
    pub struct EksVolume_ {
        pub empty_dir: Option<Box<EksEmptyDir_>>,
        pub host_path: Option<Box<EksHostPath_>>,
        pub name: crate::value::ExpString,
        pub persistent_volume_claim: Option<Box<EksPersistentVolumeClaim_>>,
        pub secret: Option<Box<EksSecret_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EksVolume {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EksVolume"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EksVolume as EksVolume;
    impl crate::value::ToValue for EksVolume_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.empty_dir {
                properties.insert(
                    "EmptyDir".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.host_path {
                properties.insert(
                    "HostPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.persistent_volume_claim {
                properties.insert(
                    "PersistentVolumeClaim".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret {
                properties.insert("Secret".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-environment.html
    pub struct Environment_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_Environment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.Environment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_Environment as Environment;
    impl crate::value::ToValue for Environment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ephemeralstorage.html
    pub struct EphemeralStorage_ {
        pub size_in_gi_b: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EphemeralStorage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EphemeralStorage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EphemeralStorage as EphemeralStorage;
    impl crate::value::ToValue for EphemeralStorage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SizeInGiB".to_string(),
                crate::value::ToValue::to_value(&self.size_in_gi_b),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-evaluateonexit.html
    pub struct EvaluateOnExit_ {
        pub action: crate::value::ExpString,
        pub on_exit_code: Option<crate::value::ExpString>,
        pub on_reason: Option<crate::value::ExpString>,
        pub on_status_reason: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_EvaluateOnExit {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.EvaluateOnExit"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_EvaluateOnExit as EvaluateOnExit;
    impl crate::value::ToValue for EvaluateOnExit_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            if let Some(ref value) = self.on_exit_code {
                properties.insert(
                    "OnExitCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_reason {
                properties.insert(
                    "OnReason".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_status_reason {
                properties.insert(
                    "OnStatusReason".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-fargateplatformconfiguration.html
    pub struct FargatePlatformConfiguration_ {
        pub platform_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_FargatePlatformConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.FargatePlatformConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_FargatePlatformConfiguration as FargatePlatformConfiguration;
    impl crate::value::ToValue for FargatePlatformConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.platform_version {
                properties.insert(
                    "PlatformVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-firelensconfiguration.html
    pub struct FirelensConfiguration_ {
        pub options: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_FirelensConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.FirelensConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_FirelensConfiguration as FirelensConfiguration;
    impl crate::value::ToValue for FirelensConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.options {
                properties.insert(
                    "Options".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-host.html
    pub struct Host_ {
        pub source_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_Host {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.Host"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_Host as Host;
    impl crate::value::ToValue for Host_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source_path {
                properties.insert(
                    "SourcePath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-imagepullsecret.html
    pub struct ImagePullSecret_ {
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_ImagePullSecret {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.ImagePullSecret"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_ImagePullSecret as ImagePullSecret;
    impl crate::value::ToValue for ImagePullSecret_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-jobtimeout.html
    pub struct JobTimeout_ {
        pub attempt_duration_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_JobTimeout {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.JobTimeout"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_JobTimeout as JobTimeout;
    impl crate::value::ToValue for JobTimeout_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attempt_duration_seconds {
                properties.insert(
                    "AttemptDurationSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-linuxparameters.html
    pub struct LinuxParameters_ {
        pub devices: Option<Vec<Device_>>,
        pub init_process_enabled: Option<crate::value::ExpBool>,
        pub max_swap: Option<i32>,
        pub shared_memory_size: Option<i32>,
        pub swappiness: Option<i32>,
        pub tmpfs: Option<Vec<Tmpfs_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_LinuxParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.LinuxParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_LinuxParameters as LinuxParameters;
    impl crate::value::ToValue for LinuxParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.devices {
                properties.insert(
                    "Devices".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.init_process_enabled {
                properties.insert(
                    "InitProcessEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_swap {
                properties.insert(
                    "MaxSwap".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shared_memory_size {
                properties.insert(
                    "SharedMemorySize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.swappiness {
                properties.insert(
                    "Swappiness".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tmpfs {
                properties.insert("Tmpfs".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-logconfiguration.html
    pub struct LogConfiguration_ {
        pub log_driver: crate::value::ExpString,
        pub options: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub secret_options: Option<Vec<Secret_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_LogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.LogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_LogConfiguration as LogConfiguration;
    impl crate::value::ToValue for LogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LogDriver".to_string(),
                crate::value::ToValue::to_value(&self.log_driver),
            );
            if let Some(ref value) = self.options {
                properties.insert(
                    "Options".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_options {
                properties.insert(
                    "SecretOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-mountpoint.html
    pub struct MountPoint_ {
        pub container_path: Option<crate::value::ExpString>,
        pub read_only: Option<crate::value::ExpBool>,
        pub source_volume: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_MountPoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.MountPoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_MountPoint as MountPoint;
    impl crate::value::ToValue for MountPoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_path {
                properties.insert(
                    "ContainerPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_only {
                properties.insert(
                    "ReadOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_volume {
                properties.insert(
                    "SourceVolume".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-multinodecontainerproperties.html
    pub struct MultiNodeContainerProperties_ {
        pub command: Option<Vec<crate::value::ExpString>>,
        pub enable_execute_command: Option<crate::value::ExpBool>,
        pub environment: Option<Vec<Environment_>>,
        pub ephemeral_storage: Option<Box<EphemeralStorage_>>,
        pub execution_role_arn: Option<crate::value::ExpString>,
        pub image: crate::value::ExpString,
        pub instance_type: Option<crate::value::ExpString>,
        pub job_role_arn: Option<crate::value::ExpString>,
        pub linux_parameters: Option<Box<LinuxParameters_>>,
        pub log_configuration: Option<Box<LogConfiguration_>>,
        pub memory: Option<i32>,
        pub mount_points: Option<Vec<MountPoint_>>,
        pub privileged: Option<crate::value::ExpBool>,
        pub readonly_root_filesystem: Option<crate::value::ExpBool>,
        pub repository_credentials: Option<Box<RepositoryCredentials_>>,
        pub resource_requirements: Option<Vec<ResourceRequirement_>>,
        pub runtime_platform: Option<Box<RuntimePlatform_>>,
        pub secrets: Option<Vec<Secret_>>,
        pub ulimits: Option<Vec<Ulimit_>>,
        pub user: Option<crate::value::ExpString>,
        pub vcpus: Option<i32>,
        pub volumes: Option<Vec<Volume_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_MultiNodeContainerProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.MultiNodeContainerProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_MultiNodeContainerProperties as MultiNodeContainerProperties;
    impl crate::value::ToValue for MultiNodeContainerProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.command {
                properties.insert(
                    "Command".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_execute_command {
                properties.insert(
                    "EnableExecuteCommand".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ephemeral_storage {
                properties.insert(
                    "EphemeralStorage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execution_role_arn {
                properties.insert(
                    "ExecutionRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Image".to_string(),
                crate::value::ToValue::to_value(&self.image),
            );
            if let Some(ref value) = self.instance_type {
                properties.insert(
                    "InstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.job_role_arn {
                properties.insert(
                    "JobRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.linux_parameters {
                properties.insert(
                    "LinuxParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_configuration {
                properties.insert(
                    "LogConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory {
                properties.insert("Memory".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.mount_points {
                properties.insert(
                    "MountPoints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.privileged {
                properties.insert(
                    "Privileged".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.readonly_root_filesystem {
                properties.insert(
                    "ReadonlyRootFilesystem".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.repository_credentials {
                properties.insert(
                    "RepositoryCredentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_requirements {
                properties.insert(
                    "ResourceRequirements".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.runtime_platform {
                properties.insert(
                    "RuntimePlatform".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets {
                properties.insert(
                    "Secrets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ulimits {
                properties.insert(
                    "Ulimits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user {
                properties.insert("User".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.vcpus {
                properties.insert("Vcpus".to_string(), crate::value::ToValue::to_value(value));
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-multinodeecsproperties.html
    pub struct MultiNodeEcsProperties_ {
        pub task_properties: Vec<MultiNodeEcsTaskProperties_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_MultiNodeEcsProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.MultiNodeEcsProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_MultiNodeEcsProperties as MultiNodeEcsProperties;
    impl crate::value::ToValue for MultiNodeEcsProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TaskProperties".to_string(),
                crate::value::ToValue::to_value(&self.task_properties),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-multinodeecstaskproperties.html
    pub struct MultiNodeEcsTaskProperties_ {
        pub containers: Option<Vec<TaskContainerProperties_>>,
        pub enable_execute_command: Option<crate::value::ExpBool>,
        pub execution_role_arn: Option<crate::value::ExpString>,
        pub ipc_mode: Option<crate::value::ExpString>,
        pub pid_mode: Option<crate::value::ExpString>,
        pub task_role_arn: Option<crate::value::ExpString>,
        pub volumes: Option<Vec<Volume_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_MultiNodeEcsTaskProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.MultiNodeEcsTaskProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_MultiNodeEcsTaskProperties as MultiNodeEcsTaskProperties;
    impl crate::value::ToValue for MultiNodeEcsTaskProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.containers {
                properties.insert(
                    "Containers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_execute_command {
                properties.insert(
                    "EnableExecuteCommand".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execution_role_arn {
                properties.insert(
                    "ExecutionRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipc_mode {
                properties.insert(
                    "IpcMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pid_mode {
                properties.insert(
                    "PidMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.task_role_arn {
                properties.insert(
                    "TaskRoleArn".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-networkconfiguration.html
    pub struct NetworkConfiguration_ {
        pub assign_public_ip: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_NetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.NetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_NetworkConfiguration as NetworkConfiguration;
    impl crate::value::ToValue for NetworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.assign_public_ip {
                properties.insert(
                    "AssignPublicIp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-nodeproperties.html
    pub struct NodeProperties_ {
        pub main_node: i32,
        pub node_range_properties: Vec<NodeRangeProperty_>,
        pub num_nodes: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_NodeProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.NodeProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_NodeProperties as NodeProperties;
    impl crate::value::ToValue for NodeProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MainNode".to_string(),
                crate::value::ToValue::to_value(&self.main_node),
            );
            properties.insert(
                "NodeRangeProperties".to_string(),
                crate::value::ToValue::to_value(&self.node_range_properties),
            );
            properties.insert(
                "NumNodes".to_string(),
                crate::value::ToValue::to_value(&self.num_nodes),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-noderangeproperty.html
    pub struct NodeRangeProperty_ {
        pub consumable_resource_properties: Option<Box<ConsumableResourceProperties_>>,
        pub container: Option<Box<MultiNodeContainerProperties_>>,
        pub ecs_properties: Option<Box<MultiNodeEcsProperties_>>,
        pub eks_properties: Option<Box<EksProperties_>>,
        pub instance_types: Option<Vec<crate::value::ExpString>>,
        pub target_nodes: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_NodeRangeProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.NodeRangeProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_NodeRangeProperty as NodeRangeProperty;
    impl crate::value::ToValue for NodeRangeProperty_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.consumable_resource_properties {
                properties.insert(
                    "ConsumableResourceProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container {
                properties.insert(
                    "Container".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ecs_properties {
                properties.insert(
                    "EcsProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.eks_properties {
                properties.insert(
                    "EksProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_types {
                properties.insert(
                    "InstanceTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetNodes".to_string(),
                crate::value::ToValue::to_value(&self.target_nodes),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-repositorycredentials.html
    pub struct RepositoryCredentials_ {
        pub credentials_parameter: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_RepositoryCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.RepositoryCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_RepositoryCredentials as RepositoryCredentials;
    impl crate::value::ToValue for RepositoryCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CredentialsParameter".to_string(),
                crate::value::ToValue::to_value(&self.credentials_parameter),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-resourcerequirement.html
    pub struct ResourceRequirement_ {
        pub r#type: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_ResourceRequirement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.ResourceRequirement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_ResourceRequirement as ResourceRequirement;
    impl crate::value::ToValue for ResourceRequirement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-resourceretentionpolicy.html
    pub struct ResourceRetentionPolicy_ {
        pub skip_deregister_on_update: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_ResourceRetentionPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.ResourceRetentionPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_ResourceRetentionPolicy as ResourceRetentionPolicy;
    impl crate::value::ToValue for ResourceRetentionPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.skip_deregister_on_update {
                properties.insert(
                    "SkipDeregisterOnUpdate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-retrystrategy.html
    pub struct RetryStrategy_ {
        pub attempts: Option<i32>,
        pub evaluate_on_exit: Option<Vec<EvaluateOnExit_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_RetryStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.RetryStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_RetryStrategy as RetryStrategy;
    impl crate::value::ToValue for RetryStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attempts {
                properties.insert(
                    "Attempts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.evaluate_on_exit {
                properties.insert(
                    "EvaluateOnExit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-runtimeplatform.html
    pub struct RuntimePlatform_ {
        pub cpu_architecture: Option<crate::value::ExpString>,
        pub operating_system_family: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_RuntimePlatform {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.RuntimePlatform"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_RuntimePlatform as RuntimePlatform;
    impl crate::value::ToValue for RuntimePlatform_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cpu_architecture {
                properties.insert(
                    "CpuArchitecture".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.operating_system_family {
                properties.insert(
                    "OperatingSystemFamily".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-secret.html
    pub struct Secret_ {
        pub name: crate::value::ExpString,
        pub value_from: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_Secret {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.Secret"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_Secret as Secret;
    impl crate::value::ToValue for Secret_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "ValueFrom".to_string(),
                crate::value::ToValue::to_value(&self.value_from),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-taskcontainerdependency.html
    pub struct TaskContainerDependency_ {
        pub condition: crate::value::ExpString,
        pub container_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_TaskContainerDependency {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.TaskContainerDependency"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_TaskContainerDependency as TaskContainerDependency;
    impl crate::value::ToValue for TaskContainerDependency_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Condition".to_string(),
                crate::value::ToValue::to_value(&self.condition),
            );
            properties.insert(
                "ContainerName".to_string(),
                crate::value::ToValue::to_value(&self.container_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-taskcontainerproperties.html
    pub struct TaskContainerProperties_ {
        pub command: Option<Vec<crate::value::ExpString>>,
        pub depends_on: Option<Vec<TaskContainerDependency_>>,
        pub environment: Option<Vec<Environment_>>,
        pub essential: Option<crate::value::ExpBool>,
        pub firelens_configuration: Option<Box<FirelensConfiguration_>>,
        pub image: crate::value::ExpString,
        pub linux_parameters: Option<Box<LinuxParameters_>>,
        pub log_configuration: Option<Box<LogConfiguration_>>,
        pub mount_points: Option<Vec<MountPoint_>>,
        pub name: Option<crate::value::ExpString>,
        pub privileged: Option<crate::value::ExpBool>,
        pub readonly_root_filesystem: Option<crate::value::ExpBool>,
        pub repository_credentials: Option<Box<RepositoryCredentials_>>,
        pub resource_requirements: Option<Vec<ResourceRequirement_>>,
        pub secrets: Option<Vec<Secret_>>,
        pub ulimits: Option<Vec<Ulimit_>>,
        pub user: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_TaskContainerProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.TaskContainerProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_TaskContainerProperties as TaskContainerProperties;
    impl crate::value::ToValue for TaskContainerProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.command {
                properties.insert(
                    "Command".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.depends_on {
                properties.insert(
                    "DependsOn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.essential {
                properties.insert(
                    "Essential".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.firelens_configuration {
                properties.insert(
                    "FirelensConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Image".to_string(),
                crate::value::ToValue::to_value(&self.image),
            );
            if let Some(ref value) = self.linux_parameters {
                properties.insert(
                    "LinuxParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_configuration {
                properties.insert(
                    "LogConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mount_points {
                properties.insert(
                    "MountPoints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.privileged {
                properties.insert(
                    "Privileged".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.readonly_root_filesystem {
                properties.insert(
                    "ReadonlyRootFilesystem".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.repository_credentials {
                properties.insert(
                    "RepositoryCredentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_requirements {
                properties.insert(
                    "ResourceRequirements".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets {
                properties.insert(
                    "Secrets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ulimits {
                properties.insert(
                    "Ulimits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user {
                properties.insert("User".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-tmpfs.html
    pub struct Tmpfs_ {
        pub container_path: crate::value::ExpString,
        pub mount_options: Option<Vec<crate::value::ExpString>>,
        pub size: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_Tmpfs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.Tmpfs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_Tmpfs as Tmpfs;
    impl crate::value::ToValue for Tmpfs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContainerPath".to_string(),
                crate::value::ToValue::to_value(&self.container_path),
            );
            if let Some(ref value) = self.mount_options {
                properties.insert(
                    "MountOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Size".to_string(),
                crate::value::ToValue::to_value(&self.size),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ulimit.html
    pub struct Ulimit_ {
        pub hard_limit: i32,
        pub name: crate::value::ExpString,
        pub soft_limit: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_Ulimit {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.Ulimit"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_Ulimit as Ulimit;
    impl crate::value::ToValue for Ulimit_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HardLimit".to_string(),
                crate::value::ToValue::to_value(&self.hard_limit),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "SoftLimit".to_string(),
                crate::value::ToValue::to_value(&self.soft_limit),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-volume.html
    pub struct Volume_ {
        pub efs_volume_configuration: Option<Box<EFSVolumeConfiguration_>>,
        pub host: Option<Box<Host_>>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobDefinition_Volume {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobDefinition.Volume"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobDefinition_Volume as Volume;
    impl crate::value::ToValue for Volume_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.efs_volume_configuration {
                properties.insert(
                    "EfsVolumeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.host {
                properties.insert("Host".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod jobqueue {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobqueue-computeenvironmentorder.html
    pub struct ComputeEnvironmentOrder_ {
        pub compute_environment: crate::value::ExpString,
        pub order: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobQueue_ComputeEnvironmentOrder {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobQueue.ComputeEnvironmentOrder"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobQueue_ComputeEnvironmentOrder as ComputeEnvironmentOrder;
    impl crate::value::ToValue for ComputeEnvironmentOrder_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ComputeEnvironment".to_string(),
                crate::value::ToValue::to_value(&self.compute_environment),
            );
            properties.insert(
                "Order".to_string(),
                crate::value::ToValue::to_value(&self.order),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobqueue-jobstatetimelimitaction.html
    pub struct JobStateTimeLimitAction_ {
        pub action: crate::value::ExpString,
        pub max_time_seconds: i32,
        pub reason: crate::value::ExpString,
        pub state: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobQueue_JobStateTimeLimitAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobQueue.JobStateTimeLimitAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobQueue_JobStateTimeLimitAction as JobStateTimeLimitAction;
    impl crate::value::ToValue for JobStateTimeLimitAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "MaxTimeSeconds".to_string(),
                crate::value::ToValue::to_value(&self.max_time_seconds),
            );
            properties.insert(
                "Reason".to_string(),
                crate::value::ToValue::to_value(&self.reason),
            );
            properties.insert(
                "State".to_string(),
                crate::value::ToValue::to_value(&self.state),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobqueue-serviceenvironmentorder.html
    pub struct ServiceEnvironmentOrder_ {
        pub order: i32,
        pub service_environment: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_JobQueue_ServiceEnvironmentOrder {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::JobQueue.ServiceEnvironmentOrder"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_JobQueue_ServiceEnvironmentOrder as ServiceEnvironmentOrder;
    impl crate::value::ToValue for ServiceEnvironmentOrder_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Order".to_string(),
                crate::value::ToValue::to_value(&self.order),
            );
            properties.insert(
                "ServiceEnvironment".to_string(),
                crate::value::ToValue::to_value(&self.service_environment),
            );
            properties.into()
        }
    }
}
pub mod schedulingpolicy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-schedulingpolicy-fairsharepolicy.html
    pub struct FairsharePolicy_ {
        pub compute_reservation: Option<f64>,
        pub share_decay_seconds: Option<f64>,
        pub share_distribution: Option<Vec<ShareAttributes_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_SchedulingPolicy_FairsharePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::SchedulingPolicy.FairsharePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_SchedulingPolicy_FairsharePolicy as FairsharePolicy;
    impl crate::value::ToValue for FairsharePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.compute_reservation {
                properties.insert(
                    "ComputeReservation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.share_decay_seconds {
                properties.insert(
                    "ShareDecaySeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.share_distribution {
                properties.insert(
                    "ShareDistribution".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-schedulingpolicy-shareattributes.html
    pub struct ShareAttributes_ {
        pub share_identifier: Option<crate::value::ExpString>,
        pub weight_factor: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_SchedulingPolicy_ShareAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::SchedulingPolicy.ShareAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_SchedulingPolicy_ShareAttributes as ShareAttributes;
    impl crate::value::ToValue for ShareAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.share_identifier {
                properties.insert(
                    "ShareIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weight_factor {
                properties.insert(
                    "WeightFactor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod serviceenvironment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-serviceenvironment-capacitylimit.html
    pub struct CapacityLimit_ {
        pub capacity_unit: Option<crate::value::ExpString>,
        pub max_capacity: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_batch_ServiceEnvironment_CapacityLimit {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Batch::ServiceEnvironment.CapacityLimit"
            $($field $value)*)
        };
    }
    pub use crate::__aws_batch_ServiceEnvironment_CapacityLimit as CapacityLimit;
    impl crate::value::ToValue for CapacityLimit_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity_unit {
                properties.insert(
                    "CapacityUnit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_capacity {
                properties.insert(
                    "MaxCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-computeenvironment.html
pub struct ComputeEnvironment_ {
    pub compute_environment_name: Option<crate::value::ExpString>,
    pub compute_resources: Option<super::batch::computeenvironment::ComputeResources_>,
    pub context: Option<crate::value::ExpString>,
    pub eks_configuration: Option<super::batch::computeenvironment::EksConfiguration_>,
    pub replace_compute_environment: Option<crate::value::ExpBool>,
    pub service_role: Option<crate::value::ExpString>,
    pub state: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub r#type: crate::value::ExpString,
    pub unmanagedv_cpus: Option<i32>,
    pub update_policy: Option<super::batch::computeenvironment::UpdatePolicy_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_batch_ComputeEnvironment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Batch::ComputeEnvironment"
        $($field $value)*)
    };
}
pub use crate::__aws_batch_ComputeEnvironment as ComputeEnvironment;
impl crate::template::ToResource for ComputeEnvironment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Batch"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ComputeEnvironment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.compute_environment_name {
            properties.insert(
                "ComputeEnvironmentName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.compute_resources {
            properties.insert(
                "ComputeResources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.context {
            properties.insert(
                "Context".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.eks_configuration {
            properties.insert(
                "EksConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.replace_compute_environment {
            properties.insert(
                "ReplaceComputeEnvironment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_role {
            properties.insert(
                "ServiceRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.state {
            properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        if let Some(ref value) = self.unmanagedv_cpus {
            properties.insert(
                "UnmanagedvCpus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.update_policy {
            properties.insert(
                "UpdatePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-consumableresource.html
pub struct ConsumableResource_ {
    pub consumable_resource_name: Option<crate::value::ExpString>,
    pub resource_type: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub total_quantity: i64,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_batch_ConsumableResource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Batch::ConsumableResource"
        $($field $value)*)
    };
}
pub use crate::__aws_batch_ConsumableResource as ConsumableResource;
impl crate::template::ToResource for ConsumableResource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Batch"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConsumableResource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.consumable_resource_name {
            properties.insert(
                "ConsumableResourceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResourceType".to_string(),
            crate::value::ToValue::to_value(&self.resource_type),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TotalQuantity".to_string(),
            crate::value::ToValue::to_value(&self.total_quantity),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobdefinition.html
pub struct JobDefinition_ {
    pub consumable_resource_properties:
        Option<super::batch::jobdefinition::ConsumableResourceProperties_>,
    pub container_properties: Option<super::batch::jobdefinition::ContainerProperties_>,
    pub ecs_properties: Option<super::batch::jobdefinition::EcsProperties_>,
    pub eks_properties: Option<super::batch::jobdefinition::EksProperties_>,
    pub job_definition_name: Option<crate::value::ExpString>,
    pub node_properties: Option<super::batch::jobdefinition::NodeProperties_>,
    pub parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub platform_capabilities: Option<Vec<crate::value::ExpString>>,
    pub propagate_tags: Option<crate::value::ExpBool>,
    pub resource_retention_policy: Option<super::batch::jobdefinition::ResourceRetentionPolicy_>,
    pub retry_strategy: Option<super::batch::jobdefinition::RetryStrategy_>,
    pub scheduling_priority: Option<i32>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub timeout: Option<super::batch::jobdefinition::JobTimeout_>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_batch_JobDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Batch::JobDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_batch_JobDefinition as JobDefinition;
impl crate::template::ToResource for JobDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Batch"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("JobDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.consumable_resource_properties {
            properties.insert(
                "ConsumableResourceProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.container_properties {
            properties.insert(
                "ContainerProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ecs_properties {
            properties.insert(
                "EcsProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.eks_properties {
            properties.insert(
                "EksProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_definition_name {
            properties.insert(
                "JobDefinitionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.node_properties {
            properties.insert(
                "NodeProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.parameters {
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.platform_capabilities {
            properties.insert(
                "PlatformCapabilities".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.propagate_tags {
            properties.insert(
                "PropagateTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_retention_policy {
            properties.insert(
                "ResourceRetentionPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.retry_strategy {
            properties.insert(
                "RetryStrategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scheduling_priority {
            properties.insert(
                "SchedulingPriority".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.timeout {
            properties.insert(
                "Timeout".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobqueue.html
pub struct JobQueue_ {
    pub compute_environment_order: Option<Vec<super::batch::jobqueue::ComputeEnvironmentOrder_>>,
    pub job_queue_name: Option<crate::value::ExpString>,
    pub job_queue_type: Option<crate::value::ExpString>,
    pub job_state_time_limit_actions: Option<Vec<super::batch::jobqueue::JobStateTimeLimitAction_>>,
    pub priority: i32,
    pub scheduling_policy_arn: Option<crate::value::ExpString>,
    pub service_environment_order: Option<Vec<super::batch::jobqueue::ServiceEnvironmentOrder_>>,
    pub state: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_batch_JobQueue {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Batch::JobQueue" $($field
        $value)*)
    };
}
pub use crate::__aws_batch_JobQueue as JobQueue;
impl crate::template::ToResource for JobQueue_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Batch"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("JobQueue"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.compute_environment_order {
            properties.insert(
                "ComputeEnvironmentOrder".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_queue_name {
            properties.insert(
                "JobQueueName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_queue_type {
            properties.insert(
                "JobQueueType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_state_time_limit_actions {
            properties.insert(
                "JobStateTimeLimitActions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Priority".to_string(),
            crate::value::ToValue::to_value(&self.priority),
        );
        if let Some(ref value) = self.scheduling_policy_arn {
            properties.insert(
                "SchedulingPolicyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_environment_order {
            properties.insert(
                "ServiceEnvironmentOrder".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.state {
            properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-schedulingpolicy.html
pub struct SchedulingPolicy_ {
    pub fairshare_policy: Option<super::batch::schedulingpolicy::FairsharePolicy_>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_batch_SchedulingPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Batch::SchedulingPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_batch_SchedulingPolicy as SchedulingPolicy;
impl crate::template::ToResource for SchedulingPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Batch"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SchedulingPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.fairshare_policy {
            properties.insert(
                "FairsharePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-serviceenvironment.html
pub struct ServiceEnvironment_ {
    pub capacity_limits: Vec<super::batch::serviceenvironment::CapacityLimit_>,
    pub service_environment_name: Option<crate::value::ExpString>,
    pub service_environment_type: crate::value::ExpString,
    pub state: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_batch_ServiceEnvironment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Batch::ServiceEnvironment"
        $($field $value)*)
    };
}
pub use crate::__aws_batch_ServiceEnvironment as ServiceEnvironment;
impl crate::template::ToResource for ServiceEnvironment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Batch"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ServiceEnvironment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CapacityLimits".to_string(),
            crate::value::ToValue::to_value(&self.capacity_limits),
        );
        if let Some(ref value) = self.service_environment_name {
            properties.insert(
                "ServiceEnvironmentName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ServiceEnvironmentType".to_string(),
            crate::value::ToValue::to_value(&self.service_environment_type),
        );
        if let Some(ref value) = self.state {
            properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
