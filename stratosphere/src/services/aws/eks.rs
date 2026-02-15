pub mod accessentry {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-accessentry-accesspolicy.html
    pub struct AccessPolicy_ {
        pub access_scope: Box<AccessScope_>,
        pub policy_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_AccessEntry_AccessPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::AccessEntry.AccessPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_AccessEntry_AccessPolicy as AccessPolicy;
    impl crate::value::ToValue for AccessPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccessScope".to_string(),
                crate::value::ToValue::to_value(&self.access_scope),
            );
            properties.insert(
                "PolicyArn".to_string(),
                crate::value::ToValue::to_value(&self.policy_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-accessentry-accessscope.html
    pub struct AccessScope_ {
        pub namespaces: Option<Vec<crate::value::ExpString>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_AccessEntry_AccessScope {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::AccessEntry.AccessScope"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_AccessEntry_AccessScope as AccessScope;
    impl crate::value::ToValue for AccessScope_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.namespaces {
                properties.insert(
                    "Namespaces".to_string(),
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
}
pub mod addon {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-addon-namespaceconfig.html
    pub struct NamespaceConfig_ {
        pub namespace: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Addon_NamespaceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Addon.NamespaceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Addon_NamespaceConfig as NamespaceConfig;
    impl crate::value::ToValue for NamespaceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Namespace".to_string(),
                crate::value::ToValue::to_value(&self.namespace),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-addon-podidentityassociation.html
    pub struct PodIdentityAssociation_ {
        pub role_arn: crate::value::ExpString,
        pub service_account: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Addon_PodIdentityAssociation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Addon.PodIdentityAssociation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Addon_PodIdentityAssociation as PodIdentityAssociation;
    impl crate::value::ToValue for PodIdentityAssociation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "ServiceAccount".to_string(),
                crate::value::ToValue::to_value(&self.service_account),
            );
            properties.into()
        }
    }
}
pub mod capability {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-capability-argocd.html
    pub struct ArgoCd_ {
        pub aws_idc: Box<AwsIdc_>,
        pub namespace: Option<crate::value::ExpString>,
        pub network_access: Option<Box<NetworkAccess_>>,
        pub rbac_role_mappings: Option<Vec<ArgoCdRoleMapping_>>,
        pub server_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Capability_ArgoCd {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Capability.ArgoCd"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Capability_ArgoCd as ArgoCd;
    impl crate::value::ToValue for ArgoCd_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AwsIdc".to_string(),
                crate::value::ToValue::to_value(&self.aws_idc),
            );
            if let Some(ref value) = self.namespace {
                properties.insert(
                    "Namespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_access {
                properties.insert(
                    "NetworkAccess".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rbac_role_mappings {
                properties.insert(
                    "RbacRoleMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.server_url {
                properties.insert(
                    "ServerUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-capability-argocdrolemapping.html
    pub struct ArgoCdRoleMapping_ {
        pub identities: Vec<SsoIdentity_>,
        pub role: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Capability_ArgoCdRoleMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Capability.ArgoCdRoleMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Capability_ArgoCdRoleMapping as ArgoCdRoleMapping;
    impl crate::value::ToValue for ArgoCdRoleMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Identities".to_string(),
                crate::value::ToValue::to_value(&self.identities),
            );
            properties.insert(
                "Role".to_string(),
                crate::value::ToValue::to_value(&self.role),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-capability-awsidc.html
    pub struct AwsIdc_ {
        pub idc_instance_arn: crate::value::ExpString,
        pub idc_managed_application_arn: Option<crate::value::ExpString>,
        pub idc_region: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Capability_AwsIdc {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Capability.AwsIdc"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Capability_AwsIdc as AwsIdc;
    impl crate::value::ToValue for AwsIdc_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IdcInstanceArn".to_string(),
                crate::value::ToValue::to_value(&self.idc_instance_arn),
            );
            if let Some(ref value) = self.idc_managed_application_arn {
                properties.insert(
                    "IdcManagedApplicationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.idc_region {
                properties.insert(
                    "IdcRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-capability-capabilityconfiguration.html
    pub struct CapabilityConfiguration_ {
        pub argo_cd: Option<Box<ArgoCd_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Capability_CapabilityConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Capability.CapabilityConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Capability_CapabilityConfiguration as CapabilityConfiguration;
    impl crate::value::ToValue for CapabilityConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.argo_cd {
                properties.insert("ArgoCd".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-capability-networkaccess.html
    pub struct NetworkAccess_ {
        pub vpce_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Capability_NetworkAccess {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Capability.NetworkAccess"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Capability_NetworkAccess as NetworkAccess;
    impl crate::value::ToValue for NetworkAccess_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.vpce_ids {
                properties.insert(
                    "VpceIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-capability-ssoidentity.html
    pub struct SsoIdentity_ {
        pub id: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Capability_SsoIdentity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Capability.SsoIdentity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Capability_SsoIdentity as SsoIdentity;
    impl crate::value::ToValue for SsoIdentity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
pub mod cluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-accessconfig.html
    pub struct AccessConfig_ {
        pub authentication_mode: Option<crate::value::ExpString>,
        pub bootstrap_cluster_creator_admin_permissions: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_AccessConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.AccessConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_AccessConfig as AccessConfig;
    impl crate::value::ToValue for AccessConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authentication_mode {
                properties.insert(
                    "AuthenticationMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bootstrap_cluster_creator_admin_permissions {
                properties.insert(
                    "BootstrapClusterCreatorAdminPermissions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-blockstorage.html
    pub struct BlockStorage_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_BlockStorage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.BlockStorage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_BlockStorage as BlockStorage;
    impl crate::value::ToValue for BlockStorage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-clusterlogging.html
    pub struct ClusterLogging_ {
        pub enabled_types: Option<Vec<LoggingTypeConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_ClusterLogging {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.ClusterLogging"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_ClusterLogging as ClusterLogging;
    impl crate::value::ToValue for ClusterLogging_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled_types {
                properties.insert(
                    "EnabledTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-computeconfig.html
    pub struct ComputeConfig_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub node_pools: Option<Vec<crate::value::ExpString>>,
        pub node_role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_ComputeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.ComputeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_ComputeConfig as ComputeConfig;
    impl crate::value::ToValue for ComputeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.node_pools {
                properties.insert(
                    "NodePools".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.node_role_arn {
                properties.insert(
                    "NodeRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-controlplaneplacement.html
    pub struct ControlPlanePlacement_ {
        pub group_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_ControlPlanePlacement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.ControlPlanePlacement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_ControlPlanePlacement as ControlPlanePlacement;
    impl crate::value::ToValue for ControlPlanePlacement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.group_name {
                properties.insert(
                    "GroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-controlplanescalingconfig.html
    pub struct ControlPlaneScalingConfig_ {
        pub tier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_ControlPlaneScalingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.ControlPlaneScalingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_ControlPlaneScalingConfig as ControlPlaneScalingConfig;
    impl crate::value::ToValue for ControlPlaneScalingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.tier {
                properties.insert("Tier".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-elasticloadbalancing.html
    pub struct ElasticLoadBalancing_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_ElasticLoadBalancing {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.ElasticLoadBalancing"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_ElasticLoadBalancing as ElasticLoadBalancing;
    impl crate::value::ToValue for ElasticLoadBalancing_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-encryptionconfig.html
    pub struct EncryptionConfig_ {
        pub provider: Option<Box<Provider_>>,
        pub resources: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_EncryptionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.EncryptionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_EncryptionConfig as EncryptionConfig;
    impl crate::value::ToValue for EncryptionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.provider {
                properties.insert(
                    "Provider".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resources {
                properties.insert(
                    "Resources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-kubernetesnetworkconfig.html
    pub struct KubernetesNetworkConfig_ {
        pub elastic_load_balancing: Option<Box<ElasticLoadBalancing_>>,
        pub ip_family: Option<crate::value::ExpString>,
        pub service_ipv4_cidr: Option<crate::value::ExpString>,
        pub service_ipv6_cidr: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_KubernetesNetworkConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.KubernetesNetworkConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_KubernetesNetworkConfig as KubernetesNetworkConfig;
    impl crate::value::ToValue for KubernetesNetworkConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.elastic_load_balancing {
                properties.insert(
                    "ElasticLoadBalancing".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ip_family {
                properties.insert(
                    "IpFamily".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_ipv4_cidr {
                properties.insert(
                    "ServiceIpv4Cidr".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_ipv6_cidr {
                properties.insert(
                    "ServiceIpv6Cidr".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-logging.html
    pub struct Logging_ {
        pub cluster_logging: Option<Box<ClusterLogging_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_Logging {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.Logging"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_Logging as Logging;
    impl crate::value::ToValue for Logging_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cluster_logging {
                properties.insert(
                    "ClusterLogging".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-loggingtypeconfig.html
    pub struct LoggingTypeConfig_ {
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_LoggingTypeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.LoggingTypeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_LoggingTypeConfig as LoggingTypeConfig;
    impl crate::value::ToValue for LoggingTypeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-outpostconfig.html
    pub struct OutpostConfig_ {
        pub control_plane_instance_type: crate::value::ExpString,
        pub control_plane_placement: Option<Box<ControlPlanePlacement_>>,
        pub outpost_arns: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_OutpostConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.OutpostConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_OutpostConfig as OutpostConfig;
    impl crate::value::ToValue for OutpostConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ControlPlaneInstanceType".to_string(),
                crate::value::ToValue::to_value(&self.control_plane_instance_type),
            );
            if let Some(ref value) = self.control_plane_placement {
                properties.insert(
                    "ControlPlanePlacement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OutpostArns".to_string(),
                crate::value::ToValue::to_value(&self.outpost_arns),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-provider.html
    pub struct Provider_ {
        pub key_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_Provider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.Provider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_Provider as Provider;
    impl crate::value::ToValue for Provider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key_arn {
                properties.insert("KeyArn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-remotenetworkconfig.html
    pub struct RemoteNetworkConfig_ {
        pub remote_node_networks: Vec<RemoteNodeNetwork_>,
        pub remote_pod_networks: Option<Vec<RemotePodNetwork_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_RemoteNetworkConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.RemoteNetworkConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_RemoteNetworkConfig as RemoteNetworkConfig;
    impl crate::value::ToValue for RemoteNetworkConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RemoteNodeNetworks".to_string(),
                crate::value::ToValue::to_value(&self.remote_node_networks),
            );
            if let Some(ref value) = self.remote_pod_networks {
                properties.insert(
                    "RemotePodNetworks".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-remotenodenetwork.html
    pub struct RemoteNodeNetwork_ {
        pub cidrs: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_RemoteNodeNetwork {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.RemoteNodeNetwork"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_RemoteNodeNetwork as RemoteNodeNetwork;
    impl crate::value::ToValue for RemoteNodeNetwork_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Cidrs".to_string(),
                crate::value::ToValue::to_value(&self.cidrs),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-remotepodnetwork.html
    pub struct RemotePodNetwork_ {
        pub cidrs: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_RemotePodNetwork {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.RemotePodNetwork"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_RemotePodNetwork as RemotePodNetwork;
    impl crate::value::ToValue for RemotePodNetwork_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Cidrs".to_string(),
                crate::value::ToValue::to_value(&self.cidrs),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-resourcesvpcconfig.html
    pub struct ResourcesVpcConfig_ {
        pub endpoint_private_access: Option<crate::value::ExpBool>,
        pub endpoint_public_access: Option<crate::value::ExpBool>,
        pub public_access_cidrs: Option<Vec<crate::value::ExpString>>,
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_ResourcesVpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.ResourcesVpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_ResourcesVpcConfig as ResourcesVpcConfig;
    impl crate::value::ToValue for ResourcesVpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.endpoint_private_access {
                properties.insert(
                    "EndpointPrivateAccess".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.endpoint_public_access {
                properties.insert(
                    "EndpointPublicAccess".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.public_access_cidrs {
                properties.insert(
                    "PublicAccessCidrs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(&self.subnet_ids),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-storageconfig.html
    pub struct StorageConfig_ {
        pub block_storage: Option<Box<BlockStorage_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_StorageConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.StorageConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_StorageConfig as StorageConfig;
    impl crate::value::ToValue for StorageConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.block_storage {
                properties.insert(
                    "BlockStorage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-upgradepolicy.html
    pub struct UpgradePolicy_ {
        pub support_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_UpgradePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.UpgradePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_UpgradePolicy as UpgradePolicy;
    impl crate::value::ToValue for UpgradePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.support_type {
                properties.insert(
                    "SupportType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-cluster-zonalshiftconfig.html
    pub struct ZonalShiftConfig_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Cluster_ZonalShiftConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Cluster.ZonalShiftConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Cluster_ZonalShiftConfig as ZonalShiftConfig;
    impl crate::value::ToValue for ZonalShiftConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod fargateprofile {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-fargateprofile-label.html
    pub struct Label_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_FargateProfile_Label {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::FargateProfile.Label"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_FargateProfile_Label as Label;
    impl crate::value::ToValue for Label_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-fargateprofile-selector.html
    pub struct Selector_ {
        pub labels: Option<Vec<Label_>>,
        pub namespace: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_FargateProfile_Selector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::FargateProfile.Selector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_FargateProfile_Selector as Selector;
    impl crate::value::ToValue for Selector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.labels {
                properties.insert("Labels".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Namespace".to_string(),
                crate::value::ToValue::to_value(&self.namespace),
            );
            properties.into()
        }
    }
}
pub mod identityproviderconfig {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-identityproviderconfig-oidcidentityproviderconfig.html
    pub struct OidcIdentityProviderConfig_ {
        pub client_id: crate::value::ExpString,
        pub groups_claim: Option<crate::value::ExpString>,
        pub groups_prefix: Option<crate::value::ExpString>,
        pub issuer_url: crate::value::ExpString,
        pub required_claims: Option<Vec<RequiredClaim_>>,
        pub username_claim: Option<crate::value::ExpString>,
        pub username_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_IdentityProviderConfig_OidcIdentityProviderConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::IdentityProviderConfig.OidcIdentityProviderConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_IdentityProviderConfig_OidcIdentityProviderConfig as OidcIdentityProviderConfig;
    impl crate::value::ToValue for OidcIdentityProviderConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClientId".to_string(),
                crate::value::ToValue::to_value(&self.client_id),
            );
            if let Some(ref value) = self.groups_claim {
                properties.insert(
                    "GroupsClaim".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.groups_prefix {
                properties.insert(
                    "GroupsPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IssuerUrl".to_string(),
                crate::value::ToValue::to_value(&self.issuer_url),
            );
            if let Some(ref value) = self.required_claims {
                properties.insert(
                    "RequiredClaims".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.username_claim {
                properties.insert(
                    "UsernameClaim".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.username_prefix {
                properties.insert(
                    "UsernamePrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-identityproviderconfig-requiredclaim.html
    pub struct RequiredClaim_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_IdentityProviderConfig_RequiredClaim {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::IdentityProviderConfig.RequiredClaim"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_IdentityProviderConfig_RequiredClaim as RequiredClaim;
    impl crate::value::ToValue for RequiredClaim_ {
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
}
pub mod nodegroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-launchtemplatespecification.html
    pub struct LaunchTemplateSpecification_ {
        pub id: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Nodegroup_LaunchTemplateSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Nodegroup.LaunchTemplateSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Nodegroup_LaunchTemplateSpecification as LaunchTemplateSpecification;
    impl crate::value::ToValue for LaunchTemplateSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-noderepairconfig.html
    pub struct NodeRepairConfig_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub max_parallel_nodes_repaired_count: Option<i32>,
        pub max_parallel_nodes_repaired_percentage: Option<i32>,
        pub max_unhealthy_node_threshold_count: Option<i32>,
        pub max_unhealthy_node_threshold_percentage: Option<i32>,
        pub node_repair_config_overrides: Option<Vec<NodeRepairConfigOverrides_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Nodegroup_NodeRepairConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Nodegroup.NodeRepairConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Nodegroup_NodeRepairConfig as NodeRepairConfig;
    impl crate::value::ToValue for NodeRepairConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_parallel_nodes_repaired_count {
                properties.insert(
                    "MaxParallelNodesRepairedCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_parallel_nodes_repaired_percentage {
                properties.insert(
                    "MaxParallelNodesRepairedPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_unhealthy_node_threshold_count {
                properties.insert(
                    "MaxUnhealthyNodeThresholdCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_unhealthy_node_threshold_percentage {
                properties.insert(
                    "MaxUnhealthyNodeThresholdPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.node_repair_config_overrides {
                properties.insert(
                    "NodeRepairConfigOverrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-noderepairconfigoverrides.html
    pub struct NodeRepairConfigOverrides_ {
        pub min_repair_wait_time_mins: Option<i32>,
        pub node_monitoring_condition: Option<crate::value::ExpString>,
        pub node_unhealthy_reason: Option<crate::value::ExpString>,
        pub repair_action: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Nodegroup_NodeRepairConfigOverrides {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Nodegroup.NodeRepairConfigOverrides"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Nodegroup_NodeRepairConfigOverrides as NodeRepairConfigOverrides;
    impl crate::value::ToValue for NodeRepairConfigOverrides_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.min_repair_wait_time_mins {
                properties.insert(
                    "MinRepairWaitTimeMins".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.node_monitoring_condition {
                properties.insert(
                    "NodeMonitoringCondition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.node_unhealthy_reason {
                properties.insert(
                    "NodeUnhealthyReason".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.repair_action {
                properties.insert(
                    "RepairAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-remoteaccess.html
    pub struct RemoteAccess_ {
        pub ec2_ssh_key: crate::value::ExpString,
        pub source_security_groups: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Nodegroup_RemoteAccess {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Nodegroup.RemoteAccess"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Nodegroup_RemoteAccess as RemoteAccess;
    impl crate::value::ToValue for RemoteAccess_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Ec2SshKey".to_string(),
                crate::value::ToValue::to_value(&self.ec2_ssh_key),
            );
            if let Some(ref value) = self.source_security_groups {
                properties.insert(
                    "SourceSecurityGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-scalingconfig.html
    pub struct ScalingConfig_ {
        pub desired_size: Option<i32>,
        pub max_size: Option<i32>,
        pub min_size: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Nodegroup_ScalingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Nodegroup.ScalingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Nodegroup_ScalingConfig as ScalingConfig;
    impl crate::value::ToValue for ScalingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.desired_size {
                properties.insert(
                    "DesiredSize".to_string(),
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
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-taint.html
    pub struct Taint_ {
        pub effect: Option<crate::value::ExpString>,
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Nodegroup_Taint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Nodegroup.Taint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Nodegroup_Taint as Taint;
    impl crate::value::ToValue for Taint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.effect {
                properties.insert("Effect".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eks-nodegroup-updateconfig.html
    pub struct UpdateConfig_ {
        pub max_unavailable: Option<f64>,
        pub max_unavailable_percentage: Option<f64>,
        pub update_strategy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eks_Nodegroup_UpdateConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EKS::Nodegroup.UpdateConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eks_Nodegroup_UpdateConfig as UpdateConfig;
    impl crate::value::ToValue for UpdateConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_unavailable {
                properties.insert(
                    "MaxUnavailable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_unavailable_percentage {
                properties.insert(
                    "MaxUnavailablePercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.update_strategy {
                properties.insert(
                    "UpdateStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-accessentry.html
pub struct AccessEntry_ {
    pub access_policies: Option<Vec<super::eks::accessentry::AccessPolicy_>>,
    pub cluster_name: crate::value::ExpString,
    pub kubernetes_groups: Option<Vec<crate::value::ExpString>>,
    pub principal_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: Option<crate::value::ExpString>,
    pub username: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_eks_AccessEntry {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EKS::AccessEntry"
        $($field $value)*)
    };
}
pub use crate::__aws_eks_AccessEntry as AccessEntry;
impl crate::template::ToResource for AccessEntry_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EKS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AccessEntry"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_policies {
            properties.insert(
                "AccessPolicies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ClusterName".to_string(),
            crate::value::ToValue::to_value(&self.cluster_name),
        );
        if let Some(ref value) = self.kubernetes_groups {
            properties.insert(
                "KubernetesGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PrincipalArn".to_string(),
            crate::value::ToValue::to_value(&self.principal_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.username {
            properties.insert(
                "Username".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-addon.html
pub struct Addon_ {
    pub addon_name: crate::value::ExpString,
    pub addon_version: Option<crate::value::ExpString>,
    pub cluster_name: crate::value::ExpString,
    pub configuration_values: Option<crate::value::ExpString>,
    pub namespace_config: Option<super::eks::addon::NamespaceConfig_>,
    pub pod_identity_associations: Option<Vec<super::eks::addon::PodIdentityAssociation_>>,
    pub preserve_on_delete: Option<crate::value::ExpBool>,
    pub resolve_conflicts: Option<crate::value::ExpString>,
    pub service_account_role_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_eks_Addon {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EKS::Addon" $($field
        $value)*)
    };
}
pub use crate::__aws_eks_Addon as Addon;
impl crate::template::ToResource for Addon_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EKS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Addon"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AddonName".to_string(),
            crate::value::ToValue::to_value(&self.addon_name),
        );
        if let Some(ref value) = self.addon_version {
            properties.insert(
                "AddonVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ClusterName".to_string(),
            crate::value::ToValue::to_value(&self.cluster_name),
        );
        if let Some(ref value) = self.configuration_values {
            properties.insert(
                "ConfigurationValues".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.namespace_config {
            properties.insert(
                "NamespaceConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.pod_identity_associations {
            properties.insert(
                "PodIdentityAssociations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.preserve_on_delete {
            properties.insert(
                "PreserveOnDelete".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resolve_conflicts {
            properties.insert(
                "ResolveConflicts".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_account_role_arn {
            properties.insert(
                "ServiceAccountRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-capability.html
pub struct Capability_ {
    pub capability_name: crate::value::ExpString,
    pub cluster_name: crate::value::ExpString,
    pub configuration: Option<super::eks::capability::CapabilityConfiguration_>,
    pub delete_propagation_policy: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_eks_Capability {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EKS::Capability" $($field
        $value)*)
    };
}
pub use crate::__aws_eks_Capability as Capability;
impl crate::template::ToResource for Capability_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EKS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Capability"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CapabilityName".to_string(),
            crate::value::ToValue::to_value(&self.capability_name),
        );
        properties.insert(
            "ClusterName".to_string(),
            crate::value::ToValue::to_value(&self.cluster_name),
        );
        if let Some(ref value) = self.configuration {
            properties.insert(
                "Configuration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DeletePropagationPolicy".to_string(),
            crate::value::ToValue::to_value(&self.delete_propagation_policy),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-cluster.html
pub struct Cluster_ {
    pub access_config: Option<super::eks::cluster::AccessConfig_>,
    pub bootstrap_self_managed_addons: Option<crate::value::ExpBool>,
    pub compute_config: Option<super::eks::cluster::ComputeConfig_>,
    pub control_plane_scaling_config: Option<super::eks::cluster::ControlPlaneScalingConfig_>,
    pub deletion_protection: Option<crate::value::ExpBool>,
    pub encryption_config: Option<Vec<super::eks::cluster::EncryptionConfig_>>,
    pub force: Option<crate::value::ExpBool>,
    pub kubernetes_network_config: Option<super::eks::cluster::KubernetesNetworkConfig_>,
    pub logging: Option<super::eks::cluster::Logging_>,
    pub name: Option<crate::value::ExpString>,
    pub outpost_config: Option<super::eks::cluster::OutpostConfig_>,
    pub remote_network_config: Option<super::eks::cluster::RemoteNetworkConfig_>,
    pub resources_vpc_config: super::eks::cluster::ResourcesVpcConfig_,
    pub role_arn: crate::value::ExpString,
    pub storage_config: Option<super::eks::cluster::StorageConfig_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub upgrade_policy: Option<super::eks::cluster::UpgradePolicy_>,
    pub version: Option<crate::value::ExpString>,
    pub zonal_shift_config: Option<super::eks::cluster::ZonalShiftConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_eks_Cluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EKS::Cluster" $($field
        $value)*)
    };
}
pub use crate::__aws_eks_Cluster as Cluster;
impl crate::template::ToResource for Cluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EKS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Cluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_config {
            properties.insert(
                "AccessConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.bootstrap_self_managed_addons {
            properties.insert(
                "BootstrapSelfManagedAddons".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.compute_config {
            properties.insert(
                "ComputeConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.control_plane_scaling_config {
            properties.insert(
                "ControlPlaneScalingConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deletion_protection {
            properties.insert(
                "DeletionProtection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_config {
            properties.insert(
                "EncryptionConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.force {
            properties.insert("Force".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.kubernetes_network_config {
            properties.insert(
                "KubernetesNetworkConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logging {
            properties.insert(
                "Logging".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.outpost_config {
            properties.insert(
                "OutpostConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.remote_network_config {
            properties.insert(
                "RemoteNetworkConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResourcesVpcConfig".to_string(),
            crate::value::ToValue::to_value(&self.resources_vpc_config),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.storage_config {
            properties.insert(
                "StorageConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.upgrade_policy {
            properties.insert(
                "UpgradePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.version {
            properties.insert(
                "Version".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.zonal_shift_config {
            properties.insert(
                "ZonalShiftConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-fargateprofile.html
pub struct FargateProfile_ {
    pub cluster_name: crate::value::ExpString,
    pub fargate_profile_name: Option<crate::value::ExpString>,
    pub pod_execution_role_arn: crate::value::ExpString,
    pub selectors: Vec<super::eks::fargateprofile::Selector_>,
    pub subnets: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_eks_FargateProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EKS::FargateProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_eks_FargateProfile as FargateProfile;
impl crate::template::ToResource for FargateProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EKS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FargateProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ClusterName".to_string(),
            crate::value::ToValue::to_value(&self.cluster_name),
        );
        if let Some(ref value) = self.fargate_profile_name {
            properties.insert(
                "FargateProfileName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PodExecutionRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.pod_execution_role_arn),
        );
        properties.insert(
            "Selectors".to_string(),
            crate::value::ToValue::to_value(&self.selectors),
        );
        if let Some(ref value) = self.subnets {
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-identityproviderconfig.html
pub struct IdentityProviderConfig_ {
    pub cluster_name: crate::value::ExpString,
    pub identity_provider_config_name: Option<crate::value::ExpString>,
    pub oidc: Option<super::eks::identityproviderconfig::OidcIdentityProviderConfig_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_eks_IdentityProviderConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EKS::IdentityProviderConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_eks_IdentityProviderConfig as IdentityProviderConfig;
impl crate::template::ToResource for IdentityProviderConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EKS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IdentityProviderConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ClusterName".to_string(),
            crate::value::ToValue::to_value(&self.cluster_name),
        );
        if let Some(ref value) = self.identity_provider_config_name {
            properties.insert(
                "IdentityProviderConfigName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.oidc {
            properties.insert("Oidc".to_string(), crate::value::ToValue::to_value(value));
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-nodegroup.html
pub struct Nodegroup_ {
    pub ami_type: Option<crate::value::ExpString>,
    pub capacity_type: Option<crate::value::ExpString>,
    pub cluster_name: crate::value::ExpString,
    pub disk_size: Option<i32>,
    pub force_update_enabled: Option<crate::value::ExpBool>,
    pub instance_types: Option<Vec<crate::value::ExpString>>,
    pub labels: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub launch_template: Option<super::eks::nodegroup::LaunchTemplateSpecification_>,
    pub node_repair_config: Option<super::eks::nodegroup::NodeRepairConfig_>,
    pub node_role: crate::value::ExpString,
    pub nodegroup_name: Option<crate::value::ExpString>,
    pub release_version: Option<crate::value::ExpString>,
    pub remote_access: Option<super::eks::nodegroup::RemoteAccess_>,
    pub scaling_config: Option<super::eks::nodegroup::ScalingConfig_>,
    pub subnets: Vec<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub taints: Option<Vec<super::eks::nodegroup::Taint_>>,
    pub update_config: Option<super::eks::nodegroup::UpdateConfig_>,
    pub version: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_eks_Nodegroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EKS::Nodegroup" $($field
        $value)*)
    };
}
pub use crate::__aws_eks_Nodegroup as Nodegroup;
impl crate::template::ToResource for Nodegroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EKS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Nodegroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.ami_type {
            properties.insert(
                "AmiType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.capacity_type {
            properties.insert(
                "CapacityType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ClusterName".to_string(),
            crate::value::ToValue::to_value(&self.cluster_name),
        );
        if let Some(ref value) = self.disk_size {
            properties.insert(
                "DiskSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.force_update_enabled {
            properties.insert(
                "ForceUpdateEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_types {
            properties.insert(
                "InstanceTypes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.labels {
            properties.insert("Labels".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.launch_template {
            properties.insert(
                "LaunchTemplate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.node_repair_config {
            properties.insert(
                "NodeRepairConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NodeRole".to_string(),
            crate::value::ToValue::to_value(&self.node_role),
        );
        if let Some(ref value) = self.nodegroup_name {
            properties.insert(
                "NodegroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.release_version {
            properties.insert(
                "ReleaseVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.remote_access {
            properties.insert(
                "RemoteAccess".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scaling_config {
            properties.insert(
                "ScalingConfig".to_string(),
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
        if let Some(ref value) = self.taints {
            properties.insert("Taints".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.update_config {
            properties.insert(
                "UpdateConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.version {
            properties.insert(
                "Version".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eks-podidentityassociation.html
pub struct PodIdentityAssociation_ {
    pub cluster_name: crate::value::ExpString,
    pub disable_session_tags: Option<crate::value::ExpBool>,
    pub namespace: crate::value::ExpString,
    pub policy: Option<crate::value::ExpString>,
    pub role_arn: crate::value::ExpString,
    pub service_account: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_role_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_eks_PodIdentityAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EKS::PodIdentityAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_eks_PodIdentityAssociation as PodIdentityAssociation;
impl crate::template::ToResource for PodIdentityAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EKS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PodIdentityAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ClusterName".to_string(),
            crate::value::ToValue::to_value(&self.cluster_name),
        );
        if let Some(ref value) = self.disable_session_tags {
            properties.insert(
                "DisableSessionTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Namespace".to_string(),
            crate::value::ToValue::to_value(&self.namespace),
        );
        if let Some(ref value) = self.policy {
            properties.insert("Policy".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties.insert(
            "ServiceAccount".to_string(),
            crate::value::ToValue::to_value(&self.service_account),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.target_role_arn {
            properties.insert(
                "TargetRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
