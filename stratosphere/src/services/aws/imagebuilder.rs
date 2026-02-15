pub mod component {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-component-latestversion.html
    pub struct LatestVersion_ {
        pub arn: Option<crate::value::ExpString>,
        pub major: Option<crate::value::ExpString>,
        pub minor: Option<crate::value::ExpString>,
        pub patch: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_Component_LatestVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::Component.LatestVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_Component_LatestVersion as LatestVersion;
    impl crate::value::ToValue for LatestVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.major {
                properties.insert("Major".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.minor {
                properties.insert("Minor".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.patch {
                properties.insert("Patch".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod containerrecipe {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-componentconfiguration.html
    pub struct ComponentConfiguration_ {
        pub component_arn: Option<crate::value::ExpString>,
        pub parameters: Option<Vec<ComponentParameter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ContainerRecipe_ComponentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ContainerRecipe.ComponentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ContainerRecipe_ComponentConfiguration as ComponentConfiguration;
    impl crate::value::ToValue for ComponentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.component_arn {
                properties.insert(
                    "ComponentArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-componentparameter.html
    pub struct ComponentParameter_ {
        pub name: crate::value::ExpString,
        pub value: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ContainerRecipe_ComponentParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ContainerRecipe.ComponentParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ContainerRecipe_ComponentParameter as ComponentParameter;
    impl crate::value::ToValue for ComponentParameter_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-ebsinstanceblockdevicespecification.html
    pub struct EbsInstanceBlockDeviceSpecification_ {
        pub delete_on_termination: Option<crate::value::ExpBool>,
        pub encrypted: Option<crate::value::ExpBool>,
        pub iops: Option<i32>,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub snapshot_id: Option<crate::value::ExpString>,
        pub throughput: Option<i32>,
        pub volume_size: Option<i32>,
        pub volume_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ContainerRecipe_EbsInstanceBlockDeviceSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ContainerRecipe.EbsInstanceBlockDeviceSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ContainerRecipe_EbsInstanceBlockDeviceSpecification as EbsInstanceBlockDeviceSpecification;
    impl crate::value::ToValue for EbsInstanceBlockDeviceSpecification_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-instanceblockdevicemapping.html
    pub struct InstanceBlockDeviceMapping_ {
        pub device_name: Option<crate::value::ExpString>,
        pub ebs: Option<Box<EbsInstanceBlockDeviceSpecification_>>,
        pub no_device: Option<crate::value::ExpString>,
        pub virtual_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ContainerRecipe_InstanceBlockDeviceMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ContainerRecipe.InstanceBlockDeviceMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ContainerRecipe_InstanceBlockDeviceMapping as InstanceBlockDeviceMapping;
    impl crate::value::ToValue for InstanceBlockDeviceMapping_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-instanceconfiguration.html
    pub struct InstanceConfiguration_ {
        pub block_device_mappings: Option<Vec<InstanceBlockDeviceMapping_>>,
        pub image: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ContainerRecipe_InstanceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ContainerRecipe.InstanceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ContainerRecipe_InstanceConfiguration as InstanceConfiguration;
    impl crate::value::ToValue for InstanceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.block_device_mappings {
                properties.insert(
                    "BlockDeviceMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image {
                properties.insert("Image".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-latestversion.html
    pub struct LatestVersion_ {
        pub arn: Option<crate::value::ExpString>,
        pub major: Option<crate::value::ExpString>,
        pub minor: Option<crate::value::ExpString>,
        pub patch: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ContainerRecipe_LatestVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ContainerRecipe.LatestVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ContainerRecipe_LatestVersion as LatestVersion;
    impl crate::value::ToValue for LatestVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.major {
                properties.insert("Major".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.minor {
                properties.insert("Minor".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.patch {
                properties.insert("Patch".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-containerrecipe-targetcontainerrepository.html
    pub struct TargetContainerRepository_ {
        pub repository_name: Option<crate::value::ExpString>,
        pub service: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ContainerRecipe_TargetContainerRepository {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ContainerRecipe.TargetContainerRepository"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ContainerRecipe_TargetContainerRepository as TargetContainerRepository;
    impl crate::value::ToValue for TargetContainerRepository_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.repository_name {
                properties.insert(
                    "RepositoryName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service {
                properties.insert(
                    "Service".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod distributionconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-amidistributionconfiguration.html
    pub struct AmiDistributionConfiguration_ {
        pub ami_tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub description: Option<crate::value::ExpString>,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub launch_permission_configuration: Option<Box<LaunchPermissionConfiguration_>>,
        pub name: Option<crate::value::ExpString>,
        pub target_account_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_DistributionConfiguration_AmiDistributionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::DistributionConfiguration.AmiDistributionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_DistributionConfiguration_AmiDistributionConfiguration as AmiDistributionConfiguration;
    impl crate::value::ToValue for AmiDistributionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ami_tags {
                properties.insert(
                    "AmiTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.launch_permission_configuration {
                properties.insert(
                    "LaunchPermissionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.target_account_ids {
                properties.insert(
                    "TargetAccountIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-containerdistributionconfiguration.html
    pub struct ContainerDistributionConfiguration_ {
        pub container_tags: Option<Vec<crate::value::ExpString>>,
        pub description: Option<crate::value::ExpString>,
        pub target_repository: Option<Box<TargetContainerRepository_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_DistributionConfiguration_ContainerDistributionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::DistributionConfiguration.ContainerDistributionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_DistributionConfiguration_ContainerDistributionConfiguration as ContainerDistributionConfiguration;
    impl crate::value::ToValue for ContainerDistributionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_tags {
                properties.insert(
                    "ContainerTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_repository {
                properties.insert(
                    "TargetRepository".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-distribution.html
    pub struct Distribution_ {
        pub ami_distribution_configuration: Option<Box<AmiDistributionConfiguration_>>,
        pub container_distribution_configuration: Option<Box<ContainerDistributionConfiguration_>>,
        pub fast_launch_configurations: Option<Vec<FastLaunchConfiguration_>>,
        pub launch_template_configurations: Option<Vec<LaunchTemplateConfiguration_>>,
        pub license_configuration_arns: Option<Vec<crate::value::ExpString>>,
        pub region: crate::value::ExpString,
        pub ssm_parameter_configurations: Option<Vec<SsmParameterConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_DistributionConfiguration_Distribution {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::DistributionConfiguration.Distribution"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_DistributionConfiguration_Distribution as Distribution;
    impl crate::value::ToValue for Distribution_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ami_distribution_configuration {
                properties.insert(
                    "AmiDistributionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_distribution_configuration {
                properties.insert(
                    "ContainerDistributionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fast_launch_configurations {
                properties.insert(
                    "FastLaunchConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.launch_template_configurations {
                properties.insert(
                    "LaunchTemplateConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.license_configuration_arns {
                properties.insert(
                    "LicenseConfigurationArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Region".to_string(),
                crate::value::ToValue::to_value(&self.region),
            );
            if let Some(ref value) = self.ssm_parameter_configurations {
                properties.insert(
                    "SsmParameterConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-fastlaunchconfiguration.html
    pub struct FastLaunchConfiguration_ {
        pub account_id: Option<crate::value::ExpString>,
        pub enabled: Option<crate::value::ExpBool>,
        pub launch_template: Option<Box<FastLaunchLaunchTemplateSpecification_>>,
        pub max_parallel_launches: Option<i32>,
        pub snapshot_configuration: Option<Box<FastLaunchSnapshotConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_DistributionConfiguration_FastLaunchConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::DistributionConfiguration.FastLaunchConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_DistributionConfiguration_FastLaunchConfiguration as FastLaunchConfiguration;
    impl crate::value::ToValue for FastLaunchConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.account_id {
                properties.insert(
                    "AccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.launch_template {
                properties.insert(
                    "LaunchTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_parallel_launches {
                properties.insert(
                    "MaxParallelLaunches".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.snapshot_configuration {
                properties.insert(
                    "SnapshotConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-fastlaunchlaunchtemplatespecification.html
    pub struct FastLaunchLaunchTemplateSpecification_ {
        pub launch_template_id: Option<crate::value::ExpString>,
        pub launch_template_name: Option<crate::value::ExpString>,
        pub launch_template_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_DistributionConfiguration_FastLaunchLaunchTemplateSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::DistributionConfiguration.FastLaunchLaunchTemplateSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_DistributionConfiguration_FastLaunchLaunchTemplateSpecification as FastLaunchLaunchTemplateSpecification;
    impl crate::value::ToValue for FastLaunchLaunchTemplateSpecification_ {
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
            if let Some(ref value) = self.launch_template_version {
                properties.insert(
                    "LaunchTemplateVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-fastlaunchsnapshotconfiguration.html
    pub struct FastLaunchSnapshotConfiguration_ {
        pub target_resource_count: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_DistributionConfiguration_FastLaunchSnapshotConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::DistributionConfiguration.FastLaunchSnapshotConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_DistributionConfiguration_FastLaunchSnapshotConfiguration as FastLaunchSnapshotConfiguration;
    impl crate::value::ToValue for FastLaunchSnapshotConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.target_resource_count {
                properties.insert(
                    "TargetResourceCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-launchpermissionconfiguration.html
    pub struct LaunchPermissionConfiguration_ {
        pub organization_arns: Option<Vec<crate::value::ExpString>>,
        pub organizational_unit_arns: Option<Vec<crate::value::ExpString>>,
        pub user_groups: Option<Vec<crate::value::ExpString>>,
        pub user_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_DistributionConfiguration_LaunchPermissionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::DistributionConfiguration.LaunchPermissionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_DistributionConfiguration_LaunchPermissionConfiguration as LaunchPermissionConfiguration;
    impl crate::value::ToValue for LaunchPermissionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.organization_arns {
                properties.insert(
                    "OrganizationArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.organizational_unit_arns {
                properties.insert(
                    "OrganizationalUnitArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_groups {
                properties.insert(
                    "UserGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_ids {
                properties.insert(
                    "UserIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-launchtemplateconfiguration.html
    pub struct LaunchTemplateConfiguration_ {
        pub account_id: Option<crate::value::ExpString>,
        pub launch_template_id: Option<crate::value::ExpString>,
        pub set_default_version: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_DistributionConfiguration_LaunchTemplateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::DistributionConfiguration.LaunchTemplateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_DistributionConfiguration_LaunchTemplateConfiguration as LaunchTemplateConfiguration;
    impl crate::value::ToValue for LaunchTemplateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.account_id {
                properties.insert(
                    "AccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.launch_template_id {
                properties.insert(
                    "LaunchTemplateId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.set_default_version {
                properties.insert(
                    "SetDefaultVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-ssmparameterconfiguration.html
    pub struct SsmParameterConfiguration_ {
        pub ami_account_id: Option<crate::value::ExpString>,
        pub data_type: Option<crate::value::ExpString>,
        pub parameter_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_DistributionConfiguration_SsmParameterConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::DistributionConfiguration.SsmParameterConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_DistributionConfiguration_SsmParameterConfiguration as SsmParameterConfiguration;
    impl crate::value::ToValue for SsmParameterConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ami_account_id {
                properties.insert(
                    "AmiAccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_type {
                properties.insert(
                    "DataType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ParameterName".to_string(),
                crate::value::ToValue::to_value(&self.parameter_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-distributionconfiguration-targetcontainerrepository.html
    pub struct TargetContainerRepository_ {
        pub repository_name: Option<crate::value::ExpString>,
        pub service: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_DistributionConfiguration_TargetContainerRepository {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::DistributionConfiguration.TargetContainerRepository"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_DistributionConfiguration_TargetContainerRepository as TargetContainerRepository;
    impl crate::value::ToValue for TargetContainerRepository_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.repository_name {
                properties.insert(
                    "RepositoryName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service {
                properties.insert(
                    "Service".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod image {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-image-deletionsettings.html
    pub struct DeletionSettings_ {
        pub execution_role: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_Image_DeletionSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::Image.DeletionSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_Image_DeletionSettings as DeletionSettings;
    impl crate::value::ToValue for DeletionSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ExecutionRole".to_string(),
                crate::value::ToValue::to_value(&self.execution_role),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-image-ecrconfiguration.html
    pub struct EcrConfiguration_ {
        pub container_tags: Option<Vec<crate::value::ExpString>>,
        pub repository_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_Image_EcrConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::Image.EcrConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_Image_EcrConfiguration as EcrConfiguration;
    impl crate::value::ToValue for EcrConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_tags {
                properties.insert(
                    "ContainerTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.repository_name {
                properties.insert(
                    "RepositoryName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-image-imageloggingconfiguration.html
    pub struct ImageLoggingConfiguration_ {
        pub log_group_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_Image_ImageLoggingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::Image.ImageLoggingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_Image_ImageLoggingConfiguration as ImageLoggingConfiguration;
    impl crate::value::ToValue for ImageLoggingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_group_name {
                properties.insert(
                    "LogGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-image-imagepipelineexecutionsettings.html
    pub struct ImagePipelineExecutionSettings_ {
        pub deployment_id: Option<crate::value::ExpString>,
        pub on_update: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_Image_ImagePipelineExecutionSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::Image.ImagePipelineExecutionSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_Image_ImagePipelineExecutionSettings as ImagePipelineExecutionSettings;
    impl crate::value::ToValue for ImagePipelineExecutionSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.deployment_id {
                properties.insert(
                    "DeploymentId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_update {
                properties.insert(
                    "OnUpdate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-image-imagescanningconfiguration.html
    pub struct ImageScanningConfiguration_ {
        pub ecr_configuration: Option<Box<EcrConfiguration_>>,
        pub image_scanning_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_Image_ImageScanningConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::Image.ImageScanningConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_Image_ImageScanningConfiguration as ImageScanningConfiguration;
    impl crate::value::ToValue for ImageScanningConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ecr_configuration {
                properties.insert(
                    "EcrConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image_scanning_enabled {
                properties.insert(
                    "ImageScanningEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-image-imagetestsconfiguration.html
    pub struct ImageTestsConfiguration_ {
        pub image_tests_enabled: Option<crate::value::ExpBool>,
        pub timeout_minutes: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_Image_ImageTestsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::Image.ImageTestsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_Image_ImageTestsConfiguration as ImageTestsConfiguration;
    impl crate::value::ToValue for ImageTestsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.image_tests_enabled {
                properties.insert(
                    "ImageTestsEnabled".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-image-latestversion.html
    pub struct LatestVersion_ {
        pub arn: Option<crate::value::ExpString>,
        pub major: Option<crate::value::ExpString>,
        pub minor: Option<crate::value::ExpString>,
        pub patch: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_Image_LatestVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::Image.LatestVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_Image_LatestVersion as LatestVersion;
    impl crate::value::ToValue for LatestVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.major {
                properties.insert("Major".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.minor {
                properties.insert("Minor".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.patch {
                properties.insert("Patch".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-image-workflowconfiguration.html
    pub struct WorkflowConfiguration_ {
        pub on_failure: Option<crate::value::ExpString>,
        pub parallel_group: Option<crate::value::ExpString>,
        pub parameters: Option<Vec<WorkflowParameter_>>,
        pub workflow_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_Image_WorkflowConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::Image.WorkflowConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_Image_WorkflowConfiguration as WorkflowConfiguration;
    impl crate::value::ToValue for WorkflowConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.on_failure {
                properties.insert(
                    "OnFailure".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parallel_group {
                properties.insert(
                    "ParallelGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.workflow_arn {
                properties.insert(
                    "WorkflowArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-image-workflowparameter.html
    pub struct WorkflowParameter_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_Image_WorkflowParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::Image.WorkflowParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_Image_WorkflowParameter as WorkflowParameter;
    impl crate::value::ToValue for WorkflowParameter_ {
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
}
pub mod imagepipeline {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagepipeline-autodisablepolicy.html
    pub struct AutoDisablePolicy_ {
        pub failure_count: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ImagePipeline_AutoDisablePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ImagePipeline.AutoDisablePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ImagePipeline_AutoDisablePolicy as AutoDisablePolicy;
    impl crate::value::ToValue for AutoDisablePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FailureCount".to_string(),
                crate::value::ToValue::to_value(&self.failure_count),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagepipeline-ecrconfiguration.html
    pub struct EcrConfiguration_ {
        pub container_tags: Option<Vec<crate::value::ExpString>>,
        pub repository_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ImagePipeline_EcrConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ImagePipeline.EcrConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ImagePipeline_EcrConfiguration as EcrConfiguration;
    impl crate::value::ToValue for EcrConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_tags {
                properties.insert(
                    "ContainerTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.repository_name {
                properties.insert(
                    "RepositoryName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagepipeline-imagescanningconfiguration.html
    pub struct ImageScanningConfiguration_ {
        pub ecr_configuration: Option<Box<EcrConfiguration_>>,
        pub image_scanning_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ImagePipeline_ImageScanningConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ImagePipeline.ImageScanningConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ImagePipeline_ImageScanningConfiguration as ImageScanningConfiguration;
    impl crate::value::ToValue for ImageScanningConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ecr_configuration {
                properties.insert(
                    "EcrConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image_scanning_enabled {
                properties.insert(
                    "ImageScanningEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagepipeline-imagetestsconfiguration.html
    pub struct ImageTestsConfiguration_ {
        pub image_tests_enabled: Option<crate::value::ExpBool>,
        pub timeout_minutes: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ImagePipeline_ImageTestsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ImagePipeline.ImageTestsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ImagePipeline_ImageTestsConfiguration as ImageTestsConfiguration;
    impl crate::value::ToValue for ImageTestsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.image_tests_enabled {
                properties.insert(
                    "ImageTestsEnabled".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagepipeline-pipelineloggingconfiguration.html
    pub struct PipelineLoggingConfiguration_ {
        pub image_log_group_name: Option<crate::value::ExpString>,
        pub pipeline_log_group_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ImagePipeline_PipelineLoggingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ImagePipeline.PipelineLoggingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ImagePipeline_PipelineLoggingConfiguration as PipelineLoggingConfiguration;
    impl crate::value::ToValue for PipelineLoggingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.image_log_group_name {
                properties.insert(
                    "ImageLogGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pipeline_log_group_name {
                properties.insert(
                    "PipelineLogGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagepipeline-schedule.html
    pub struct Schedule_ {
        pub auto_disable_policy: Option<Box<AutoDisablePolicy_>>,
        pub pipeline_execution_start_condition: Option<crate::value::ExpString>,
        pub schedule_expression: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ImagePipeline_Schedule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ImagePipeline.Schedule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ImagePipeline_Schedule as Schedule;
    impl crate::value::ToValue for Schedule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_disable_policy {
                properties.insert(
                    "AutoDisablePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pipeline_execution_start_condition {
                properties.insert(
                    "PipelineExecutionStartCondition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schedule_expression {
                properties.insert(
                    "ScheduleExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagepipeline-workflowconfiguration.html
    pub struct WorkflowConfiguration_ {
        pub on_failure: Option<crate::value::ExpString>,
        pub parallel_group: Option<crate::value::ExpString>,
        pub parameters: Option<Vec<WorkflowParameter_>>,
        pub workflow_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ImagePipeline_WorkflowConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ImagePipeline.WorkflowConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ImagePipeline_WorkflowConfiguration as WorkflowConfiguration;
    impl crate::value::ToValue for WorkflowConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.on_failure {
                properties.insert(
                    "OnFailure".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parallel_group {
                properties.insert(
                    "ParallelGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.workflow_arn {
                properties.insert(
                    "WorkflowArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagepipeline-workflowparameter.html
    pub struct WorkflowParameter_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ImagePipeline_WorkflowParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ImagePipeline.WorkflowParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ImagePipeline_WorkflowParameter as WorkflowParameter;
    impl crate::value::ToValue for WorkflowParameter_ {
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
}
pub mod imagerecipe {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-additionalinstanceconfiguration.html
    pub struct AdditionalInstanceConfiguration_ {
        pub systems_manager_agent: Option<Box<SystemsManagerAgent_>>,
        pub user_data_override: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ImageRecipe_AdditionalInstanceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ImageRecipe.AdditionalInstanceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ImageRecipe_AdditionalInstanceConfiguration as AdditionalInstanceConfiguration;
    impl crate::value::ToValue for AdditionalInstanceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.systems_manager_agent {
                properties.insert(
                    "SystemsManagerAgent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_data_override {
                properties.insert(
                    "UserDataOverride".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-componentconfiguration.html
    pub struct ComponentConfiguration_ {
        pub component_arn: Option<crate::value::ExpString>,
        pub parameters: Option<Vec<ComponentParameter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ImageRecipe_ComponentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ImageRecipe.ComponentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ImageRecipe_ComponentConfiguration as ComponentConfiguration;
    impl crate::value::ToValue for ComponentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.component_arn {
                properties.insert(
                    "ComponentArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-componentparameter.html
    pub struct ComponentParameter_ {
        pub name: crate::value::ExpString,
        pub value: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ImageRecipe_ComponentParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ImageRecipe.ComponentParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ImageRecipe_ComponentParameter as ComponentParameter;
    impl crate::value::ToValue for ComponentParameter_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-ebsinstanceblockdevicespecification.html
    pub struct EbsInstanceBlockDeviceSpecification_ {
        pub delete_on_termination: Option<crate::value::ExpBool>,
        pub encrypted: Option<crate::value::ExpBool>,
        pub iops: Option<i32>,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub snapshot_id: Option<crate::value::ExpString>,
        pub throughput: Option<i32>,
        pub volume_size: Option<i32>,
        pub volume_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ImageRecipe_EbsInstanceBlockDeviceSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ImageRecipe.EbsInstanceBlockDeviceSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ImageRecipe_EbsInstanceBlockDeviceSpecification as EbsInstanceBlockDeviceSpecification;
    impl crate::value::ToValue for EbsInstanceBlockDeviceSpecification_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-instanceblockdevicemapping.html
    pub struct InstanceBlockDeviceMapping_ {
        pub device_name: Option<crate::value::ExpString>,
        pub ebs: Option<Box<EbsInstanceBlockDeviceSpecification_>>,
        pub no_device: Option<crate::value::ExpString>,
        pub virtual_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ImageRecipe_InstanceBlockDeviceMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ImageRecipe.InstanceBlockDeviceMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ImageRecipe_InstanceBlockDeviceMapping as InstanceBlockDeviceMapping;
    impl crate::value::ToValue for InstanceBlockDeviceMapping_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-latestversion.html
    pub struct LatestVersion_ {
        pub arn: Option<crate::value::ExpString>,
        pub major: Option<crate::value::ExpString>,
        pub minor: Option<crate::value::ExpString>,
        pub patch: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ImageRecipe_LatestVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ImageRecipe.LatestVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ImageRecipe_LatestVersion as LatestVersion;
    impl crate::value::ToValue for LatestVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.major {
                properties.insert("Major".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.minor {
                properties.insert("Minor".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.patch {
                properties.insert("Patch".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-imagerecipe-systemsmanageragent.html
    pub struct SystemsManagerAgent_ {
        pub uninstall_after_build: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_ImageRecipe_SystemsManagerAgent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::ImageRecipe.SystemsManagerAgent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_ImageRecipe_SystemsManagerAgent as SystemsManagerAgent;
    impl crate::value::ToValue for SystemsManagerAgent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.uninstall_after_build {
                properties.insert(
                    "UninstallAfterBuild".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod infrastructureconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-infrastructureconfiguration-instancemetadataoptions.html
    pub struct InstanceMetadataOptions_ {
        pub http_put_response_hop_limit: Option<i32>,
        pub http_tokens: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_InfrastructureConfiguration_InstanceMetadataOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::InfrastructureConfiguration.InstanceMetadataOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_InfrastructureConfiguration_InstanceMetadataOptions as InstanceMetadataOptions;
    impl crate::value::ToValue for InstanceMetadataOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-infrastructureconfiguration-logging.html
    pub struct Logging_ {
        pub s3_logs: Option<Box<S3Logs_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_InfrastructureConfiguration_Logging {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::InfrastructureConfiguration.Logging"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_InfrastructureConfiguration_Logging as Logging;
    impl crate::value::ToValue for Logging_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_logs {
                properties.insert("S3Logs".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-infrastructureconfiguration-placement.html
    pub struct Placement_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub host_id: Option<crate::value::ExpString>,
        pub host_resource_group_arn: Option<crate::value::ExpString>,
        pub tenancy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_InfrastructureConfiguration_Placement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::InfrastructureConfiguration.Placement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_InfrastructureConfiguration_Placement as Placement;
    impl crate::value::ToValue for Placement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.availability_zone {
                properties.insert(
                    "AvailabilityZone".to_string(),
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
            if let Some(ref value) = self.tenancy {
                properties.insert(
                    "Tenancy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-infrastructureconfiguration-s3logs.html
    pub struct S3Logs_ {
        pub s3_bucket_name: Option<crate::value::ExpString>,
        pub s3_key_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_InfrastructureConfiguration_S3Logs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::InfrastructureConfiguration.S3Logs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_InfrastructureConfiguration_S3Logs as S3Logs;
    impl crate::value::ToValue for S3Logs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_bucket_name {
                properties.insert(
                    "S3BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_key_prefix {
                properties.insert(
                    "S3KeyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod lifecyclepolicy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-lifecyclepolicy-action.html
    pub struct Action_ {
        pub include_resources: Option<Box<IncludeResources_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_LifecyclePolicy_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::LifecyclePolicy.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_LifecyclePolicy_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.include_resources {
                properties.insert(
                    "IncludeResources".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-lifecyclepolicy-amiexclusionrules.html
    pub struct AmiExclusionRules_ {
        pub is_public: Option<crate::value::ExpBool>,
        pub last_launched: Option<Box<LastLaunched_>>,
        pub regions: Option<Vec<crate::value::ExpString>>,
        pub shared_accounts: Option<Vec<crate::value::ExpString>>,
        pub tag_map: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_LifecyclePolicy_AmiExclusionRules {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::LifecyclePolicy.AmiExclusionRules"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_LifecyclePolicy_AmiExclusionRules as AmiExclusionRules;
    impl crate::value::ToValue for AmiExclusionRules_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.is_public {
                properties.insert(
                    "IsPublic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.last_launched {
                properties.insert(
                    "LastLaunched".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.regions {
                properties.insert(
                    "Regions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shared_accounts {
                properties.insert(
                    "SharedAccounts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_map {
                properties.insert("TagMap".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-lifecyclepolicy-exclusionrules.html
    pub struct ExclusionRules_ {
        pub amis: Option<Box<AmiExclusionRules_>>,
        pub tag_map: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_LifecyclePolicy_ExclusionRules {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::LifecyclePolicy.ExclusionRules"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_LifecyclePolicy_ExclusionRules as ExclusionRules;
    impl crate::value::ToValue for ExclusionRules_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.amis {
                properties.insert("Amis".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tag_map {
                properties.insert("TagMap".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-lifecyclepolicy-filter.html
    pub struct Filter_ {
        pub retain_at_least: Option<i32>,
        pub r#type: crate::value::ExpString,
        pub unit: Option<crate::value::ExpString>,
        pub value: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_LifecyclePolicy_Filter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::LifecyclePolicy.Filter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_LifecyclePolicy_Filter as Filter;
    impl crate::value::ToValue for Filter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.retain_at_least {
                properties.insert(
                    "RetainAtLeast".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-lifecyclepolicy-includeresources.html
    pub struct IncludeResources_ {
        pub amis: Option<crate::value::ExpBool>,
        pub containers: Option<crate::value::ExpBool>,
        pub snapshots: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_LifecyclePolicy_IncludeResources {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::LifecyclePolicy.IncludeResources"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_LifecyclePolicy_IncludeResources as IncludeResources;
    impl crate::value::ToValue for IncludeResources_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.amis {
                properties.insert("Amis".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.containers {
                properties.insert(
                    "Containers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.snapshots {
                properties.insert(
                    "Snapshots".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-lifecyclepolicy-lastlaunched.html
    pub struct LastLaunched_ {
        pub unit: crate::value::ExpString,
        pub value: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_LifecyclePolicy_LastLaunched {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::LifecyclePolicy.LastLaunched"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_LifecyclePolicy_LastLaunched as LastLaunched;
    impl crate::value::ToValue for LastLaunched_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-lifecyclepolicy-policydetail.html
    pub struct PolicyDetail_ {
        pub action: Box<Action_>,
        pub exclusion_rules: Option<Box<ExclusionRules_>>,
        pub filter: Box<Filter_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_LifecyclePolicy_PolicyDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::LifecyclePolicy.PolicyDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_LifecyclePolicy_PolicyDetail as PolicyDetail;
    impl crate::value::ToValue for PolicyDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            if let Some(ref value) = self.exclusion_rules {
                properties.insert(
                    "ExclusionRules".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Filter".to_string(),
                crate::value::ToValue::to_value(&self.filter),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-lifecyclepolicy-recipeselection.html
    pub struct RecipeSelection_ {
        pub name: crate::value::ExpString,
        pub semantic_version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_LifecyclePolicy_RecipeSelection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::LifecyclePolicy.RecipeSelection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_LifecyclePolicy_RecipeSelection as RecipeSelection;
    impl crate::value::ToValue for RecipeSelection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "SemanticVersion".to_string(),
                crate::value::ToValue::to_value(&self.semantic_version),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-lifecyclepolicy-resourceselection.html
    pub struct ResourceSelection_ {
        pub recipes: Option<Vec<RecipeSelection_>>,
        pub tag_map: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_LifecyclePolicy_ResourceSelection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::LifecyclePolicy.ResourceSelection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_LifecyclePolicy_ResourceSelection as ResourceSelection;
    impl crate::value::ToValue for ResourceSelection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.recipes {
                properties.insert(
                    "Recipes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_map {
                properties.insert("TagMap".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod workflow {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-imagebuilder-workflow-latestversion.html
    pub struct LatestVersion_ {
        pub arn: Option<crate::value::ExpString>,
        pub major: Option<crate::value::ExpString>,
        pub minor: Option<crate::value::ExpString>,
        pub patch: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_imagebuilder_Workflow_LatestVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ImageBuilder::Workflow.LatestVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_imagebuilder_Workflow_LatestVersion as LatestVersion;
    impl crate::value::ToValue for LatestVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.major {
                properties.insert("Major".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.minor {
                properties.insert("Minor".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.patch {
                properties.insert("Patch".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-component.html
pub struct Component_ {
    pub change_description: Option<crate::value::ExpString>,
    pub data: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub platform: crate::value::ExpString,
    pub supported_os_versions: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub uri: Option<crate::value::ExpString>,
    pub version: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_imagebuilder_Component {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ImageBuilder::Component"
        $($field $value)*)
    };
}
pub use crate::__aws_imagebuilder_Component as Component;
impl crate::template::ToResource for Component_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ImageBuilder"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Component"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.change_description {
            properties.insert(
                "ChangeDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data {
            properties.insert("Data".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
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
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Platform".to_string(),
            crate::value::ToValue::to_value(&self.platform),
        );
        if let Some(ref value) = self.supported_os_versions {
            properties.insert(
                "SupportedOsVersions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.uri {
            properties.insert("Uri".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Version".to_string(),
            crate::value::ToValue::to_value(&self.version),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-containerrecipe.html
pub struct ContainerRecipe_ {
    pub components: Option<Vec<super::imagebuilder::containerrecipe::ComponentConfiguration_>>,
    pub container_type: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub dockerfile_template_data: Option<crate::value::ExpString>,
    pub dockerfile_template_uri: Option<crate::value::ExpString>,
    pub image_os_version_override: Option<crate::value::ExpString>,
    pub instance_configuration:
        Option<super::imagebuilder::containerrecipe::InstanceConfiguration_>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub parent_image: crate::value::ExpString,
    pub platform_override: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub target_repository: super::imagebuilder::containerrecipe::TargetContainerRepository_,
    pub version: crate::value::ExpString,
    pub working_directory: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_imagebuilder_ContainerRecipe {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ImageBuilder::ContainerRecipe"
        $($field $value)*)
    };
}
pub use crate::__aws_imagebuilder_ContainerRecipe as ContainerRecipe;
impl crate::template::ToResource for ContainerRecipe_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ImageBuilder"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ContainerRecipe"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.components {
            properties.insert(
                "Components".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ContainerType".to_string(),
            crate::value::ToValue::to_value(&self.container_type),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dockerfile_template_data {
            properties.insert(
                "DockerfileTemplateData".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dockerfile_template_uri {
            properties.insert(
                "DockerfileTemplateUri".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_os_version_override {
            properties.insert(
                "ImageOsVersionOverride".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_configuration {
            properties.insert(
                "InstanceConfiguration".to_string(),
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
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "ParentImage".to_string(),
            crate::value::ToValue::to_value(&self.parent_image),
        );
        if let Some(ref value) = self.platform_override {
            properties.insert(
                "PlatformOverride".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TargetRepository".to_string(),
            crate::value::ToValue::to_value(&self.target_repository),
        );
        properties.insert(
            "Version".to_string(),
            crate::value::ToValue::to_value(&self.version),
        );
        if let Some(ref value) = self.working_directory {
            properties.insert(
                "WorkingDirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-distributionconfiguration.html
pub struct DistributionConfiguration_ {
    pub description: Option<crate::value::ExpString>,
    pub distributions: Vec<super::imagebuilder::distributionconfiguration::Distribution_>,
    pub name: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_imagebuilder_DistributionConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ImageBuilder::DistributionConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_imagebuilder_DistributionConfiguration as DistributionConfiguration;
impl crate::template::ToResource for DistributionConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ImageBuilder"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DistributionConfiguration"),
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
            "Distributions".to_string(),
            crate::value::ToValue::to_value(&self.distributions),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-image.html
pub struct Image_ {
    pub container_recipe_arn: Option<crate::value::ExpString>,
    pub deletion_settings: Option<super::imagebuilder::image::DeletionSettings_>,
    pub distribution_configuration_arn: Option<crate::value::ExpString>,
    pub enhanced_image_metadata_enabled: Option<crate::value::ExpBool>,
    pub execution_role: Option<crate::value::ExpString>,
    pub image_pipeline_execution_settings:
        Option<super::imagebuilder::image::ImagePipelineExecutionSettings_>,
    pub image_recipe_arn: Option<crate::value::ExpString>,
    pub image_scanning_configuration:
        Option<super::imagebuilder::image::ImageScanningConfiguration_>,
    pub image_tests_configuration: Option<super::imagebuilder::image::ImageTestsConfiguration_>,
    pub infrastructure_configuration_arn: Option<crate::value::ExpString>,
    pub logging_configuration: Option<super::imagebuilder::image::ImageLoggingConfiguration_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub workflows: Option<Vec<super::imagebuilder::image::WorkflowConfiguration_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_imagebuilder_Image {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ImageBuilder::Image"
        $($field $value)*)
    };
}
pub use crate::__aws_imagebuilder_Image as Image;
impl crate::template::ToResource for Image_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ImageBuilder"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Image"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.container_recipe_arn {
            properties.insert(
                "ContainerRecipeArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deletion_settings {
            properties.insert(
                "DeletionSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.distribution_configuration_arn {
            properties.insert(
                "DistributionConfigurationArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enhanced_image_metadata_enabled {
            properties.insert(
                "EnhancedImageMetadataEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.execution_role {
            properties.insert(
                "ExecutionRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_pipeline_execution_settings {
            properties.insert(
                "ImagePipelineExecutionSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_recipe_arn {
            properties.insert(
                "ImageRecipeArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_scanning_configuration {
            properties.insert(
                "ImageScanningConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_tests_configuration {
            properties.insert(
                "ImageTestsConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.infrastructure_configuration_arn {
            properties.insert(
                "InfrastructureConfigurationArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logging_configuration {
            properties.insert(
                "LoggingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.workflows {
            properties.insert(
                "Workflows".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagepipeline.html
pub struct ImagePipeline_ {
    pub container_recipe_arn: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub distribution_configuration_arn: Option<crate::value::ExpString>,
    pub enhanced_image_metadata_enabled: Option<crate::value::ExpBool>,
    pub execution_role: Option<crate::value::ExpString>,
    pub image_recipe_arn: Option<crate::value::ExpString>,
    pub image_scanning_configuration:
        Option<super::imagebuilder::imagepipeline::ImageScanningConfiguration_>,
    pub image_tests_configuration:
        Option<super::imagebuilder::imagepipeline::ImageTestsConfiguration_>,
    pub infrastructure_configuration_arn: crate::value::ExpString,
    pub logging_configuration:
        Option<super::imagebuilder::imagepipeline::PipelineLoggingConfiguration_>,
    pub name: crate::value::ExpString,
    pub schedule: Option<super::imagebuilder::imagepipeline::Schedule_>,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub workflows: Option<Vec<super::imagebuilder::imagepipeline::WorkflowConfiguration_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_imagebuilder_ImagePipeline {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ImageBuilder::ImagePipeline"
        $($field $value)*)
    };
}
pub use crate::__aws_imagebuilder_ImagePipeline as ImagePipeline;
impl crate::template::ToResource for ImagePipeline_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ImageBuilder"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ImagePipeline"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.container_recipe_arn {
            properties.insert(
                "ContainerRecipeArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.distribution_configuration_arn {
            properties.insert(
                "DistributionConfigurationArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enhanced_image_metadata_enabled {
            properties.insert(
                "EnhancedImageMetadataEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.execution_role {
            properties.insert(
                "ExecutionRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_recipe_arn {
            properties.insert(
                "ImageRecipeArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_scanning_configuration {
            properties.insert(
                "ImageScanningConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_tests_configuration {
            properties.insert(
                "ImageTestsConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InfrastructureConfigurationArn".to_string(),
            crate::value::ToValue::to_value(&self.infrastructure_configuration_arn),
        );
        if let Some(ref value) = self.logging_configuration {
            properties.insert(
                "LoggingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.schedule {
            properties.insert(
                "Schedule".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.workflows {
            properties.insert(
                "Workflows".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-imagerecipe.html
pub struct ImageRecipe_ {
    pub additional_instance_configuration:
        Option<super::imagebuilder::imagerecipe::AdditionalInstanceConfiguration_>,
    pub ami_tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub block_device_mappings:
        Option<Vec<super::imagebuilder::imagerecipe::InstanceBlockDeviceMapping_>>,
    pub components: Option<Vec<super::imagebuilder::imagerecipe::ComponentConfiguration_>>,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub parent_image: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub version: crate::value::ExpString,
    pub working_directory: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_imagebuilder_ImageRecipe {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ImageBuilder::ImageRecipe"
        $($field $value)*)
    };
}
pub use crate::__aws_imagebuilder_ImageRecipe as ImageRecipe;
impl crate::template::ToResource for ImageRecipe_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ImageBuilder"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ImageRecipe"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_instance_configuration {
            properties.insert(
                "AdditionalInstanceConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ami_tags {
            properties.insert(
                "AmiTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.block_device_mappings {
            properties.insert(
                "BlockDeviceMappings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.components {
            properties.insert(
                "Components".to_string(),
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
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "ParentImage".to_string(),
            crate::value::ToValue::to_value(&self.parent_image),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Version".to_string(),
            crate::value::ToValue::to_value(&self.version),
        );
        if let Some(ref value) = self.working_directory {
            properties.insert(
                "WorkingDirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-infrastructureconfiguration.html
pub struct InfrastructureConfiguration_ {
    pub description: Option<crate::value::ExpString>,
    pub instance_metadata_options:
        Option<super::imagebuilder::infrastructureconfiguration::InstanceMetadataOptions_>,
    pub instance_profile_name: crate::value::ExpString,
    pub instance_types: Option<Vec<crate::value::ExpString>>,
    pub key_pair: Option<crate::value::ExpString>,
    pub logging: Option<super::imagebuilder::infrastructureconfiguration::Logging_>,
    pub name: crate::value::ExpString,
    pub placement: Option<super::imagebuilder::infrastructureconfiguration::Placement_>,
    pub resource_tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub sns_topic_arn: Option<crate::value::ExpString>,
    pub subnet_id: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub terminate_instance_on_failure: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_imagebuilder_InfrastructureConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ImageBuilder::InfrastructureConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_imagebuilder_InfrastructureConfiguration as InfrastructureConfiguration;
impl crate::template::ToResource for InfrastructureConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ImageBuilder"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "InfrastructureConfiguration",
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
        if let Some(ref value) = self.instance_metadata_options {
            properties.insert(
                "InstanceMetadataOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceProfileName".to_string(),
            crate::value::ToValue::to_value(&self.instance_profile_name),
        );
        if let Some(ref value) = self.instance_types {
            properties.insert(
                "InstanceTypes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.key_pair {
            properties.insert(
                "KeyPair".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logging {
            properties.insert(
                "Logging".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.placement {
            properties.insert(
                "Placement".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_tags {
            properties.insert(
                "ResourceTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sns_topic_arn {
            properties.insert(
                "SnsTopicArn".to_string(),
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
        if let Some(ref value) = self.terminate_instance_on_failure {
            properties.insert(
                "TerminateInstanceOnFailure".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-lifecyclepolicy.html
pub struct LifecyclePolicy_ {
    pub description: Option<crate::value::ExpString>,
    pub execution_role: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub policy_details: Vec<super::imagebuilder::lifecyclepolicy::PolicyDetail_>,
    pub resource_selection: super::imagebuilder::lifecyclepolicy::ResourceSelection_,
    pub resource_type: crate::value::ExpString,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_imagebuilder_LifecyclePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ImageBuilder::LifecyclePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_imagebuilder_LifecyclePolicy as LifecyclePolicy;
impl crate::template::ToResource for LifecyclePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ImageBuilder"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LifecyclePolicy"),
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
            "ExecutionRole".to_string(),
            crate::value::ToValue::to_value(&self.execution_role),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "PolicyDetails".to_string(),
            crate::value::ToValue::to_value(&self.policy_details),
        );
        properties.insert(
            "ResourceSelection".to_string(),
            crate::value::ToValue::to_value(&self.resource_selection),
        );
        properties.insert(
            "ResourceType".to_string(),
            crate::value::ToValue::to_value(&self.resource_type),
        );
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-imagebuilder-workflow.html
pub struct Workflow_ {
    pub change_description: Option<crate::value::ExpString>,
    pub data: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub r#type: crate::value::ExpString,
    pub uri: Option<crate::value::ExpString>,
    pub version: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_imagebuilder_Workflow {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ImageBuilder::Workflow"
        $($field $value)*)
    };
}
pub use crate::__aws_imagebuilder_Workflow as Workflow;
impl crate::template::ToResource for Workflow_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ImageBuilder"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Workflow"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.change_description {
            properties.insert(
                "ChangeDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data {
            properties.insert("Data".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
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
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        if let Some(ref value) = self.uri {
            properties.insert("Uri".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Version".to_string(),
            crate::value::ToValue::to_value(&self.version),
        );
        properties
    }
}
