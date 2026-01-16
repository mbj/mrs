pub mod application {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-application-appconfig.html
    pub struct AppConfig_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Application_AppConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Application.AppConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Application_AppConfig as AppConfig;
    impl crate::value::ToValue for AppConfig_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-application-datasource.html
    pub struct DataSource_ {
        pub data_source_arn: crate::value::ExpString,
        pub data_source_description: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Application_DataSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Application.DataSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Application_DataSource as DataSource;
    impl crate::value::ToValue for DataSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataSourceArn".to_string(),
                crate::value::ToValue::to_value(&self.data_source_arn),
            );
            if let Some(ref value) = self.data_source_description {
                properties.insert(
                    "DataSourceDescription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-application-iamidentitycenteroptions.html
    pub struct IamIdentityCenterOptions_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub iam_identity_center_instance_arn: Option<crate::value::ExpString>,
        pub iam_role_for_identity_center_application_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Application_IamIdentityCenterOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Application.IamIdentityCenterOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Application_IamIdentityCenterOptions as IamIdentityCenterOptions;
    impl crate::value::ToValue for IamIdentityCenterOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iam_identity_center_instance_arn {
                properties.insert(
                    "IamIdentityCenterInstanceArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iam_role_for_identity_center_application_arn {
                properties.insert(
                    "IamRoleForIdentityCenterApplicationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod domain {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-advancedsecurityoptionsinput.html
    pub struct AdvancedSecurityOptionsInput_ {
        pub anonymous_auth_disable_date: Option<crate::value::ExpString>,
        pub anonymous_auth_enabled: Option<crate::value::ExpBool>,
        pub enabled: Option<crate::value::ExpBool>,
        pub iam_federation_options: Option<Box<IAMFederationOptions_>>,
        pub internal_user_database_enabled: Option<crate::value::ExpBool>,
        pub jwt_options: Option<Box<JWTOptions_>>,
        pub master_user_options: Option<Box<MasterUserOptions_>>,
        pub saml_options: Option<Box<SAMLOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_AdvancedSecurityOptionsInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.AdvancedSecurityOptionsInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_AdvancedSecurityOptionsInput as AdvancedSecurityOptionsInput;
    impl crate::value::ToValue for AdvancedSecurityOptionsInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.anonymous_auth_disable_date {
                properties.insert(
                    "AnonymousAuthDisableDate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
            if let Some(ref value) = self.iam_federation_options {
                properties.insert(
                    "IAMFederationOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.internal_user_database_enabled {
                properties.insert(
                    "InternalUserDatabaseEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.jwt_options {
                properties.insert(
                    "JWTOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.master_user_options {
                properties.insert(
                    "MasterUserOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.saml_options {
                properties.insert(
                    "SAMLOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-clusterconfig.html
    pub struct ClusterConfig_ {
        pub cold_storage_options: Option<Box<ColdStorageOptions_>>,
        pub dedicated_master_count: Option<i64>,
        pub dedicated_master_enabled: Option<crate::value::ExpBool>,
        pub dedicated_master_type: Option<crate::value::ExpString>,
        pub instance_count: Option<i64>,
        pub instance_type: Option<crate::value::ExpString>,
        pub multi_az_with_standby_enabled: Option<crate::value::ExpBool>,
        pub node_options: Option<Vec<NodeOption_>>,
        pub warm_count: Option<i64>,
        pub warm_enabled: Option<crate::value::ExpBool>,
        pub warm_type: Option<crate::value::ExpString>,
        pub zone_awareness_config: Option<Box<ZoneAwarenessConfig_>>,
        pub zone_awareness_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_ClusterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.ClusterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_ClusterConfig as ClusterConfig;
    impl crate::value::ToValue for ClusterConfig_ {
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
            if let Some(ref value) = self.multi_az_with_standby_enabled {
                properties.insert(
                    "MultiAZWithStandbyEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.node_options {
                properties.insert(
                    "NodeOptions".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-cognitooptions.html
    pub struct CognitoOptions_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub identity_pool_id: Option<crate::value::ExpString>,
        pub role_arn: Option<crate::value::ExpString>,
        pub user_pool_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_CognitoOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.CognitoOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_CognitoOptions as CognitoOptions;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-coldstorageoptions.html
    pub struct ColdStorageOptions_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_ColdStorageOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.ColdStorageOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_ColdStorageOptions as ColdStorageOptions;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-domainendpointoptions.html
    pub struct DomainEndpointOptions_ {
        pub custom_endpoint: Option<crate::value::ExpString>,
        pub custom_endpoint_certificate_arn: Option<crate::value::ExpString>,
        pub custom_endpoint_enabled: Option<crate::value::ExpBool>,
        pub enforce_https: Option<crate::value::ExpBool>,
        pub tls_security_policy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_DomainEndpointOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.DomainEndpointOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_DomainEndpointOptions as DomainEndpointOptions;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-ebsoptions.html
    pub struct EBSOptions_ {
        pub ebs_enabled: Option<crate::value::ExpBool>,
        pub iops: Option<i64>,
        pub throughput: Option<i64>,
        pub volume_size: Option<i64>,
        pub volume_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_EBSOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.EBSOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_EBSOptions as EBSOptions;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-encryptionatrestoptions.html
    pub struct EncryptionAtRestOptions_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_EncryptionAtRestOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.EncryptionAtRestOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_EncryptionAtRestOptions as EncryptionAtRestOptions;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-iamfederationoptions.html
    pub struct IAMFederationOptions_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub roles_key: Option<crate::value::ExpString>,
        pub subject_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_IAMFederationOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.IAMFederationOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_IAMFederationOptions as IAMFederationOptions;
    impl crate::value::ToValue for IAMFederationOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.roles_key {
                properties.insert(
                    "RolesKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subject_key {
                properties.insert(
                    "SubjectKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-identitycenteroptions.html
    pub struct IdentityCenterOptions_ {
        pub enabled_api_access: Option<crate::value::ExpBool>,
        pub identity_center_application_arn: Option<crate::value::ExpString>,
        pub identity_center_instance_arn: Option<crate::value::ExpString>,
        pub identity_store_id: Option<crate::value::ExpString>,
        pub roles_key: Option<crate::value::ExpString>,
        pub subject_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_IdentityCenterOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.IdentityCenterOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_IdentityCenterOptions as IdentityCenterOptions;
    impl crate::value::ToValue for IdentityCenterOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled_api_access {
                properties.insert(
                    "EnabledAPIAccess".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.identity_center_application_arn {
                properties.insert(
                    "IdentityCenterApplicationARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.identity_center_instance_arn {
                properties.insert(
                    "IdentityCenterInstanceARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.identity_store_id {
                properties.insert(
                    "IdentityStoreId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.roles_key {
                properties.insert(
                    "RolesKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subject_key {
                properties.insert(
                    "SubjectKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-idp.html
    pub struct Idp_ {
        pub entity_id: crate::value::ExpString,
        pub metadata_content: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_Idp {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.Idp"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_Idp as Idp;
    impl crate::value::ToValue for Idp_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EntityId".to_string(),
                crate::value::ToValue::to_value(&self.entity_id),
            );
            properties.insert(
                "MetadataContent".to_string(),
                crate::value::ToValue::to_value(&self.metadata_content),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-jwtoptions.html
    pub struct JWTOptions_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub public_key: Option<crate::value::ExpString>,
        pub roles_key: Option<crate::value::ExpString>,
        pub subject_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_JWTOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.JWTOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_JWTOptions as JWTOptions;
    impl crate::value::ToValue for JWTOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.public_key {
                properties.insert(
                    "PublicKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.roles_key {
                properties.insert(
                    "RolesKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subject_key {
                properties.insert(
                    "SubjectKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-logpublishingoption.html
    pub struct LogPublishingOption_ {
        pub cloud_watch_logs_log_group_arn: Option<crate::value::ExpString>,
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_LogPublishingOption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.LogPublishingOption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_LogPublishingOption as LogPublishingOption;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-masteruseroptions.html
    pub struct MasterUserOptions_ {
        pub master_user_arn: Option<crate::value::ExpString>,
        pub master_user_name: Option<crate::value::ExpString>,
        pub master_user_password: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_MasterUserOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.MasterUserOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_MasterUserOptions as MasterUserOptions;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-nodeconfig.html
    pub struct NodeConfig_ {
        pub count: Option<i64>,
        pub enabled: Option<crate::value::ExpBool>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_NodeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.NodeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_NodeConfig as NodeConfig;
    impl crate::value::ToValue for NodeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.count {
                properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-nodeoption.html
    pub struct NodeOption_ {
        pub node_config: Option<Box<NodeConfig_>>,
        pub node_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_NodeOption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.NodeOption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_NodeOption as NodeOption;
    impl crate::value::ToValue for NodeOption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.node_config {
                properties.insert(
                    "NodeConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.node_type {
                properties.insert(
                    "NodeType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-nodetonodeencryptionoptions.html
    pub struct NodeToNodeEncryptionOptions_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_NodeToNodeEncryptionOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.NodeToNodeEncryptionOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_NodeToNodeEncryptionOptions as NodeToNodeEncryptionOptions;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-offpeakwindow.html
    pub struct OffPeakWindow_ {
        pub window_start_time: Option<Box<WindowStartTime_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_OffPeakWindow {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.OffPeakWindow"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_OffPeakWindow as OffPeakWindow;
    impl crate::value::ToValue for OffPeakWindow_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.window_start_time {
                properties.insert(
                    "WindowStartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-offpeakwindowoptions.html
    pub struct OffPeakWindowOptions_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub off_peak_window: Option<Box<OffPeakWindow_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_OffPeakWindowOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.OffPeakWindowOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_OffPeakWindowOptions as OffPeakWindowOptions;
    impl crate::value::ToValue for OffPeakWindowOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.off_peak_window {
                properties.insert(
                    "OffPeakWindow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-samloptions.html
    pub struct SAMLOptions_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub idp: Option<Box<Idp_>>,
        pub master_backend_role: Option<crate::value::ExpString>,
        pub master_user_name: Option<crate::value::ExpString>,
        pub roles_key: Option<crate::value::ExpString>,
        pub session_timeout_minutes: Option<i64>,
        pub subject_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_SAMLOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.SAMLOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_SAMLOptions as SAMLOptions;
    impl crate::value::ToValue for SAMLOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.idp {
                properties.insert("Idp".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.master_backend_role {
                properties.insert(
                    "MasterBackendRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.master_user_name {
                properties.insert(
                    "MasterUserName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.roles_key {
                properties.insert(
                    "RolesKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.session_timeout_minutes {
                properties.insert(
                    "SessionTimeoutMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subject_key {
                properties.insert(
                    "SubjectKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-servicesoftwareoptions.html
    pub struct ServiceSoftwareOptions_ {
        pub automated_update_date: Option<crate::value::ExpString>,
        pub cancellable: Option<crate::value::ExpBool>,
        pub current_version: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub new_version: Option<crate::value::ExpString>,
        pub optional_deployment: Option<crate::value::ExpBool>,
        pub update_available: Option<crate::value::ExpBool>,
        pub update_status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_ServiceSoftwareOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.ServiceSoftwareOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_ServiceSoftwareOptions as ServiceSoftwareOptions;
    impl crate::value::ToValue for ServiceSoftwareOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.automated_update_date {
                properties.insert(
                    "AutomatedUpdateDate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cancellable {
                properties.insert(
                    "Cancellable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.current_version {
                properties.insert(
                    "CurrentVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.new_version {
                properties.insert(
                    "NewVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.optional_deployment {
                properties.insert(
                    "OptionalDeployment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.update_available {
                properties.insert(
                    "UpdateAvailable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.update_status {
                properties.insert(
                    "UpdateStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-snapshotoptions.html
    pub struct SnapshotOptions_ {
        pub automated_snapshot_start_hour: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_SnapshotOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.SnapshotOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_SnapshotOptions as SnapshotOptions;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-softwareupdateoptions.html
    pub struct SoftwareUpdateOptions_ {
        pub auto_software_update_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_SoftwareUpdateOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.SoftwareUpdateOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_SoftwareUpdateOptions as SoftwareUpdateOptions;
    impl crate::value::ToValue for SoftwareUpdateOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_software_update_enabled {
                properties.insert(
                    "AutoSoftwareUpdateEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-vpcoptions.html
    pub struct VPCOptions_ {
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_VPCOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.VPCOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_VPCOptions as VPCOptions;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-windowstarttime.html
    pub struct WindowStartTime_ {
        pub hours: i64,
        pub minutes: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_WindowStartTime {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.WindowStartTime"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_WindowStartTime as WindowStartTime;
    impl crate::value::ToValue for WindowStartTime_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Hours".to_string(),
                crate::value::ToValue::to_value(&self.hours),
            );
            properties.insert(
                "Minutes".to_string(),
                crate::value::ToValue::to_value(&self.minutes),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchservice-domain-zoneawarenessconfig.html
    pub struct ZoneAwarenessConfig_ {
        pub availability_zone_count: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchservice_Domain_ZoneAwarenessConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::OpenSearchService::Domain.ZoneAwarenessConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchservice_Domain_ZoneAwarenessConfig as ZoneAwarenessConfig;
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-application.html
pub struct Application_ {
    pub app_configs: Option<Vec<super::opensearchservice::application::AppConfig_>>,
    pub data_sources: Option<Vec<super::opensearchservice::application::DataSource_>>,
    pub endpoint: Option<crate::value::ExpString>,
    pub iam_identity_center_options:
        Option<super::opensearchservice::application::IamIdentityCenterOptions_>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opensearchservice_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::OpenSearchService::Application"
        $($field $value)*)
    };
}
pub use crate::__aws_opensearchservice_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpenSearchService"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.app_configs {
            properties.insert(
                "AppConfigs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_sources {
            properties.insert(
                "DataSources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint {
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iam_identity_center_options {
            properties.insert(
                "IamIdentityCenterOptions".to_string(),
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
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchservice-domain.html
pub struct Domain_ {
    pub access_policies: Option<serde_json::Value>,
    pub advanced_options: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub advanced_security_options:
        Option<super::opensearchservice::domain::AdvancedSecurityOptionsInput_>,
    pub cluster_config: Option<super::opensearchservice::domain::ClusterConfig_>,
    pub cognito_options: Option<super::opensearchservice::domain::CognitoOptions_>,
    pub domain_endpoint_options: Option<super::opensearchservice::domain::DomainEndpointOptions_>,
    pub domain_name: Option<crate::value::ExpString>,
    pub ebs_options: Option<super::opensearchservice::domain::EBSOptions_>,
    pub encryption_at_rest_options:
        Option<super::opensearchservice::domain::EncryptionAtRestOptions_>,
    pub engine_version: Option<crate::value::ExpString>,
    pub ip_address_type: Option<crate::value::ExpString>,
    pub identity_center_options: Option<super::opensearchservice::domain::IdentityCenterOptions_>,
    pub log_publishing_options: Option<
        std::collections::BTreeMap<String, super::opensearchservice::domain::LogPublishingOption_>,
    >,
    pub node_to_node_encryption_options:
        Option<super::opensearchservice::domain::NodeToNodeEncryptionOptions_>,
    pub off_peak_window_options: Option<super::opensearchservice::domain::OffPeakWindowOptions_>,
    pub skip_shard_migration_wait: Option<crate::value::ExpBool>,
    pub snapshot_options: Option<super::opensearchservice::domain::SnapshotOptions_>,
    pub software_update_options: Option<super::opensearchservice::domain::SoftwareUpdateOptions_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_options: Option<super::opensearchservice::domain::VPCOptions_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opensearchservice_Domain {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::OpenSearchService::Domain"
        $($field $value)*)
    };
}
pub use crate::__aws_opensearchservice_Domain as Domain;
impl crate::template::ToResource for Domain_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpenSearchService"),
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
        if let Some(ref value) = self.cluster_config {
            properties.insert(
                "ClusterConfig".to_string(),
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
        if let Some(ref value) = self.encryption_at_rest_options {
            properties.insert(
                "EncryptionAtRestOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine_version {
            properties.insert(
                "EngineVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ip_address_type {
            properties.insert(
                "IPAddressType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.identity_center_options {
            properties.insert(
                "IdentityCenterOptions".to_string(),
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
        if let Some(ref value) = self.off_peak_window_options {
            properties.insert(
                "OffPeakWindowOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.skip_shard_migration_wait {
            properties.insert(
                "SkipShardMigrationWait".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_options {
            properties.insert(
                "SnapshotOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.software_update_options {
            properties.insert(
                "SoftwareUpdateOptions".to_string(),
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
