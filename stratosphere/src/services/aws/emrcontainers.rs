pub mod endpoint {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-endpoint-certificate.html
    pub struct Certificate_ {
        pub certificate_arn: Option<crate::value::ExpString>,
        pub certificate_data: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_Endpoint_Certificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::Endpoint.Certificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_Endpoint_Certificate as Certificate;
    impl crate::value::ToValue for Certificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.certificate_data {
                properties.insert(
                    "CertificateData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-endpoint-cloudwatchmonitoringconfiguration.html
    pub struct CloudWatchMonitoringConfiguration_ {
        pub log_group_name: crate::value::ExpString,
        pub log_stream_name_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_Endpoint_CloudWatchMonitoringConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::Endpoint.CloudWatchMonitoringConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_Endpoint_CloudWatchMonitoringConfiguration as CloudWatchMonitoringConfiguration;
    impl crate::value::ToValue for CloudWatchMonitoringConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LogGroupName".to_string(),
                crate::value::ToValue::to_value(&self.log_group_name),
            );
            if let Some(ref value) = self.log_stream_name_prefix {
                properties.insert(
                    "LogStreamNamePrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-endpoint-configurationoverrides.html
    pub struct ConfigurationOverrides_ {
        pub application_configuration: Option<Vec<EMREKSConfiguration_>>,
        pub monitoring_configuration: Option<Box<MonitoringConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_Endpoint_ConfigurationOverrides {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::Endpoint.ConfigurationOverrides"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_Endpoint_ConfigurationOverrides as ConfigurationOverrides;
    impl crate::value::ToValue for ConfigurationOverrides_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_configuration {
                properties.insert(
                    "ApplicationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.monitoring_configuration {
                properties.insert(
                    "MonitoringConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-endpoint-containerlogrotationconfiguration.html
    pub struct ContainerLogRotationConfiguration_ {
        pub max_files_to_keep: i32,
        pub rotation_size: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_Endpoint_ContainerLogRotationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::Endpoint.ContainerLogRotationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_Endpoint_ContainerLogRotationConfiguration as ContainerLogRotationConfiguration;
    impl crate::value::ToValue for ContainerLogRotationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxFilesToKeep".to_string(),
                crate::value::ToValue::to_value(&self.max_files_to_keep),
            );
            properties.insert(
                "RotationSize".to_string(),
                crate::value::ToValue::to_value(&self.rotation_size),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-endpoint-emreksconfiguration.html
    pub struct EMREKSConfiguration_ {
        pub classification: crate::value::ExpString,
        pub configurations: Option<Vec<EMREKSConfiguration_>>,
        pub properties: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_Endpoint_EMREKSConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::Endpoint.EMREKSConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_Endpoint_EMREKSConfiguration as EMREKSConfiguration;
    impl crate::value::ToValue for EMREKSConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Classification".to_string(),
                crate::value::ToValue::to_value(&self.classification),
            );
            if let Some(ref value) = self.configurations {
                properties.insert(
                    "Configurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.properties {
                properties.insert(
                    "Properties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-endpoint-monitoringconfiguration.html
    pub struct MonitoringConfiguration_ {
        pub cloud_watch_monitoring_configuration: Option<Box<CloudWatchMonitoringConfiguration_>>,
        pub container_log_rotation_configuration: Option<Box<ContainerLogRotationConfiguration_>>,
        pub persistent_app_ui: Option<crate::value::ExpString>,
        pub s3_monitoring_configuration: Option<Box<S3MonitoringConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_Endpoint_MonitoringConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::Endpoint.MonitoringConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_Endpoint_MonitoringConfiguration as MonitoringConfiguration;
    impl crate::value::ToValue for MonitoringConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_monitoring_configuration {
                properties.insert(
                    "CloudWatchMonitoringConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_log_rotation_configuration {
                properties.insert(
                    "ContainerLogRotationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.persistent_app_ui {
                properties.insert(
                    "PersistentAppUI".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_monitoring_configuration {
                properties.insert(
                    "S3MonitoringConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-endpoint-s3monitoringconfiguration.html
    pub struct S3MonitoringConfiguration_ {
        pub log_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_Endpoint_S3MonitoringConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::Endpoint.S3MonitoringConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_Endpoint_S3MonitoringConfiguration as S3MonitoringConfiguration;
    impl crate::value::ToValue for S3MonitoringConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LogUri".to_string(),
                crate::value::ToValue::to_value(&self.log_uri),
            );
            properties.into()
        }
    }
}
pub mod securityconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-securityconfiguration-atrestencryptionconfiguration.html
    pub struct AtRestEncryptionConfiguration_ {
        pub local_disk_encryption_configuration: Option<Box<LocalDiskEncryptionConfiguration_>>,
        pub s3_encryption_configuration: Option<Box<S3EncryptionConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_SecurityConfiguration_AtRestEncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::SecurityConfiguration.AtRestEncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_SecurityConfiguration_AtRestEncryptionConfiguration as AtRestEncryptionConfiguration;
    impl crate::value::ToValue for AtRestEncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.local_disk_encryption_configuration {
                properties.insert(
                    "LocalDiskEncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_encryption_configuration {
                properties.insert(
                    "S3EncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-securityconfiguration-authenticationconfiguration.html
    pub struct AuthenticationConfiguration_ {
        pub iam_configuration: Option<Box<IAMConfiguration_>>,
        pub identity_center_configuration: Option<Box<IdentityCenterConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_SecurityConfiguration_AuthenticationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::SecurityConfiguration.AuthenticationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_SecurityConfiguration_AuthenticationConfiguration as AuthenticationConfiguration;
    impl crate::value::ToValue for AuthenticationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.iam_configuration {
                properties.insert(
                    "IAMConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.identity_center_configuration {
                properties.insert(
                    "IdentityCenterConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-securityconfiguration-authorizationconfiguration.html
    pub struct AuthorizationConfiguration_ {
        pub lake_formation_configuration: Option<Box<LakeFormationConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_SecurityConfiguration_AuthorizationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::SecurityConfiguration.AuthorizationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_SecurityConfiguration_AuthorizationConfiguration as AuthorizationConfiguration;
    impl crate::value::ToValue for AuthorizationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.lake_formation_configuration {
                properties.insert(
                    "LakeFormationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-securityconfiguration-containerinfo.html
    pub struct ContainerInfo_ {
        pub eks_info: Option<Box<EksInfo_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_SecurityConfiguration_ContainerInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::SecurityConfiguration.ContainerInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_SecurityConfiguration_ContainerInfo as ContainerInfo;
    impl crate::value::ToValue for ContainerInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.eks_info {
                properties.insert(
                    "EksInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-securityconfiguration-containerprovider.html
    pub struct ContainerProvider_ {
        pub id: crate::value::ExpString,
        pub info: Option<Box<ContainerInfo_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_SecurityConfiguration_ContainerProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::SecurityConfiguration.ContainerProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_SecurityConfiguration_ContainerProvider as ContainerProvider;
    impl crate::value::ToValue for ContainerProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.info {
                properties.insert("Info".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-securityconfiguration-eksinfo.html
    pub struct EksInfo_ {
        pub namespace: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_SecurityConfiguration_EksInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::SecurityConfiguration.EksInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_SecurityConfiguration_EksInfo as EksInfo;
    impl crate::value::ToValue for EksInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.namespace {
                properties.insert(
                    "Namespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-securityconfiguration-encryptionconfiguration.html
    pub struct EncryptionConfiguration_ {
        pub at_rest_encryption_configuration: Option<Box<AtRestEncryptionConfiguration_>>,
        pub in_transit_encryption_configuration: Option<Box<InTransitEncryptionConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_SecurityConfiguration_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::SecurityConfiguration.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_SecurityConfiguration_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.at_rest_encryption_configuration {
                properties.insert(
                    "AtRestEncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.in_transit_encryption_configuration {
                properties.insert(
                    "InTransitEncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-securityconfiguration-iamconfiguration.html
    pub struct IAMConfiguration_ {
        pub system_role: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_SecurityConfiguration_IAMConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::SecurityConfiguration.IAMConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_SecurityConfiguration_IAMConfiguration as IAMConfiguration;
    impl crate::value::ToValue for IAMConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.system_role {
                properties.insert(
                    "SystemRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-securityconfiguration-identitycenterconfiguration.html
    pub struct IdentityCenterConfiguration_ {
        pub enable_identity_center: Option<crate::value::ExpBool>,
        pub identity_center_application_assignment_required: Option<crate::value::ExpBool>,
        pub identity_center_instance_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_SecurityConfiguration_IdentityCenterConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::SecurityConfiguration.IdentityCenterConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_SecurityConfiguration_IdentityCenterConfiguration as IdentityCenterConfiguration;
    impl crate::value::ToValue for IdentityCenterConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_identity_center {
                properties.insert(
                    "EnableIdentityCenter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.identity_center_application_assignment_required {
                properties.insert(
                    "IdentityCenterApplicationAssignmentRequired".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.identity_center_instance_arn {
                properties.insert(
                    "IdentityCenterInstanceARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-securityconfiguration-intransitencryptionconfiguration.html
    pub struct InTransitEncryptionConfiguration_ {
        pub tls_certificate_configuration: Option<Box<TLSCertificateConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_SecurityConfiguration_InTransitEncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::SecurityConfiguration.InTransitEncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_SecurityConfiguration_InTransitEncryptionConfiguration as InTransitEncryptionConfiguration;
    impl crate::value::ToValue for InTransitEncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.tls_certificate_configuration {
                properties.insert(
                    "TLSCertificateConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-securityconfiguration-lakeformationconfiguration.html
    pub struct LakeFormationConfiguration_ {
        pub authorized_session_tag_value: Option<crate::value::ExpString>,
        pub query_access_control_enabled: Option<crate::value::ExpBool>,
        pub query_engine_role_arn: Option<crate::value::ExpString>,
        pub secure_namespace_info: Option<Box<SecureNamespaceInfo_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_SecurityConfiguration_LakeFormationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::SecurityConfiguration.LakeFormationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_SecurityConfiguration_LakeFormationConfiguration as LakeFormationConfiguration;
    impl crate::value::ToValue for LakeFormationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authorized_session_tag_value {
                properties.insert(
                    "AuthorizedSessionTagValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.query_access_control_enabled {
                properties.insert(
                    "QueryAccessControlEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.query_engine_role_arn {
                properties.insert(
                    "QueryEngineRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secure_namespace_info {
                properties.insert(
                    "SecureNamespaceInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-securityconfiguration-localdiskencryptionconfiguration.html
    pub struct LocalDiskEncryptionConfiguration_ {
        pub aws_kms_key_id: Option<crate::value::ExpString>,
        pub encryption_key_provider_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_SecurityConfiguration_LocalDiskEncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::SecurityConfiguration.LocalDiskEncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_SecurityConfiguration_LocalDiskEncryptionConfiguration as LocalDiskEncryptionConfiguration;
    impl crate::value::ToValue for LocalDiskEncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_kms_key_id {
                properties.insert(
                    "AwsKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_key_provider_type {
                properties.insert(
                    "EncryptionKeyProviderType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-securityconfiguration-s3encryptionconfiguration.html
    pub struct S3EncryptionConfiguration_ {
        pub encryption_option: Option<crate::value::ExpString>,
        pub kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_SecurityConfiguration_S3EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::SecurityConfiguration.S3EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_SecurityConfiguration_S3EncryptionConfiguration as S3EncryptionConfiguration;
    impl crate::value::ToValue for S3EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption_option {
                properties.insert(
                    "EncryptionOption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KMSKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-securityconfiguration-securenamespaceinfo.html
    pub struct SecureNamespaceInfo_ {
        pub cluster_id: Option<crate::value::ExpString>,
        pub namespace: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_SecurityConfiguration_SecureNamespaceInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::SecurityConfiguration.SecureNamespaceInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_SecurityConfiguration_SecureNamespaceInfo as SecureNamespaceInfo;
    impl crate::value::ToValue for SecureNamespaceInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cluster_id {
                properties.insert(
                    "ClusterId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-securityconfiguration-securityconfigurationdata.html
    pub struct SecurityConfigurationData_ {
        pub authentication_configuration: Option<Box<AuthenticationConfiguration_>>,
        pub authorization_configuration: Option<Box<AuthorizationConfiguration_>>,
        pub encryption_configuration: Option<Box<EncryptionConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_SecurityConfiguration_SecurityConfigurationData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::SecurityConfiguration.SecurityConfigurationData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_SecurityConfiguration_SecurityConfigurationData as SecurityConfigurationData;
    impl crate::value::ToValue for SecurityConfigurationData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authentication_configuration {
                properties.insert(
                    "AuthenticationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.authorization_configuration {
                properties.insert(
                    "AuthorizationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_configuration {
                properties.insert(
                    "EncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-securityconfiguration-tlscertificateconfiguration.html
    pub struct TLSCertificateConfiguration_ {
        pub certificate_provider_type: Option<crate::value::ExpString>,
        pub private_key_secret_arn: Option<crate::value::ExpString>,
        pub public_key_secret_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_SecurityConfiguration_TLSCertificateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::SecurityConfiguration.TLSCertificateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_SecurityConfiguration_TLSCertificateConfiguration as TLSCertificateConfiguration;
    impl crate::value::ToValue for TLSCertificateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_provider_type {
                properties.insert(
                    "CertificateProviderType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.private_key_secret_arn {
                properties.insert(
                    "PrivateKeySecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.public_key_secret_arn {
                properties.insert(
                    "PublicKeySecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod virtualcluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-virtualcluster-containerinfo.html
    pub struct ContainerInfo_ {
        pub eks_info: Box<EksInfo_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_VirtualCluster_ContainerInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::VirtualCluster.ContainerInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_VirtualCluster_ContainerInfo as ContainerInfo;
    impl crate::value::ToValue for ContainerInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EksInfo".to_string(),
                crate::value::ToValue::to_value(&self.eks_info),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-virtualcluster-containerprovider.html
    pub struct ContainerProvider_ {
        pub id: crate::value::ExpString,
        pub info: Box<ContainerInfo_>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_VirtualCluster_ContainerProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::VirtualCluster.ContainerProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_VirtualCluster_ContainerProvider as ContainerProvider;
    impl crate::value::ToValue for ContainerProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Info".to_string(),
                crate::value::ToValue::to_value(&self.info),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-virtualcluster-eksinfo.html
    pub struct EksInfo_ {
        pub namespace: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_VirtualCluster_EksInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRContainers::VirtualCluster.EksInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_VirtualCluster_EksInfo as EksInfo;
    impl crate::value::ToValue for EksInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Namespace".to_string(),
                crate::value::ToValue::to_value(&self.namespace),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrcontainers-endpoint.html
pub struct Endpoint_ {
    pub configuration_overrides: Option<super::emrcontainers::endpoint::ConfigurationOverrides_>,
    pub execution_role_arn: crate::value::ExpString,
    pub name: Option<crate::value::ExpString>,
    pub release_label: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
    pub virtual_cluster_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_emrcontainers_Endpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EMRContainers::Endpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_emrcontainers_Endpoint as Endpoint;
impl crate::template::ToResource for Endpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EMRContainers"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Endpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.configuration_overrides {
            properties.insert(
                "ConfigurationOverrides".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ExecutionRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.execution_role_arn),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "ReleaseLabel".to_string(),
            crate::value::ToValue::to_value(&self.release_label),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties.insert(
            "VirtualClusterId".to_string(),
            crate::value::ToValue::to_value(&self.virtual_cluster_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrcontainers-securityconfiguration.html
pub struct SecurityConfiguration_ {
    pub container_provider: Option<super::emrcontainers::securityconfiguration::ContainerProvider_>,
    pub name: Option<crate::value::ExpString>,
    pub security_configuration_data:
        super::emrcontainers::securityconfiguration::SecurityConfigurationData_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_emrcontainers_SecurityConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EMRContainers::SecurityConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_emrcontainers_SecurityConfiguration as SecurityConfiguration;
impl crate::template::ToResource for SecurityConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EMRContainers"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SecurityConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.container_provider {
            properties.insert(
                "ContainerProvider".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "SecurityConfigurationData".to_string(),
            crate::value::ToValue::to_value(&self.security_configuration_data),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrcontainers-virtualcluster.html
pub struct VirtualCluster_ {
    pub container_provider: super::emrcontainers::virtualcluster::ContainerProvider_,
    pub name: crate::value::ExpString,
    pub security_configuration_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_emrcontainers_VirtualCluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EMRContainers::VirtualCluster"
        $($field $value)*)
    };
}
pub use crate::__aws_emrcontainers_VirtualCluster as VirtualCluster;
impl crate::template::ToResource for VirtualCluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EMRContainers"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VirtualCluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ContainerProvider".to_string(),
            crate::value::ToValue::to_value(&self.container_provider),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.security_configuration_id {
            properties.insert(
                "SecurityConfigurationId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
