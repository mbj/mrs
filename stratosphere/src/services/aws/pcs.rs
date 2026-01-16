pub mod cluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-cluster-accounting.html
    pub struct Accounting_ {
        pub default_purge_time_in_days: Option<i64>,
        pub mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_Cluster_Accounting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::Cluster.Accounting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_Cluster_Accounting as Accounting;
    impl crate::value::ToValue for Accounting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_purge_time_in_days {
                properties.insert(
                    "DefaultPurgeTimeInDays".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-cluster-authkey.html
    pub struct AuthKey_ {
        pub secret_arn: crate::value::ExpString,
        pub secret_version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_Cluster_AuthKey {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::Cluster.AuthKey"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_Cluster_AuthKey as AuthKey;
    impl crate::value::ToValue for AuthKey_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.insert(
                "SecretVersion".to_string(),
                crate::value::ToValue::to_value(&self.secret_version),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-cluster-endpoint.html
    pub struct Endpoint_ {
        pub ipv6_address: Option<crate::value::ExpString>,
        pub port: crate::value::ExpString,
        pub private_ip_address: crate::value::ExpString,
        pub public_ip_address: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_Cluster_Endpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::Cluster.Endpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_Cluster_Endpoint as Endpoint;
    impl crate::value::ToValue for Endpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ipv6_address {
                properties.insert(
                    "Ipv6Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.insert(
                "PrivateIpAddress".to_string(),
                crate::value::ToValue::to_value(&self.private_ip_address),
            );
            if let Some(ref value) = self.public_ip_address {
                properties.insert(
                    "PublicIpAddress".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-cluster-errorinfo.html
    pub struct ErrorInfo_ {
        pub code: Option<crate::value::ExpString>,
        pub message: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_Cluster_ErrorInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::Cluster.ErrorInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_Cluster_ErrorInfo as ErrorInfo;
    impl crate::value::ToValue for ErrorInfo_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-cluster-networking.html
    pub struct Networking_ {
        pub network_type: Option<crate::value::ExpString>,
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_Cluster_Networking {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::Cluster.Networking"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_Cluster_Networking as Networking;
    impl crate::value::ToValue for Networking_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.network_type {
                properties.insert(
                    "NetworkType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-cluster-scheduler.html
    pub struct Scheduler_ {
        pub r#type: crate::value::ExpString,
        pub version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_Cluster_Scheduler {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::Cluster.Scheduler"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_Cluster_Scheduler as Scheduler;
    impl crate::value::ToValue for Scheduler_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.insert(
                "Version".to_string(),
                crate::value::ToValue::to_value(&self.version),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-cluster-slurmconfiguration.html
    pub struct SlurmConfiguration_ {
        pub accounting: Option<Box<Accounting_>>,
        pub auth_key: Option<Box<AuthKey_>>,
        pub scale_down_idle_time_in_seconds: Option<i64>,
        pub slurm_custom_settings: Option<Vec<SlurmCustomSetting_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_Cluster_SlurmConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::Cluster.SlurmConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_Cluster_SlurmConfiguration as SlurmConfiguration;
    impl crate::value::ToValue for SlurmConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.accounting {
                properties.insert(
                    "Accounting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.auth_key {
                properties.insert(
                    "AuthKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scale_down_idle_time_in_seconds {
                properties.insert(
                    "ScaleDownIdleTimeInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slurm_custom_settings {
                properties.insert(
                    "SlurmCustomSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-cluster-slurmcustomsetting.html
    pub struct SlurmCustomSetting_ {
        pub parameter_name: crate::value::ExpString,
        pub parameter_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_Cluster_SlurmCustomSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::Cluster.SlurmCustomSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_Cluster_SlurmCustomSetting as SlurmCustomSetting;
    impl crate::value::ToValue for SlurmCustomSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ParameterName".to_string(),
                crate::value::ToValue::to_value(&self.parameter_name),
            );
            properties.insert(
                "ParameterValue".to_string(),
                crate::value::ToValue::to_value(&self.parameter_value),
            );
            properties.into()
        }
    }
}
pub mod computenodegroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-computenodegroup-customlaunchtemplate.html
    pub struct CustomLaunchTemplate_ {
        pub template_id: Option<crate::value::ExpString>,
        pub version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_ComputeNodeGroup_CustomLaunchTemplate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::ComputeNodeGroup.CustomLaunchTemplate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_ComputeNodeGroup_CustomLaunchTemplate as CustomLaunchTemplate;
    impl crate::value::ToValue for CustomLaunchTemplate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.template_id {
                properties.insert(
                    "TemplateId".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-computenodegroup-errorinfo.html
    pub struct ErrorInfo_ {
        pub code: Option<crate::value::ExpString>,
        pub message: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_ComputeNodeGroup_ErrorInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::ComputeNodeGroup.ErrorInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_ComputeNodeGroup_ErrorInfo as ErrorInfo;
    impl crate::value::ToValue for ErrorInfo_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-computenodegroup-instanceconfig.html
    pub struct InstanceConfig_ {
        pub instance_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_ComputeNodeGroup_InstanceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::ComputeNodeGroup.InstanceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_ComputeNodeGroup_InstanceConfig as InstanceConfig;
    impl crate::value::ToValue for InstanceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instance_type {
                properties.insert(
                    "InstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-computenodegroup-scalingconfiguration.html
    pub struct ScalingConfiguration_ {
        pub max_instance_count: i64,
        pub min_instance_count: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_ComputeNodeGroup_ScalingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::ComputeNodeGroup.ScalingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_ComputeNodeGroup_ScalingConfiguration as ScalingConfiguration;
    impl crate::value::ToValue for ScalingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxInstanceCount".to_string(),
                crate::value::ToValue::to_value(&self.max_instance_count),
            );
            properties.insert(
                "MinInstanceCount".to_string(),
                crate::value::ToValue::to_value(&self.min_instance_count),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-computenodegroup-slurmconfiguration.html
    pub struct SlurmConfiguration_ {
        pub slurm_custom_settings: Option<Vec<SlurmCustomSetting_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_ComputeNodeGroup_SlurmConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::ComputeNodeGroup.SlurmConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_ComputeNodeGroup_SlurmConfiguration as SlurmConfiguration;
    impl crate::value::ToValue for SlurmConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.slurm_custom_settings {
                properties.insert(
                    "SlurmCustomSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-computenodegroup-slurmcustomsetting.html
    pub struct SlurmCustomSetting_ {
        pub parameter_name: crate::value::ExpString,
        pub parameter_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_ComputeNodeGroup_SlurmCustomSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::ComputeNodeGroup.SlurmCustomSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_ComputeNodeGroup_SlurmCustomSetting as SlurmCustomSetting;
    impl crate::value::ToValue for SlurmCustomSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ParameterName".to_string(),
                crate::value::ToValue::to_value(&self.parameter_name),
            );
            properties.insert(
                "ParameterValue".to_string(),
                crate::value::ToValue::to_value(&self.parameter_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-computenodegroup-spotoptions.html
    pub struct SpotOptions_ {
        pub allocation_strategy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_ComputeNodeGroup_SpotOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::ComputeNodeGroup.SpotOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_ComputeNodeGroup_SpotOptions as SpotOptions;
    impl crate::value::ToValue for SpotOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allocation_strategy {
                properties.insert(
                    "AllocationStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod queue {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-queue-computenodegroupconfiguration.html
    pub struct ComputeNodeGroupConfiguration_ {
        pub compute_node_group_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_Queue_ComputeNodeGroupConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::Queue.ComputeNodeGroupConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_Queue_ComputeNodeGroupConfiguration as ComputeNodeGroupConfiguration;
    impl crate::value::ToValue for ComputeNodeGroupConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.compute_node_group_id {
                properties.insert(
                    "ComputeNodeGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcs-queue-errorinfo.html
    pub struct ErrorInfo_ {
        pub code: Option<crate::value::ExpString>,
        pub message: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcs_Queue_ErrorInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCS::Queue.ErrorInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcs_Queue_ErrorInfo as ErrorInfo;
    impl crate::value::ToValue for ErrorInfo_ {
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcs-cluster.html
pub struct Cluster_ {
    pub name: Option<crate::value::ExpString>,
    pub networking: super::pcs::cluster::Networking_,
    pub scheduler: super::pcs::cluster::Scheduler_,
    pub size: crate::value::ExpString,
    pub slurm_configuration: Option<super::pcs::cluster::SlurmConfiguration_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pcs_Cluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::PCS::Cluster" $($field
        $value)*)
    };
}
pub use crate::__aws_pcs_Cluster as Cluster;
impl crate::template::ToResource for Cluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("PCS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Cluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Networking".to_string(),
            crate::value::ToValue::to_value(&self.networking),
        );
        properties.insert(
            "Scheduler".to_string(),
            crate::value::ToValue::to_value(&self.scheduler),
        );
        properties.insert(
            "Size".to_string(),
            crate::value::ToValue::to_value(&self.size),
        );
        if let Some(ref value) = self.slurm_configuration {
            properties.insert(
                "SlurmConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcs-computenodegroup.html
pub struct ComputeNodeGroup_ {
    pub ami_id: Option<crate::value::ExpString>,
    pub cluster_id: crate::value::ExpString,
    pub custom_launch_template: super::pcs::computenodegroup::CustomLaunchTemplate_,
    pub iam_instance_profile_arn: crate::value::ExpString,
    pub instance_configs: Vec<super::pcs::computenodegroup::InstanceConfig_>,
    pub name: Option<crate::value::ExpString>,
    pub purchase_option: Option<crate::value::ExpString>,
    pub scaling_configuration: super::pcs::computenodegroup::ScalingConfiguration_,
    pub slurm_configuration: Option<super::pcs::computenodegroup::SlurmConfiguration_>,
    pub spot_options: Option<super::pcs::computenodegroup::SpotOptions_>,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pcs_ComputeNodeGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::PCS::ComputeNodeGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_pcs_ComputeNodeGroup as ComputeNodeGroup;
impl crate::template::ToResource for ComputeNodeGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("PCS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ComputeNodeGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.ami_id {
            properties.insert("AmiId".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "ClusterId".to_string(),
            crate::value::ToValue::to_value(&self.cluster_id),
        );
        properties.insert(
            "CustomLaunchTemplate".to_string(),
            crate::value::ToValue::to_value(&self.custom_launch_template),
        );
        properties.insert(
            "IamInstanceProfileArn".to_string(),
            crate::value::ToValue::to_value(&self.iam_instance_profile_arn),
        );
        properties.insert(
            "InstanceConfigs".to_string(),
            crate::value::ToValue::to_value(&self.instance_configs),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.purchase_option {
            properties.insert(
                "PurchaseOption".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ScalingConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.scaling_configuration),
        );
        if let Some(ref value) = self.slurm_configuration {
            properties.insert(
                "SlurmConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.spot_options {
            properties.insert(
                "SpotOptions".to_string(),
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
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcs-queue.html
pub struct Queue_ {
    pub cluster_id: crate::value::ExpString,
    pub compute_node_group_configurations:
        Option<Vec<super::pcs::queue::ComputeNodeGroupConfiguration_>>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pcs_Queue {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::PCS::Queue" $($field
        $value)*)
    };
}
pub use crate::__aws_pcs_Queue as Queue;
impl crate::template::ToResource for Queue_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("PCS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Queue"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ClusterId".to_string(),
            crate::value::ToValue::to_value(&self.cluster_id),
        );
        if let Some(ref value) = self.compute_node_group_configurations {
            properties.insert(
                "ComputeNodeGroupConfigurations".to_string(),
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
