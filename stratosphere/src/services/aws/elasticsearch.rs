pub mod domain {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-advancedsecurityoptionsinput.html
    pub struct AdvancedSecurityOptionsInput_ {
        pub anonymous_auth_enabled: Option<crate::value::ExpBool>,
        pub enabled: Option<crate::value::ExpBool>,
        pub internal_user_database_enabled: Option<crate::value::ExpBool>,
        pub master_user_options: Option<Box<MasterUserOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticsearch_Domain_AdvancedSecurityOptionsInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Elasticsearch::Domain.AdvancedSecurityOptionsInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticsearch_Domain_AdvancedSecurityOptionsInput as AdvancedSecurityOptionsInput;
    impl crate::value::ToValue for AdvancedSecurityOptionsInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.anonymous_auth_enabled {
                properties.insert(
                    "AnonymousAuthEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.internal_user_database_enabled {
                properties.insert(
                    "InternalUserDatabaseEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.master_user_options {
                properties.insert(
                    "MasterUserOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-cognitooptions.html
    pub struct CognitoOptions_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub identity_pool_id: Option<crate::value::ExpString>,
        pub role_arn: Option<crate::value::ExpString>,
        pub user_pool_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticsearch_Domain_CognitoOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Elasticsearch::Domain.CognitoOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticsearch_Domain_CognitoOptions as CognitoOptions;
    impl crate::value::ToValue for CognitoOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.identity_pool_id {
                properties.insert(
                    "IdentityPoolId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_pool_id {
                properties.insert(
                    "UserPoolId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-coldstorageoptions.html
    pub struct ColdStorageOptions_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticsearch_Domain_ColdStorageOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Elasticsearch::Domain.ColdStorageOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticsearch_Domain_ColdStorageOptions as ColdStorageOptions;
    impl crate::value::ToValue for ColdStorageOptions_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-domainendpointoptions.html
    pub struct DomainEndpointOptions_ {
        pub custom_endpoint: Option<crate::value::ExpString>,
        pub custom_endpoint_certificate_arn: Option<crate::value::ExpString>,
        pub custom_endpoint_enabled: Option<crate::value::ExpBool>,
        pub enforce_https: Option<crate::value::ExpBool>,
        pub tls_security_policy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticsearch_Domain_DomainEndpointOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Elasticsearch::Domain.DomainEndpointOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticsearch_Domain_DomainEndpointOptions as DomainEndpointOptions;
    impl crate::value::ToValue for DomainEndpointOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_endpoint {
                properties.insert(
                    "CustomEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_endpoint_certificate_arn {
                properties.insert(
                    "CustomEndpointCertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_endpoint_enabled {
                properties.insert(
                    "CustomEndpointEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enforce_https {
                properties.insert(
                    "EnforceHTTPS".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tls_security_policy {
                properties.insert(
                    "TLSSecurityPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-ebsoptions.html
    pub struct EBSOptions_ {
        pub ebs_enabled: Option<crate::value::ExpBool>,
        pub iops: Option<i64>,
        pub volume_size: Option<i64>,
        pub volume_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticsearch_Domain_EBSOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Elasticsearch::Domain.EBSOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticsearch_Domain_EBSOptions as EBSOptions;
    impl crate::value::ToValue for EBSOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ebs_enabled {
                properties.insert(
                    "EBSEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iops {
                properties.insert("Iops".to_string(), crate::value::ToValue::to_value(value));
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-elasticsearchclusterconfig.html
    pub struct ElasticsearchClusterConfig_ {
        pub cold_storage_options: Option<Box<ColdStorageOptions_>>,
        pub dedicated_master_count: Option<i64>,
        pub dedicated_master_enabled: Option<crate::value::ExpBool>,
        pub dedicated_master_type: Option<crate::value::ExpString>,
        pub instance_count: Option<i64>,
        pub instance_type: Option<crate::value::ExpString>,
        pub warm_count: Option<i64>,
        pub warm_enabled: Option<crate::value::ExpBool>,
        pub warm_type: Option<crate::value::ExpString>,
        pub zone_awareness_config: Option<Box<ZoneAwarenessConfig_>>,
        pub zone_awareness_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticsearch_Domain_ElasticsearchClusterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Elasticsearch::Domain.ElasticsearchClusterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticsearch_Domain_ElasticsearchClusterConfig as ElasticsearchClusterConfig;
    impl crate::value::ToValue for ElasticsearchClusterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cold_storage_options {
                properties.insert(
                    "ColdStorageOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dedicated_master_count {
                properties.insert(
                    "DedicatedMasterCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dedicated_master_enabled {
                properties.insert(
                    "DedicatedMasterEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dedicated_master_type {
                properties.insert(
                    "DedicatedMasterType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_count {
                properties.insert(
                    "InstanceCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_type {
                properties.insert(
                    "InstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.warm_count {
                properties.insert(
                    "WarmCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.warm_enabled {
                properties.insert(
                    "WarmEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.warm_type {
                properties.insert(
                    "WarmType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.zone_awareness_config {
                properties.insert(
                    "ZoneAwarenessConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.zone_awareness_enabled {
                properties.insert(
                    "ZoneAwarenessEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-encryptionatrestoptions.html
    pub struct EncryptionAtRestOptions_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticsearch_Domain_EncryptionAtRestOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Elasticsearch::Domain.EncryptionAtRestOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticsearch_Domain_EncryptionAtRestOptions as EncryptionAtRestOptions;
    impl crate::value::ToValue for EncryptionAtRestOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-logpublishingoption.html
    pub struct LogPublishingOption_ {
        pub cloud_watch_logs_log_group_arn: Option<crate::value::ExpString>,
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticsearch_Domain_LogPublishingOption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Elasticsearch::Domain.LogPublishingOption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticsearch_Domain_LogPublishingOption as LogPublishingOption;
    impl crate::value::ToValue for LogPublishingOption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_logs_log_group_arn {
                properties.insert(
                    "CloudWatchLogsLogGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-masteruseroptions.html
    pub struct MasterUserOptions_ {
        pub master_user_arn: Option<crate::value::ExpString>,
        pub master_user_name: Option<crate::value::ExpString>,
        pub master_user_password: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticsearch_Domain_MasterUserOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Elasticsearch::Domain.MasterUserOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticsearch_Domain_MasterUserOptions as MasterUserOptions;
    impl crate::value::ToValue for MasterUserOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.master_user_arn {
                properties.insert(
                    "MasterUserARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.master_user_name {
                properties.insert(
                    "MasterUserName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.master_user_password {
                properties.insert(
                    "MasterUserPassword".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-nodetonodeencryptionoptions.html
    pub struct NodeToNodeEncryptionOptions_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticsearch_Domain_NodeToNodeEncryptionOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Elasticsearch::Domain.NodeToNodeEncryptionOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticsearch_Domain_NodeToNodeEncryptionOptions as NodeToNodeEncryptionOptions;
    impl crate::value::ToValue for NodeToNodeEncryptionOptions_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-snapshotoptions.html
    pub struct SnapshotOptions_ {
        pub automated_snapshot_start_hour: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticsearch_Domain_SnapshotOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Elasticsearch::Domain.SnapshotOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticsearch_Domain_SnapshotOptions as SnapshotOptions;
    impl crate::value::ToValue for SnapshotOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.automated_snapshot_start_hour {
                properties.insert(
                    "AutomatedSnapshotStartHour".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-vpcoptions.html
    pub struct VPCOptions_ {
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticsearch_Domain_VPCOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Elasticsearch::Domain.VPCOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticsearch_Domain_VPCOptions as VPCOptions;
    impl crate::value::ToValue for VPCOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticsearch-domain-zoneawarenessconfig.html
    pub struct ZoneAwarenessConfig_ {
        pub availability_zone_count: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticsearch_Domain_ZoneAwarenessConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Elasticsearch::Domain.ZoneAwarenessConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticsearch_Domain_ZoneAwarenessConfig as ZoneAwarenessConfig;
    impl crate::value::ToValue for ZoneAwarenessConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.availability_zone_count {
                properties.insert(
                    "AvailabilityZoneCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticsearch-domain.html
pub struct Domain_ {
    pub access_policies: Option<serde_json::Value>,
    pub advanced_options: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub advanced_security_options:
        Option<super::elasticsearch::domain::AdvancedSecurityOptionsInput_>,
    pub cognito_options: Option<super::elasticsearch::domain::CognitoOptions_>,
    pub domain_endpoint_options: Option<super::elasticsearch::domain::DomainEndpointOptions_>,
    pub domain_name: Option<crate::value::ExpString>,
    pub ebs_options: Option<super::elasticsearch::domain::EBSOptions_>,
    pub elasticsearch_cluster_config:
        Option<super::elasticsearch::domain::ElasticsearchClusterConfig_>,
    pub elasticsearch_version: Option<crate::value::ExpString>,
    pub encryption_at_rest_options: Option<super::elasticsearch::domain::EncryptionAtRestOptions_>,
    pub log_publishing_options: Option<
        std::collections::BTreeMap<String, super::elasticsearch::domain::LogPublishingOption_>,
    >,
    pub node_to_node_encryption_options:
        Option<super::elasticsearch::domain::NodeToNodeEncryptionOptions_>,
    pub snapshot_options: Option<super::elasticsearch::domain::SnapshotOptions_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_options: Option<super::elasticsearch::domain::VPCOptions_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticsearch_Domain {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Elasticsearch::Domain"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticsearch_Domain as Domain;
impl crate::template::ToResource for Domain_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Elasticsearch"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Domain"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_policies {
            properties.insert(
                "AccessPolicies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.advanced_options {
            properties.insert(
                "AdvancedOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.advanced_security_options {
            properties.insert(
                "AdvancedSecurityOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cognito_options {
            properties.insert(
                "CognitoOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_endpoint_options {
            properties.insert(
                "DomainEndpointOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_name {
            properties.insert(
                "DomainName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ebs_options {
            properties.insert(
                "EBSOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.elasticsearch_cluster_config {
            properties.insert(
                "ElasticsearchClusterConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.elasticsearch_version {
            properties.insert(
                "ElasticsearchVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_at_rest_options {
            properties.insert(
                "EncryptionAtRestOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_publishing_options {
            properties.insert(
                "LogPublishingOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.node_to_node_encryption_options {
            properties.insert(
                "NodeToNodeEncryptionOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_options {
            properties.insert(
                "SnapshotOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_options {
            properties.insert(
                "VPCOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
