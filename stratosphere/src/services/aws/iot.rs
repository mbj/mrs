pub mod accountauditconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfiguration.html
    pub struct AuditCheckConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_AccountAuditConfiguration_AuditCheckConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::AccountAuditConfiguration.AuditCheckConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_AccountAuditConfiguration_AuditCheckConfiguration as AuditCheckConfiguration;
    impl crate::value::ToValue for AuditCheckConfiguration_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html
    pub struct AuditCheckConfigurations_ {
        pub authenticated_cognito_role_overly_permissive_check:
            Option<Box<AuditCheckConfiguration_>>,
        pub ca_certificate_expiring_check: Option<Box<AuditCheckConfiguration_>>,
        pub ca_certificate_key_quality_check: Option<Box<AuditCheckConfiguration_>>,
        pub conflicting_client_ids_check: Option<Box<AuditCheckConfiguration_>>,
        pub device_certificate_age_check: Option<Box<DeviceCertAgeAuditCheckConfiguration_>>,
        pub device_certificate_expiring_check:
            Option<Box<DeviceCertExpirationAuditCheckConfiguration_>>,
        pub device_certificate_key_quality_check: Option<Box<AuditCheckConfiguration_>>,
        pub device_certificate_shared_check: Option<Box<AuditCheckConfiguration_>>,
        pub intermediate_ca_revoked_for_active_device_certificates_check:
            Option<Box<AuditCheckConfiguration_>>,
        pub io_t_policy_potential_mis_configuration_check: Option<Box<AuditCheckConfiguration_>>,
        pub iot_policy_overly_permissive_check: Option<Box<AuditCheckConfiguration_>>,
        pub iot_role_alias_allows_access_to_unused_services_check:
            Option<Box<AuditCheckConfiguration_>>,
        pub iot_role_alias_overly_permissive_check: Option<Box<AuditCheckConfiguration_>>,
        pub logging_disabled_check: Option<Box<AuditCheckConfiguration_>>,
        pub revoked_ca_certificate_still_active_check: Option<Box<AuditCheckConfiguration_>>,
        pub revoked_device_certificate_still_active_check: Option<Box<AuditCheckConfiguration_>>,
        pub unauthenticated_cognito_role_overly_permissive_check:
            Option<Box<AuditCheckConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_AccountAuditConfiguration_AuditCheckConfigurations {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::AccountAuditConfiguration.AuditCheckConfigurations"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_AccountAuditConfiguration_AuditCheckConfigurations as AuditCheckConfigurations;
    impl crate::value::ToValue for AuditCheckConfigurations_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authenticated_cognito_role_overly_permissive_check {
                properties.insert(
                    "AuthenticatedCognitoRoleOverlyPermissiveCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ca_certificate_expiring_check {
                properties.insert(
                    "CaCertificateExpiringCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ca_certificate_key_quality_check {
                properties.insert(
                    "CaCertificateKeyQualityCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.conflicting_client_ids_check {
                properties.insert(
                    "ConflictingClientIdsCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.device_certificate_age_check {
                properties.insert(
                    "DeviceCertificateAgeCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.device_certificate_expiring_check {
                properties.insert(
                    "DeviceCertificateExpiringCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.device_certificate_key_quality_check {
                properties.insert(
                    "DeviceCertificateKeyQualityCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.device_certificate_shared_check {
                properties.insert(
                    "DeviceCertificateSharedCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) =
                self.intermediate_ca_revoked_for_active_device_certificates_check
            {
                properties.insert(
                    "IntermediateCaRevokedForActiveDeviceCertificatesCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.io_t_policy_potential_mis_configuration_check {
                properties.insert(
                    "IoTPolicyPotentialMisConfigurationCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iot_policy_overly_permissive_check {
                properties.insert(
                    "IotPolicyOverlyPermissiveCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iot_role_alias_allows_access_to_unused_services_check {
                properties.insert(
                    "IotRoleAliasAllowsAccessToUnusedServicesCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iot_role_alias_overly_permissive_check {
                properties.insert(
                    "IotRoleAliasOverlyPermissiveCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logging_disabled_check {
                properties.insert(
                    "LoggingDisabledCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.revoked_ca_certificate_still_active_check {
                properties.insert(
                    "RevokedCaCertificateStillActiveCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.revoked_device_certificate_still_active_check {
                properties.insert(
                    "RevokedDeviceCertificateStillActiveCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unauthenticated_cognito_role_overly_permissive_check {
                properties.insert(
                    "UnauthenticatedCognitoRoleOverlyPermissiveCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditnotificationtarget.html
    pub struct AuditNotificationTarget_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub role_arn: Option<crate::value::ExpString>,
        pub target_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_AccountAuditConfiguration_AuditNotificationTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::AccountAuditConfiguration.AuditNotificationTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_AccountAuditConfiguration_AuditNotificationTarget as AuditNotificationTarget;
    impl crate::value::ToValue for AuditNotificationTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_arn {
                properties.insert(
                    "TargetArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditnotificationtargetconfigurations.html
    pub struct AuditNotificationTargetConfigurations_ {
        pub sns: Option<Box<AuditNotificationTarget_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_AccountAuditConfiguration_AuditNotificationTargetConfigurations {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::AccountAuditConfiguration.AuditNotificationTargetConfigurations"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_AccountAuditConfiguration_AuditNotificationTargetConfigurations as AuditNotificationTargetConfigurations;
    impl crate::value::ToValue for AuditNotificationTargetConfigurations_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sns {
                properties.insert("Sns".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-certagecheckcustomconfiguration.html
    pub struct CertAgeCheckCustomConfiguration_ {
        pub cert_age_threshold_in_days: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_AccountAuditConfiguration_CertAgeCheckCustomConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::AccountAuditConfiguration.CertAgeCheckCustomConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_AccountAuditConfiguration_CertAgeCheckCustomConfiguration as CertAgeCheckCustomConfiguration;
    impl crate::value::ToValue for CertAgeCheckCustomConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cert_age_threshold_in_days {
                properties.insert(
                    "CertAgeThresholdInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-certexpirationcheckcustomconfiguration.html
    pub struct CertExpirationCheckCustomConfiguration_ {
        pub cert_expiration_threshold_in_days: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_AccountAuditConfiguration_CertExpirationCheckCustomConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::AccountAuditConfiguration.CertExpirationCheckCustomConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_AccountAuditConfiguration_CertExpirationCheckCustomConfiguration as CertExpirationCheckCustomConfiguration;
    impl crate::value::ToValue for CertExpirationCheckCustomConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cert_expiration_threshold_in_days {
                properties.insert(
                    "CertExpirationThresholdInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-devicecertageauditcheckconfiguration.html
    pub struct DeviceCertAgeAuditCheckConfiguration_ {
        pub configuration: Option<Box<CertAgeCheckCustomConfiguration_>>,
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_AccountAuditConfiguration_DeviceCertAgeAuditCheckConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::AccountAuditConfiguration.DeviceCertAgeAuditCheckConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_AccountAuditConfiguration_DeviceCertAgeAuditCheckConfiguration as DeviceCertAgeAuditCheckConfiguration;
    impl crate::value::ToValue for DeviceCertAgeAuditCheckConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configuration {
                properties.insert(
                    "Configuration".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-devicecertexpirationauditcheckconfiguration.html
    pub struct DeviceCertExpirationAuditCheckConfiguration_ {
        pub configuration: Option<Box<CertExpirationCheckCustomConfiguration_>>,
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_AccountAuditConfiguration_DeviceCertExpirationAuditCheckConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::AccountAuditConfiguration.DeviceCertExpirationAuditCheckConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_AccountAuditConfiguration_DeviceCertExpirationAuditCheckConfiguration as DeviceCertExpirationAuditCheckConfiguration;
    impl crate::value::ToValue for DeviceCertExpirationAuditCheckConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configuration {
                properties.insert(
                    "Configuration".to_string(),
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
}
pub mod billinggroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-billinggroup-billinggroupproperties.html
    pub struct BillingGroupProperties_ {
        pub billing_group_description: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_BillingGroup_BillingGroupProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::BillingGroup.BillingGroupProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_BillingGroup_BillingGroupProperties as BillingGroupProperties;
    impl crate::value::ToValue for BillingGroupProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.billing_group_description {
                properties.insert(
                    "BillingGroupDescription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod cacertificate {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-cacertificate-registrationconfig.html
    pub struct RegistrationConfig_ {
        pub role_arn: Option<crate::value::ExpString>,
        pub template_body: Option<crate::value::ExpString>,
        pub template_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_CACertificate_RegistrationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::CACertificate.RegistrationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_CACertificate_RegistrationConfig as RegistrationConfig;
    impl crate::value::ToValue for RegistrationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.template_body {
                properties.insert(
                    "TemplateBody".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.template_name {
                properties.insert(
                    "TemplateName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod command {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-command-commandparameter.html
    pub struct CommandParameter_ {
        pub default_value: Option<Box<CommandParameterValue_>>,
        pub description: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub value: Option<Box<CommandParameterValue_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_Command_CommandParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::Command.CommandParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_Command_CommandParameter as CommandParameter;
    impl crate::value::ToValue for CommandParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_value {
                properties.insert(
                    "DefaultValue".to_string(),
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
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-command-commandparametervalue.html
    pub struct CommandParameterValue_ {
        pub b: Option<crate::value::ExpBool>,
        pub bin: Option<crate::value::ExpString>,
        pub d: Option<f64>,
        pub i: Option<i64>,
        pub l: Option<crate::value::ExpString>,
        pub s: Option<crate::value::ExpString>,
        pub ul: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_Command_CommandParameterValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::Command.CommandParameterValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_Command_CommandParameterValue as CommandParameterValue;
    impl crate::value::ToValue for CommandParameterValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.b {
                properties.insert("B".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.bin {
                properties.insert("BIN".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.d {
                properties.insert("D".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.i {
                properties.insert("I".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.l {
                properties.insert("L".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.s {
                properties.insert("S".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.ul {
                properties.insert("UL".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-command-commandpayload.html
    pub struct CommandPayload_ {
        pub content: Option<crate::value::ExpString>,
        pub content_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_Command_CommandPayload {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::Command.CommandPayload"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_Command_CommandPayload as CommandPayload;
    impl crate::value::ToValue for CommandPayload_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content {
                properties.insert(
                    "Content".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.content_type {
                properties.insert(
                    "ContentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod domainconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-domainconfiguration-authorizerconfig.html
    pub struct AuthorizerConfig_ {
        pub allow_authorizer_override: Option<crate::value::ExpBool>,
        pub default_authorizer_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_DomainConfiguration_AuthorizerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::DomainConfiguration.AuthorizerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_DomainConfiguration_AuthorizerConfig as AuthorizerConfig;
    impl crate::value::ToValue for AuthorizerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_authorizer_override {
                properties.insert(
                    "AllowAuthorizerOverride".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_authorizer_name {
                properties.insert(
                    "DefaultAuthorizerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-domainconfiguration-clientcertificateconfig.html
    pub struct ClientCertificateConfig_ {
        pub client_certificate_callback_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_DomainConfiguration_ClientCertificateConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::DomainConfiguration.ClientCertificateConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_DomainConfiguration_ClientCertificateConfig as ClientCertificateConfig;
    impl crate::value::ToValue for ClientCertificateConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.client_certificate_callback_arn {
                properties.insert(
                    "ClientCertificateCallbackArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-domainconfiguration-servercertificateconfig.html
    pub struct ServerCertificateConfig_ {
        pub enable_ocsp_check: Option<crate::value::ExpBool>,
        pub ocsp_authorized_responder_arn: Option<crate::value::ExpString>,
        pub ocsp_lambda_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_DomainConfiguration_ServerCertificateConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::DomainConfiguration.ServerCertificateConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_DomainConfiguration_ServerCertificateConfig as ServerCertificateConfig;
    impl crate::value::ToValue for ServerCertificateConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_ocsp_check {
                properties.insert(
                    "EnableOCSPCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ocsp_authorized_responder_arn {
                properties.insert(
                    "OcspAuthorizedResponderArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ocsp_lambda_arn {
                properties.insert(
                    "OcspLambdaArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-domainconfiguration-servercertificatesummary.html
    pub struct ServerCertificateSummary_ {
        pub server_certificate_arn: Option<crate::value::ExpString>,
        pub server_certificate_status: Option<crate::value::ExpString>,
        pub server_certificate_status_detail: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_DomainConfiguration_ServerCertificateSummary {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::DomainConfiguration.ServerCertificateSummary"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_DomainConfiguration_ServerCertificateSummary as ServerCertificateSummary;
    impl crate::value::ToValue for ServerCertificateSummary_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.server_certificate_arn {
                properties.insert(
                    "ServerCertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.server_certificate_status {
                properties.insert(
                    "ServerCertificateStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.server_certificate_status_detail {
                properties.insert(
                    "ServerCertificateStatusDetail".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-domainconfiguration-tlsconfig.html
    pub struct TlsConfig_ {
        pub security_policy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_DomainConfiguration_TlsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::DomainConfiguration.TlsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_DomainConfiguration_TlsConfig as TlsConfig;
    impl crate::value::ToValue for TlsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.security_policy {
                properties.insert(
                    "SecurityPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod encryptionconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-encryptionconfiguration-configurationdetails.html
    pub struct ConfigurationDetails_ {
        pub configuration_status: Option<crate::value::ExpString>,
        pub error_code: Option<crate::value::ExpString>,
        pub error_message: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_EncryptionConfiguration_ConfigurationDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::EncryptionConfiguration.ConfigurationDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_EncryptionConfiguration_ConfigurationDetails as ConfigurationDetails;
    impl crate::value::ToValue for ConfigurationDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configuration_status {
                properties.insert(
                    "ConfigurationStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.error_code {
                properties.insert(
                    "ErrorCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.error_message {
                properties.insert(
                    "ErrorMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod fleetmetric {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-fleetmetric-aggregationtype.html
    pub struct AggregationType_ {
        pub name: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_FleetMetric_AggregationType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::FleetMetric.AggregationType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_FleetMetric_AggregationType as AggregationType;
    impl crate::value::ToValue for AggregationType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
}
pub mod jobtemplate {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-abortconfig.html
    pub struct AbortConfig_ {
        pub criteria_list: Vec<AbortCriteria_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_JobTemplate_AbortConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::JobTemplate.AbortConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_JobTemplate_AbortConfig as AbortConfig;
    impl crate::value::ToValue for AbortConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CriteriaList".to_string(),
                crate::value::ToValue::to_value(&self.criteria_list),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-abortcriteria.html
    pub struct AbortCriteria_ {
        pub action: crate::value::ExpString,
        pub failure_type: crate::value::ExpString,
        pub min_number_of_executed_things: i64,
        pub threshold_percentage: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_JobTemplate_AbortCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::JobTemplate.AbortCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_JobTemplate_AbortCriteria as AbortCriteria;
    impl crate::value::ToValue for AbortCriteria_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-exponentialrolloutrate.html
    pub struct ExponentialRolloutRate_ {
        pub base_rate_per_minute: i64,
        pub increment_factor: f64,
        pub rate_increase_criteria: Box<RateIncreaseCriteria_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_JobTemplate_ExponentialRolloutRate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::JobTemplate.ExponentialRolloutRate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_JobTemplate_ExponentialRolloutRate as ExponentialRolloutRate;
    impl crate::value::ToValue for ExponentialRolloutRate_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-jobexecutionsretryconfig.html
    pub struct JobExecutionsRetryConfig_ {
        pub retry_criteria_list: Option<Vec<RetryCriteria_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_JobTemplate_JobExecutionsRetryConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::JobTemplate.JobExecutionsRetryConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_JobTemplate_JobExecutionsRetryConfig as JobExecutionsRetryConfig;
    impl crate::value::ToValue for JobExecutionsRetryConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.retry_criteria_list {
                properties.insert(
                    "RetryCriteriaList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-jobexecutionsrolloutconfig.html
    pub struct JobExecutionsRolloutConfig_ {
        pub exponential_rollout_rate: Option<Box<ExponentialRolloutRate_>>,
        pub maximum_per_minute: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_JobTemplate_JobExecutionsRolloutConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::JobTemplate.JobExecutionsRolloutConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_JobTemplate_JobExecutionsRolloutConfig as JobExecutionsRolloutConfig;
    impl crate::value::ToValue for JobExecutionsRolloutConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exponential_rollout_rate {
                properties.insert(
                    "ExponentialRolloutRate".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-maintenancewindow.html
    pub struct MaintenanceWindow_ {
        pub duration_in_minutes: Option<i64>,
        pub start_time: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_JobTemplate_MaintenanceWindow {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::JobTemplate.MaintenanceWindow"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_JobTemplate_MaintenanceWindow as MaintenanceWindow;
    impl crate::value::ToValue for MaintenanceWindow_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.duration_in_minutes {
                properties.insert(
                    "DurationInMinutes".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-presignedurlconfig.html
    pub struct PresignedUrlConfig_ {
        pub expires_in_sec: Option<i64>,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_JobTemplate_PresignedUrlConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::JobTemplate.PresignedUrlConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_JobTemplate_PresignedUrlConfig as PresignedUrlConfig;
    impl crate::value::ToValue for PresignedUrlConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.expires_in_sec {
                properties.insert(
                    "ExpiresInSec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-rateincreasecriteria.html
    pub struct RateIncreaseCriteria_ {
        pub number_of_notified_things: Option<i64>,
        pub number_of_succeeded_things: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_JobTemplate_RateIncreaseCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::JobTemplate.RateIncreaseCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_JobTemplate_RateIncreaseCriteria as RateIncreaseCriteria;
    impl crate::value::ToValue for RateIncreaseCriteria_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-retrycriteria.html
    pub struct RetryCriteria_ {
        pub failure_type: Option<crate::value::ExpString>,
        pub number_of_retries: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_JobTemplate_RetryCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::JobTemplate.RetryCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_JobTemplate_RetryCriteria as RetryCriteria;
    impl crate::value::ToValue for RetryCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.failure_type {
                properties.insert(
                    "FailureType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_of_retries {
                properties.insert(
                    "NumberOfRetries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-timeoutconfig.html
    pub struct TimeoutConfig_ {
        pub in_progress_timeout_in_minutes: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_JobTemplate_TimeoutConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::JobTemplate.TimeoutConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_JobTemplate_TimeoutConfig as TimeoutConfig;
    impl crate::value::ToValue for TimeoutConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InProgressTimeoutInMinutes".to_string(),
                crate::value::ToValue::to_value(&self.in_progress_timeout_in_minutes),
            );
            properties.into()
        }
    }
}
pub mod mitigationaction {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-actionparams.html
    pub struct ActionParams_ {
        pub add_things_to_thing_group_params: Option<Box<AddThingsToThingGroupParams_>>,
        pub enable_io_t_logging_params: Option<Box<EnableIoTLoggingParams_>>,
        pub publish_finding_to_sns_params: Option<Box<PublishFindingToSnsParams_>>,
        pub replace_default_policy_version_params: Option<Box<ReplaceDefaultPolicyVersionParams_>>,
        pub update_ca_certificate_params: Option<Box<UpdateCACertificateParams_>>,
        pub update_device_certificate_params: Option<Box<UpdateDeviceCertificateParams_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_MitigationAction_ActionParams {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::MitigationAction.ActionParams"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_MitigationAction_ActionParams as ActionParams;
    impl crate::value::ToValue for ActionParams_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.add_things_to_thing_group_params {
                properties.insert(
                    "AddThingsToThingGroupParams".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_io_t_logging_params {
                properties.insert(
                    "EnableIoTLoggingParams".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.publish_finding_to_sns_params {
                properties.insert(
                    "PublishFindingToSnsParams".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.replace_default_policy_version_params {
                properties.insert(
                    "ReplaceDefaultPolicyVersionParams".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.update_ca_certificate_params {
                properties.insert(
                    "UpdateCACertificateParams".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.update_device_certificate_params {
                properties.insert(
                    "UpdateDeviceCertificateParams".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-addthingstothinggroupparams.html
    pub struct AddThingsToThingGroupParams_ {
        pub override_dynamic_groups: Option<crate::value::ExpBool>,
        pub thing_group_names: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_MitigationAction_AddThingsToThingGroupParams {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::MitigationAction.AddThingsToThingGroupParams"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_MitigationAction_AddThingsToThingGroupParams as AddThingsToThingGroupParams;
    impl crate::value::ToValue for AddThingsToThingGroupParams_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.override_dynamic_groups {
                properties.insert(
                    "OverrideDynamicGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ThingGroupNames".to_string(),
                crate::value::ToValue::to_value(&self.thing_group_names),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-enableiotloggingparams.html
    pub struct EnableIoTLoggingParams_ {
        pub log_level: crate::value::ExpString,
        pub role_arn_for_logging: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_MitigationAction_EnableIoTLoggingParams {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::MitigationAction.EnableIoTLoggingParams"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_MitigationAction_EnableIoTLoggingParams as EnableIoTLoggingParams;
    impl crate::value::ToValue for EnableIoTLoggingParams_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LogLevel".to_string(),
                crate::value::ToValue::to_value(&self.log_level),
            );
            properties.insert(
                "RoleArnForLogging".to_string(),
                crate::value::ToValue::to_value(&self.role_arn_for_logging),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-publishfindingtosnsparams.html
    pub struct PublishFindingToSnsParams_ {
        pub topic_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_MitigationAction_PublishFindingToSnsParams {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::MitigationAction.PublishFindingToSnsParams"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_MitigationAction_PublishFindingToSnsParams as PublishFindingToSnsParams;
    impl crate::value::ToValue for PublishFindingToSnsParams_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TopicArn".to_string(),
                crate::value::ToValue::to_value(&self.topic_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-replacedefaultpolicyversionparams.html
    pub struct ReplaceDefaultPolicyVersionParams_ {
        pub template_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_MitigationAction_ReplaceDefaultPolicyVersionParams {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::MitigationAction.ReplaceDefaultPolicyVersionParams"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_MitigationAction_ReplaceDefaultPolicyVersionParams as ReplaceDefaultPolicyVersionParams;
    impl crate::value::ToValue for ReplaceDefaultPolicyVersionParams_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TemplateName".to_string(),
                crate::value::ToValue::to_value(&self.template_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-updatecacertificateparams.html
    pub struct UpdateCACertificateParams_ {
        pub action: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_MitigationAction_UpdateCACertificateParams {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::MitigationAction.UpdateCACertificateParams"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_MitigationAction_UpdateCACertificateParams as UpdateCACertificateParams;
    impl crate::value::ToValue for UpdateCACertificateParams_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-updatedevicecertificateparams.html
    pub struct UpdateDeviceCertificateParams_ {
        pub action: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_MitigationAction_UpdateDeviceCertificateParams {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::MitigationAction.UpdateDeviceCertificateParams"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_MitigationAction_UpdateDeviceCertificateParams as UpdateDeviceCertificateParams;
    impl crate::value::ToValue for UpdateDeviceCertificateParams_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.into()
        }
    }
}
pub mod provisioningtemplate {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-provisioningtemplate-provisioninghook.html
    pub struct ProvisioningHook_ {
        pub payload_version: Option<crate::value::ExpString>,
        pub target_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_ProvisioningTemplate_ProvisioningHook {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::ProvisioningTemplate.ProvisioningHook"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_ProvisioningTemplate_ProvisioningHook as ProvisioningHook;
    impl crate::value::ToValue for ProvisioningHook_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.payload_version {
                properties.insert(
                    "PayloadVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_arn {
                properties.insert(
                    "TargetArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod securityprofile {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-alerttarget.html
    pub struct AlertTarget_ {
        pub alert_target_arn: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_SecurityProfile_AlertTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::SecurityProfile.AlertTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_SecurityProfile_AlertTarget as AlertTarget;
    impl crate::value::ToValue for AlertTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AlertTargetArn".to_string(),
                crate::value::ToValue::to_value(&self.alert_target_arn),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behavior.html
    pub struct Behavior_ {
        pub criteria: Option<Box<BehaviorCriteria_>>,
        pub export_metric: Option<crate::value::ExpBool>,
        pub metric: Option<crate::value::ExpString>,
        pub metric_dimension: Option<Box<MetricDimension_>>,
        pub name: crate::value::ExpString,
        pub suppress_alerts: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_SecurityProfile_Behavior {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::SecurityProfile.Behavior"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_SecurityProfile_Behavior as Behavior;
    impl crate::value::ToValue for Behavior_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.criteria {
                properties.insert(
                    "Criteria".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.export_metric {
                properties.insert(
                    "ExportMetric".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric {
                properties.insert("Metric".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.metric_dimension {
                properties.insert(
                    "MetricDimension".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.suppress_alerts {
                properties.insert(
                    "SuppressAlerts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behaviorcriteria.html
    pub struct BehaviorCriteria_ {
        pub comparison_operator: Option<crate::value::ExpString>,
        pub consecutive_datapoints_to_alarm: Option<i64>,
        pub consecutive_datapoints_to_clear: Option<i64>,
        pub duration_seconds: Option<i64>,
        pub ml_detection_config: Option<Box<MachineLearningDetectionConfig_>>,
        pub statistical_threshold: Option<Box<StatisticalThreshold_>>,
        pub value: Option<Box<MetricValue_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_SecurityProfile_BehaviorCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::SecurityProfile.BehaviorCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_SecurityProfile_BehaviorCriteria as BehaviorCriteria;
    impl crate::value::ToValue for BehaviorCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.comparison_operator {
                properties.insert(
                    "ComparisonOperator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.consecutive_datapoints_to_alarm {
                properties.insert(
                    "ConsecutiveDatapointsToAlarm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.consecutive_datapoints_to_clear {
                properties.insert(
                    "ConsecutiveDatapointsToClear".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.duration_seconds {
                properties.insert(
                    "DurationSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ml_detection_config {
                properties.insert(
                    "MlDetectionConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.statistical_threshold {
                properties.insert(
                    "StatisticalThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-machinelearningdetectionconfig.html
    pub struct MachineLearningDetectionConfig_ {
        pub confidence_level: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_SecurityProfile_MachineLearningDetectionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::SecurityProfile.MachineLearningDetectionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_SecurityProfile_MachineLearningDetectionConfig as MachineLearningDetectionConfig;
    impl crate::value::ToValue for MachineLearningDetectionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.confidence_level {
                properties.insert(
                    "ConfidenceLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metricdimension.html
    pub struct MetricDimension_ {
        pub dimension_name: crate::value::ExpString,
        pub operator: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_SecurityProfile_MetricDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::SecurityProfile.MetricDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_SecurityProfile_MetricDimension as MetricDimension;
    impl crate::value::ToValue for MetricDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DimensionName".to_string(),
                crate::value::ToValue::to_value(&self.dimension_name),
            );
            if let Some(ref value) = self.operator {
                properties.insert(
                    "Operator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metrictoretain.html
    pub struct MetricToRetain_ {
        pub export_metric: Option<crate::value::ExpBool>,
        pub metric: crate::value::ExpString,
        pub metric_dimension: Option<Box<MetricDimension_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_SecurityProfile_MetricToRetain {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::SecurityProfile.MetricToRetain"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_SecurityProfile_MetricToRetain as MetricToRetain;
    impl crate::value::ToValue for MetricToRetain_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.export_metric {
                properties.insert(
                    "ExportMetric".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Metric".to_string(),
                crate::value::ToValue::to_value(&self.metric),
            );
            if let Some(ref value) = self.metric_dimension {
                properties.insert(
                    "MetricDimension".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metricvalue.html
    pub struct MetricValue_ {
        pub cidrs: Option<Vec<crate::value::ExpString>>,
        pub count: Option<crate::value::ExpString>,
        pub number: Option<f64>,
        pub numbers: Option<Vec<f64>>,
        pub ports: Option<Vec<i64>>,
        pub strings: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_SecurityProfile_MetricValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::SecurityProfile.MetricValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_SecurityProfile_MetricValue as MetricValue;
    impl crate::value::ToValue for MetricValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cidrs {
                properties.insert("Cidrs".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.count {
                properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.number {
                properties.insert("Number".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.numbers {
                properties.insert(
                    "Numbers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ports {
                properties.insert("Ports".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.strings {
                properties.insert(
                    "Strings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metricsexportconfig.html
    pub struct MetricsExportConfig_ {
        pub mqtt_topic: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_SecurityProfile_MetricsExportConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::SecurityProfile.MetricsExportConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_SecurityProfile_MetricsExportConfig as MetricsExportConfig;
    impl crate::value::ToValue for MetricsExportConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MqttTopic".to_string(),
                crate::value::ToValue::to_value(&self.mqtt_topic),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-statisticalthreshold.html
    pub struct StatisticalThreshold_ {
        pub statistic: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_SecurityProfile_StatisticalThreshold {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::SecurityProfile.StatisticalThreshold"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_SecurityProfile_StatisticalThreshold as StatisticalThreshold;
    impl crate::value::ToValue for StatisticalThreshold_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.statistic {
                properties.insert(
                    "Statistic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod softwarepackageversion {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-softwarepackageversion-packageversionartifact.html
    pub struct PackageVersionArtifact_ {
        pub s3_location: Box<S3Location_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_SoftwarePackageVersion_PackageVersionArtifact {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::SoftwarePackageVersion.PackageVersionArtifact"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_SoftwarePackageVersion_PackageVersionArtifact as PackageVersionArtifact;
    impl crate::value::ToValue for PackageVersionArtifact_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Location".to_string(),
                crate::value::ToValue::to_value(&self.s3_location),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-softwarepackageversion-s3location.html
    pub struct S3Location_ {
        pub bucket: crate::value::ExpString,
        pub key: crate::value::ExpString,
        pub version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_SoftwarePackageVersion_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::SoftwarePackageVersion.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_SoftwarePackageVersion_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Version".to_string(),
                crate::value::ToValue::to_value(&self.version),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-softwarepackageversion-sbom.html
    pub struct Sbom_ {
        pub s3_location: Box<S3Location_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_SoftwarePackageVersion_Sbom {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::SoftwarePackageVersion.Sbom"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_SoftwarePackageVersion_Sbom as Sbom;
    impl crate::value::ToValue for Sbom_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Location".to_string(),
                crate::value::ToValue::to_value(&self.s3_location),
            );
            properties.into()
        }
    }
}
pub mod thing {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thing-attributepayload.html
    pub struct AttributePayload_ {
        pub attributes: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_Thing_AttributePayload {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::Thing.AttributePayload"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_Thing_AttributePayload as AttributePayload;
    impl crate::value::ToValue for AttributePayload_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attributes {
                properties.insert(
                    "Attributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod thinggroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thinggroup-attributepayload.html
    pub struct AttributePayload_ {
        pub attributes: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_ThingGroup_AttributePayload {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::ThingGroup.AttributePayload"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_ThingGroup_AttributePayload as AttributePayload;
    impl crate::value::ToValue for AttributePayload_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attributes {
                properties.insert(
                    "Attributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thinggroup-thinggroupproperties.html
    pub struct ThingGroupProperties_ {
        pub attribute_payload: Option<Box<AttributePayload_>>,
        pub thing_group_description: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_ThingGroup_ThingGroupProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::ThingGroup.ThingGroupProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_ThingGroup_ThingGroupProperties as ThingGroupProperties;
    impl crate::value::ToValue for ThingGroupProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attribute_payload {
                properties.insert(
                    "AttributePayload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.thing_group_description {
                properties.insert(
                    "ThingGroupDescription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod thingtype {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thingtype-mqtt5configuration.html
    pub struct Mqtt5Configuration_ {
        pub propagating_attributes: Option<Vec<PropagatingAttribute_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_ThingType_Mqtt5Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::ThingType.Mqtt5Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_ThingType_Mqtt5Configuration as Mqtt5Configuration;
    impl crate::value::ToValue for Mqtt5Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.propagating_attributes {
                properties.insert(
                    "PropagatingAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thingtype-propagatingattribute.html
    pub struct PropagatingAttribute_ {
        pub connection_attribute: Option<crate::value::ExpString>,
        pub thing_attribute: Option<crate::value::ExpString>,
        pub user_property_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_ThingType_PropagatingAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::ThingType.PropagatingAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_ThingType_PropagatingAttribute as PropagatingAttribute;
    impl crate::value::ToValue for PropagatingAttribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_attribute {
                properties.insert(
                    "ConnectionAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.thing_attribute {
                properties.insert(
                    "ThingAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "UserPropertyKey".to_string(),
                crate::value::ToValue::to_value(&self.user_property_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thingtype-thingtypeproperties.html
    pub struct ThingTypeProperties_ {
        pub mqtt5_configuration: Option<Box<Mqtt5Configuration_>>,
        pub searchable_attributes: Option<Vec<crate::value::ExpString>>,
        pub thing_type_description: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_ThingType_ThingTypeProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::ThingType.ThingTypeProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_ThingType_ThingTypeProperties as ThingTypeProperties;
    impl crate::value::ToValue for ThingTypeProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mqtt5_configuration {
                properties.insert(
                    "Mqtt5Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.searchable_attributes {
                properties.insert(
                    "SearchableAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.thing_type_description {
                properties.insert(
                    "ThingTypeDescription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod topicrule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html
    pub struct Action_ {
        pub cloudwatch_alarm: Option<Box<CloudwatchAlarmAction_>>,
        pub cloudwatch_logs: Option<Box<CloudwatchLogsAction_>>,
        pub cloudwatch_metric: Option<Box<CloudwatchMetricAction_>>,
        pub dynamo_db: Option<Box<DynamoDBAction_>>,
        pub dynamo_d_bv2: Option<Box<DynamoDBv2Action_>>,
        pub elasticsearch: Option<Box<ElasticsearchAction_>>,
        pub firehose: Option<Box<FirehoseAction_>>,
        pub http: Option<Box<HttpAction_>>,
        pub iot_analytics: Option<Box<IotAnalyticsAction_>>,
        pub iot_events: Option<Box<IotEventsAction_>>,
        pub iot_site_wise: Option<Box<IotSiteWiseAction_>>,
        pub kafka: Option<Box<KafkaAction_>>,
        pub kinesis: Option<Box<KinesisAction_>>,
        pub lambda: Option<Box<LambdaAction_>>,
        pub location: Option<Box<LocationAction_>>,
        pub open_search: Option<Box<OpenSearchAction_>>,
        pub republish: Option<Box<RepublishAction_>>,
        pub s3: Option<Box<S3Action_>>,
        pub sns: Option<Box<SnsAction_>>,
        pub sqs: Option<Box<SqsAction_>>,
        pub step_functions: Option<Box<StepFunctionsAction_>>,
        pub timestream: Option<Box<TimestreamAction_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloudwatch_alarm {
                properties.insert(
                    "CloudwatchAlarm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloudwatch_logs {
                properties.insert(
                    "CloudwatchLogs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloudwatch_metric {
                properties.insert(
                    "CloudwatchMetric".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dynamo_db {
                properties.insert(
                    "DynamoDB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dynamo_d_bv2 {
                properties.insert(
                    "DynamoDBv2".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.elasticsearch {
                properties.insert(
                    "Elasticsearch".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.firehose {
                properties.insert(
                    "Firehose".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http {
                properties.insert("Http".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.iot_analytics {
                properties.insert(
                    "IotAnalytics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iot_events {
                properties.insert(
                    "IotEvents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iot_site_wise {
                properties.insert(
                    "IotSiteWise".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kafka {
                properties.insert("Kafka".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.kinesis {
                properties.insert(
                    "Kinesis".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda {
                properties.insert("Lambda".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.location {
                properties.insert(
                    "Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.open_search {
                properties.insert(
                    "OpenSearch".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.republish {
                properties.insert(
                    "Republish".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sns {
                properties.insert("Sns".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sqs {
                properties.insert("Sqs".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.step_functions {
                properties.insert(
                    "StepFunctions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timestream {
                properties.insert(
                    "Timestream".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-assetpropertytimestamp.html
    pub struct AssetPropertyTimestamp_ {
        pub offset_in_nanos: Option<crate::value::ExpString>,
        pub time_in_seconds: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_AssetPropertyTimestamp {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.AssetPropertyTimestamp"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_AssetPropertyTimestamp as AssetPropertyTimestamp;
    impl crate::value::ToValue for AssetPropertyTimestamp_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.offset_in_nanos {
                properties.insert(
                    "OffsetInNanos".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TimeInSeconds".to_string(),
                crate::value::ToValue::to_value(&self.time_in_seconds),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-assetpropertyvalue.html
    pub struct AssetPropertyValue_ {
        pub quality: Option<crate::value::ExpString>,
        pub timestamp: Box<AssetPropertyTimestamp_>,
        pub value: Box<AssetPropertyVariant_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_AssetPropertyValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.AssetPropertyValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_AssetPropertyValue as AssetPropertyValue;
    impl crate::value::ToValue for AssetPropertyValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.quality {
                properties.insert(
                    "Quality".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Timestamp".to_string(),
                crate::value::ToValue::to_value(&self.timestamp),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-assetpropertyvariant.html
    pub struct AssetPropertyVariant_ {
        pub boolean_value: Option<crate::value::ExpString>,
        pub double_value: Option<crate::value::ExpString>,
        pub integer_value: Option<crate::value::ExpString>,
        pub string_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_AssetPropertyVariant {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.AssetPropertyVariant"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_AssetPropertyVariant as AssetPropertyVariant;
    impl crate::value::ToValue for AssetPropertyVariant_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.boolean_value {
                properties.insert(
                    "BooleanValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.double_value {
                properties.insert(
                    "DoubleValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.integer_value {
                properties.insert(
                    "IntegerValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string_value {
                properties.insert(
                    "StringValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchalarmaction.html
    pub struct CloudwatchAlarmAction_ {
        pub alarm_name: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
        pub state_reason: crate::value::ExpString,
        pub state_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_CloudwatchAlarmAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.CloudwatchAlarmAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_CloudwatchAlarmAction as CloudwatchAlarmAction;
    impl crate::value::ToValue for CloudwatchAlarmAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AlarmName".to_string(),
                crate::value::ToValue::to_value(&self.alarm_name),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "StateReason".to_string(),
                crate::value::ToValue::to_value(&self.state_reason),
            );
            properties.insert(
                "StateValue".to_string(),
                crate::value::ToValue::to_value(&self.state_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchlogsaction.html
    pub struct CloudwatchLogsAction_ {
        pub batch_mode: Option<crate::value::ExpBool>,
        pub log_group_name: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_CloudwatchLogsAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.CloudwatchLogsAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_CloudwatchLogsAction as CloudwatchLogsAction;
    impl crate::value::ToValue for CloudwatchLogsAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_mode {
                properties.insert(
                    "BatchMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LogGroupName".to_string(),
                crate::value::ToValue::to_value(&self.log_group_name),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html
    pub struct CloudwatchMetricAction_ {
        pub metric_name: crate::value::ExpString,
        pub metric_namespace: crate::value::ExpString,
        pub metric_timestamp: Option<crate::value::ExpString>,
        pub metric_unit: crate::value::ExpString,
        pub metric_value: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_CloudwatchMetricAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.CloudwatchMetricAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_CloudwatchMetricAction as CloudwatchMetricAction;
    impl crate::value::ToValue for CloudwatchMetricAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MetricName".to_string(),
                crate::value::ToValue::to_value(&self.metric_name),
            );
            properties.insert(
                "MetricNamespace".to_string(),
                crate::value::ToValue::to_value(&self.metric_namespace),
            );
            if let Some(ref value) = self.metric_timestamp {
                properties.insert(
                    "MetricTimestamp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MetricUnit".to_string(),
                crate::value::ToValue::to_value(&self.metric_unit),
            );
            properties.insert(
                "MetricValue".to_string(),
                crate::value::ToValue::to_value(&self.metric_value),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html
    pub struct DynamoDBAction_ {
        pub hash_key_field: crate::value::ExpString,
        pub hash_key_type: Option<crate::value::ExpString>,
        pub hash_key_value: crate::value::ExpString,
        pub payload_field: Option<crate::value::ExpString>,
        pub range_key_field: Option<crate::value::ExpString>,
        pub range_key_type: Option<crate::value::ExpString>,
        pub range_key_value: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
        pub table_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_DynamoDBAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.DynamoDBAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_DynamoDBAction as DynamoDBAction;
    impl crate::value::ToValue for DynamoDBAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HashKeyField".to_string(),
                crate::value::ToValue::to_value(&self.hash_key_field),
            );
            if let Some(ref value) = self.hash_key_type {
                properties.insert(
                    "HashKeyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "HashKeyValue".to_string(),
                crate::value::ToValue::to_value(&self.hash_key_value),
            );
            if let Some(ref value) = self.payload_field {
                properties.insert(
                    "PayloadField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.range_key_field {
                properties.insert(
                    "RangeKeyField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.range_key_type {
                properties.insert(
                    "RangeKeyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.range_key_value {
                properties.insert(
                    "RangeKeyValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbv2action.html
    pub struct DynamoDBv2Action_ {
        pub put_item: Option<Box<PutItemInput_>>,
        pub role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_DynamoDBv2Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.DynamoDBv2Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_DynamoDBv2Action as DynamoDBv2Action;
    impl crate::value::ToValue for DynamoDBv2Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.put_item {
                properties.insert(
                    "PutItem".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-elasticsearchaction.html
    pub struct ElasticsearchAction_ {
        pub endpoint: crate::value::ExpString,
        pub id: crate::value::ExpString,
        pub index: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_ElasticsearchAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.ElasticsearchAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_ElasticsearchAction as ElasticsearchAction;
    impl crate::value::ToValue for ElasticsearchAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(&self.endpoint),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Index".to_string(),
                crate::value::ToValue::to_value(&self.index),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-firehoseaction.html
    pub struct FirehoseAction_ {
        pub batch_mode: Option<crate::value::ExpBool>,
        pub delivery_stream_name: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
        pub separator: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_FirehoseAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.FirehoseAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_FirehoseAction as FirehoseAction;
    impl crate::value::ToValue for FirehoseAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_mode {
                properties.insert(
                    "BatchMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DeliveryStreamName".to_string(),
                crate::value::ToValue::to_value(&self.delivery_stream_name),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.separator {
                properties.insert(
                    "Separator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-httpaction.html
    pub struct HttpAction_ {
        pub auth: Option<Box<HttpAuthorization_>>,
        pub confirmation_url: Option<crate::value::ExpString>,
        pub headers: Option<Vec<HttpActionHeader_>>,
        pub url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_HttpAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.HttpAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_HttpAction as HttpAction;
    impl crate::value::ToValue for HttpAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auth {
                properties.insert("Auth".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.confirmation_url {
                properties.insert(
                    "ConfirmationUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.headers {
                properties.insert(
                    "Headers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Url".to_string(),
                crate::value::ToValue::to_value(&self.url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-httpactionheader.html
    pub struct HttpActionHeader_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_HttpActionHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.HttpActionHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_HttpActionHeader as HttpActionHeader;
    impl crate::value::ToValue for HttpActionHeader_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-httpauthorization.html
    pub struct HttpAuthorization_ {
        pub sigv4: Option<Box<SigV4Authorization_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_HttpAuthorization {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.HttpAuthorization"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_HttpAuthorization as HttpAuthorization;
    impl crate::value::ToValue for HttpAuthorization_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sigv4 {
                properties.insert("Sigv4".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-iotanalyticsaction.html
    pub struct IotAnalyticsAction_ {
        pub batch_mode: Option<crate::value::ExpBool>,
        pub channel_name: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_IotAnalyticsAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.IotAnalyticsAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_IotAnalyticsAction as IotAnalyticsAction;
    impl crate::value::ToValue for IotAnalyticsAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_mode {
                properties.insert(
                    "BatchMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ChannelName".to_string(),
                crate::value::ToValue::to_value(&self.channel_name),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-ioteventsaction.html
    pub struct IotEventsAction_ {
        pub batch_mode: Option<crate::value::ExpBool>,
        pub input_name: crate::value::ExpString,
        pub message_id: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_IotEventsAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.IotEventsAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_IotEventsAction as IotEventsAction;
    impl crate::value::ToValue for IotEventsAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_mode {
                properties.insert(
                    "BatchMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InputName".to_string(),
                crate::value::ToValue::to_value(&self.input_name),
            );
            if let Some(ref value) = self.message_id {
                properties.insert(
                    "MessageId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-iotsitewiseaction.html
    pub struct IotSiteWiseAction_ {
        pub put_asset_property_value_entries: Vec<PutAssetPropertyValueEntry_>,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_IotSiteWiseAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.IotSiteWiseAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_IotSiteWiseAction as IotSiteWiseAction;
    impl crate::value::ToValue for IotSiteWiseAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PutAssetPropertyValueEntries".to_string(),
                crate::value::ToValue::to_value(&self.put_asset_property_value_entries),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kafkaaction.html
    pub struct KafkaAction_ {
        pub client_properties: std::collections::BTreeMap<String, crate::value::ExpString>,
        pub destination_arn: crate::value::ExpString,
        pub headers: Option<Vec<KafkaActionHeader_>>,
        pub key: Option<crate::value::ExpString>,
        pub partition: Option<crate::value::ExpString>,
        pub topic: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_KafkaAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.KafkaAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_KafkaAction as KafkaAction;
    impl crate::value::ToValue for KafkaAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClientProperties".to_string(),
                crate::value::ToValue::to_value(&self.client_properties),
            );
            properties.insert(
                "DestinationArn".to_string(),
                crate::value::ToValue::to_value(&self.destination_arn),
            );
            if let Some(ref value) = self.headers {
                properties.insert(
                    "Headers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.partition {
                properties.insert(
                    "Partition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Topic".to_string(),
                crate::value::ToValue::to_value(&self.topic),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kafkaactionheader.html
    pub struct KafkaActionHeader_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_KafkaActionHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.KafkaActionHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_KafkaActionHeader as KafkaActionHeader;
    impl crate::value::ToValue for KafkaActionHeader_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kinesisaction.html
    pub struct KinesisAction_ {
        pub partition_key: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
        pub stream_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_KinesisAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.KinesisAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_KinesisAction as KinesisAction;
    impl crate::value::ToValue for KinesisAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.partition_key {
                properties.insert(
                    "PartitionKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "StreamName".to_string(),
                crate::value::ToValue::to_value(&self.stream_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-lambdaaction.html
    pub struct LambdaAction_ {
        pub function_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_LambdaAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.LambdaAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_LambdaAction as LambdaAction;
    impl crate::value::ToValue for LambdaAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.function_arn {
                properties.insert(
                    "FunctionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-locationaction.html
    pub struct LocationAction_ {
        pub device_id: crate::value::ExpString,
        pub latitude: crate::value::ExpString,
        pub longitude: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
        pub timestamp: Option<Box<Timestamp_>>,
        pub tracker_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_LocationAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.LocationAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_LocationAction as LocationAction;
    impl crate::value::ToValue for LocationAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DeviceId".to_string(),
                crate::value::ToValue::to_value(&self.device_id),
            );
            properties.insert(
                "Latitude".to_string(),
                crate::value::ToValue::to_value(&self.latitude),
            );
            properties.insert(
                "Longitude".to_string(),
                crate::value::ToValue::to_value(&self.longitude),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.timestamp {
                properties.insert(
                    "Timestamp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TrackerName".to_string(),
                crate::value::ToValue::to_value(&self.tracker_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-opensearchaction.html
    pub struct OpenSearchAction_ {
        pub endpoint: crate::value::ExpString,
        pub id: crate::value::ExpString,
        pub index: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_OpenSearchAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.OpenSearchAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_OpenSearchAction as OpenSearchAction;
    impl crate::value::ToValue for OpenSearchAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(&self.endpoint),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Index".to_string(),
                crate::value::ToValue::to_value(&self.index),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-putassetpropertyvalueentry.html
    pub struct PutAssetPropertyValueEntry_ {
        pub asset_id: Option<crate::value::ExpString>,
        pub entry_id: Option<crate::value::ExpString>,
        pub property_alias: Option<crate::value::ExpString>,
        pub property_id: Option<crate::value::ExpString>,
        pub property_values: Vec<AssetPropertyValue_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_PutAssetPropertyValueEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.PutAssetPropertyValueEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_PutAssetPropertyValueEntry as PutAssetPropertyValueEntry;
    impl crate::value::ToValue for PutAssetPropertyValueEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.asset_id {
                properties.insert(
                    "AssetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.entry_id {
                properties.insert(
                    "EntryId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_alias {
                properties.insert(
                    "PropertyAlias".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_id {
                properties.insert(
                    "PropertyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PropertyValues".to_string(),
                crate::value::ToValue::to_value(&self.property_values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-putiteminput.html
    pub struct PutItemInput_ {
        pub table_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_PutItemInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.PutItemInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_PutItemInput as PutItemInput;
    impl crate::value::ToValue for PutItemInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishaction.html
    pub struct RepublishAction_ {
        pub headers: Option<Box<RepublishActionHeaders_>>,
        pub qos: Option<i64>,
        pub role_arn: crate::value::ExpString,
        pub topic: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_RepublishAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.RepublishAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_RepublishAction as RepublishAction;
    impl crate::value::ToValue for RepublishAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.headers {
                properties.insert(
                    "Headers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.qos {
                properties.insert("Qos".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "Topic".to_string(),
                crate::value::ToValue::to_value(&self.topic),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishactionheaders.html
    pub struct RepublishActionHeaders_ {
        pub content_type: Option<crate::value::ExpString>,
        pub correlation_data: Option<crate::value::ExpString>,
        pub message_expiry: Option<crate::value::ExpString>,
        pub payload_format_indicator: Option<crate::value::ExpString>,
        pub response_topic: Option<crate::value::ExpString>,
        pub user_properties: Option<Vec<UserProperty_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_RepublishActionHeaders {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.RepublishActionHeaders"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_RepublishActionHeaders as RepublishActionHeaders;
    impl crate::value::ToValue for RepublishActionHeaders_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content_type {
                properties.insert(
                    "ContentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.correlation_data {
                properties.insert(
                    "CorrelationData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.message_expiry {
                properties.insert(
                    "MessageExpiry".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.payload_format_indicator {
                properties.insert(
                    "PayloadFormatIndicator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.response_topic {
                properties.insert(
                    "ResponseTopic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_properties {
                properties.insert(
                    "UserProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-s3action.html
    pub struct S3Action_ {
        pub bucket_name: crate::value::ExpString,
        pub canned_acl: Option<crate::value::ExpString>,
        pub key: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_S3Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.S3Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_S3Action as S3Action;
    impl crate::value::ToValue for S3Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            if let Some(ref value) = self.canned_acl {
                properties.insert(
                    "CannedAcl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-sigv4authorization.html
    pub struct SigV4Authorization_ {
        pub role_arn: crate::value::ExpString,
        pub service_name: crate::value::ExpString,
        pub signing_region: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_SigV4Authorization {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.SigV4Authorization"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_SigV4Authorization as SigV4Authorization;
    impl crate::value::ToValue for SigV4Authorization_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "ServiceName".to_string(),
                crate::value::ToValue::to_value(&self.service_name),
            );
            properties.insert(
                "SigningRegion".to_string(),
                crate::value::ToValue::to_value(&self.signing_region),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-snsaction.html
    pub struct SnsAction_ {
        pub message_format: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
        pub target_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_SnsAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.SnsAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_SnsAction as SnsAction;
    impl crate::value::ToValue for SnsAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.message_format {
                properties.insert(
                    "MessageFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "TargetArn".to_string(),
                crate::value::ToValue::to_value(&self.target_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-sqsaction.html
    pub struct SqsAction_ {
        pub queue_url: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
        pub use_base64: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_SqsAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.SqsAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_SqsAction as SqsAction;
    impl crate::value::ToValue for SqsAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "QueueUrl".to_string(),
                crate::value::ToValue::to_value(&self.queue_url),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.use_base64 {
                properties.insert(
                    "UseBase64".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-stepfunctionsaction.html
    pub struct StepFunctionsAction_ {
        pub execution_name_prefix: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
        pub state_machine_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_StepFunctionsAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.StepFunctionsAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_StepFunctionsAction as StepFunctionsAction;
    impl crate::value::ToValue for StepFunctionsAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.execution_name_prefix {
                properties.insert(
                    "ExecutionNamePrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "StateMachineName".to_string(),
                crate::value::ToValue::to_value(&self.state_machine_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestamp.html
    pub struct Timestamp_ {
        pub unit: Option<crate::value::ExpString>,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_Timestamp {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.Timestamp"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_Timestamp as Timestamp;
    impl crate::value::ToValue for Timestamp_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestreamaction.html
    pub struct TimestreamAction_ {
        pub database_name: crate::value::ExpString,
        pub dimensions: Vec<TimestreamDimension_>,
        pub role_arn: crate::value::ExpString,
        pub table_name: crate::value::ExpString,
        pub timestamp: Option<Box<TimestreamTimestamp_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_TimestreamAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.TimestreamAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_TimestreamAction as TimestreamAction;
    impl crate::value::ToValue for TimestreamAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "Dimensions".to_string(),
                crate::value::ToValue::to_value(&self.dimensions),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            if let Some(ref value) = self.timestamp {
                properties.insert(
                    "Timestamp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestreamdimension.html
    pub struct TimestreamDimension_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_TimestreamDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.TimestreamDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_TimestreamDimension as TimestreamDimension;
    impl crate::value::ToValue for TimestreamDimension_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestreamtimestamp.html
    pub struct TimestreamTimestamp_ {
        pub unit: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_TimestreamTimestamp {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.TimestreamTimestamp"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_TimestreamTimestamp as TimestreamTimestamp;
    impl crate::value::ToValue for TimestreamTimestamp_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-topicrulepayload.html
    pub struct TopicRulePayload_ {
        pub actions: Vec<Action_>,
        pub aws_iot_sql_version: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub error_action: Option<Box<Action_>>,
        pub rule_disabled: Option<crate::value::ExpBool>,
        pub sql: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_TopicRulePayload {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.TopicRulePayload"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_TopicRulePayload as TopicRulePayload;
    impl crate::value::ToValue for TopicRulePayload_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Actions".to_string(),
                crate::value::ToValue::to_value(&self.actions),
            );
            if let Some(ref value) = self.aws_iot_sql_version {
                properties.insert(
                    "AwsIotSqlVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.error_action {
                properties.insert(
                    "ErrorAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule_disabled {
                properties.insert(
                    "RuleDisabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Sql".to_string(),
                crate::value::ToValue::to_value(&self.sql),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-userproperty.html
    pub struct UserProperty_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRule_UserProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRule.UserProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRule_UserProperty as UserProperty;
    impl crate::value::ToValue for UserProperty_ {
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
pub mod topicruledestination {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicruledestination-httpurldestinationsummary.html
    pub struct HttpUrlDestinationSummary_ {
        pub confirmation_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRuleDestination_HttpUrlDestinationSummary {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRuleDestination.HttpUrlDestinationSummary"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRuleDestination_HttpUrlDestinationSummary as HttpUrlDestinationSummary;
    impl crate::value::ToValue for HttpUrlDestinationSummary_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.confirmation_url {
                properties.insert(
                    "ConfirmationUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicruledestination-vpcdestinationproperties.html
    pub struct VpcDestinationProperties_ {
        pub role_arn: Option<crate::value::ExpString>,
        pub security_groups: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
        pub vpc_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iot_TopicRuleDestination_VpcDestinationProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoT::TopicRuleDestination.VpcDestinationProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iot_TopicRuleDestination_VpcDestinationProperties as VpcDestinationProperties;
    impl crate::value::ToValue for VpcDestinationProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_groups {
                properties.insert(
                    "SecurityGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnet_ids {
                properties.insert(
                    "SubnetIds".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-accountauditconfiguration.html
pub struct AccountAuditConfiguration_ {
    pub account_id: crate::value::ExpString,
    pub audit_check_configurations:
        super::iot::accountauditconfiguration::AuditCheckConfigurations_,
    pub audit_notification_target_configurations:
        Option<super::iot::accountauditconfiguration::AuditNotificationTargetConfigurations_>,
    pub role_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_AccountAuditConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::AccountAuditConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_AccountAuditConfiguration as AccountAuditConfiguration;
impl crate::template::ToResource for AccountAuditConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AccountAuditConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AccountId".to_string(),
            crate::value::ToValue::to_value(&self.account_id),
        );
        properties.insert(
            "AuditCheckConfigurations".to_string(),
            crate::value::ToValue::to_value(&self.audit_check_configurations),
        );
        if let Some(ref value) = self.audit_notification_target_configurations {
            properties.insert(
                "AuditNotificationTargetConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-authorizer.html
pub struct Authorizer_ {
    pub authorizer_function_arn: crate::value::ExpString,
    pub authorizer_name: Option<crate::value::ExpString>,
    pub enable_caching_for_http: Option<crate::value::ExpBool>,
    pub signing_disabled: Option<crate::value::ExpBool>,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub token_key_name: Option<crate::value::ExpString>,
    pub token_signing_public_keys:
        Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_Authorizer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::Authorizer" $($field
        $value)*)
    };
}
pub use crate::__aws_iot_Authorizer as Authorizer;
impl crate::template::ToResource for Authorizer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Authorizer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AuthorizerFunctionArn".to_string(),
            crate::value::ToValue::to_value(&self.authorizer_function_arn),
        );
        if let Some(ref value) = self.authorizer_name {
            properties.insert(
                "AuthorizerName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_caching_for_http {
            properties.insert(
                "EnableCachingForHttp".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.signing_disabled {
            properties.insert(
                "SigningDisabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.token_key_name {
            properties.insert(
                "TokenKeyName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.token_signing_public_keys {
            properties.insert(
                "TokenSigningPublicKeys".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-billinggroup.html
pub struct BillingGroup_ {
    pub billing_group_name: Option<crate::value::ExpString>,
    pub billing_group_properties: Option<super::iot::billinggroup::BillingGroupProperties_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_BillingGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::BillingGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_BillingGroup as BillingGroup;
impl crate::template::ToResource for BillingGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BillingGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.billing_group_name {
            properties.insert(
                "BillingGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.billing_group_properties {
            properties.insert(
                "BillingGroupProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-cacertificate.html
pub struct CACertificate_ {
    pub auto_registration_status: Option<crate::value::ExpString>,
    pub ca_certificate_pem: crate::value::ExpString,
    pub certificate_mode: Option<crate::value::ExpString>,
    pub registration_config: Option<super::iot::cacertificate::RegistrationConfig_>,
    pub remove_auto_registration: Option<crate::value::ExpBool>,
    pub status: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub verification_certificate_pem: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_CACertificate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::CACertificate"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_CACertificate as CACertificate;
impl crate::template::ToResource for CACertificate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CACertificate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_registration_status {
            properties.insert(
                "AutoRegistrationStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "CACertificatePem".to_string(),
            crate::value::ToValue::to_value(&self.ca_certificate_pem),
        );
        if let Some(ref value) = self.certificate_mode {
            properties.insert(
                "CertificateMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.registration_config {
            properties.insert(
                "RegistrationConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.remove_auto_registration {
            properties.insert(
                "RemoveAutoRegistration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Status".to_string(),
            crate::value::ToValue::to_value(&self.status),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.verification_certificate_pem {
            properties.insert(
                "VerificationCertificatePem".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificate.html
pub struct Certificate_ {
    pub ca_certificate_pem: Option<crate::value::ExpString>,
    pub certificate_mode: Option<crate::value::ExpString>,
    pub certificate_pem: Option<crate::value::ExpString>,
    pub certificate_signing_request: Option<crate::value::ExpString>,
    pub status: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_Certificate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::Certificate"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_Certificate as Certificate;
impl crate::template::ToResource for Certificate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Certificate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.ca_certificate_pem {
            properties.insert(
                "CACertificatePem".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate_mode {
            properties.insert(
                "CertificateMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate_pem {
            properties.insert(
                "CertificatePem".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate_signing_request {
            properties.insert(
                "CertificateSigningRequest".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Status".to_string(),
            crate::value::ToValue::to_value(&self.status),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificateprovider.html
pub struct CertificateProvider_ {
    pub account_default_for_operations: Vec<crate::value::ExpString>,
    pub certificate_provider_name: Option<crate::value::ExpString>,
    pub lambda_function_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_CertificateProvider {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::CertificateProvider"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_CertificateProvider as CertificateProvider;
impl crate::template::ToResource for CertificateProvider_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CertificateProvider"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AccountDefaultForOperations".to_string(),
            crate::value::ToValue::to_value(&self.account_default_for_operations),
        );
        if let Some(ref value) = self.certificate_provider_name {
            properties.insert(
                "CertificateProviderName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LambdaFunctionArn".to_string(),
            crate::value::ToValue::to_value(&self.lambda_function_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-command.html
pub struct Command_ {
    pub command_id: crate::value::ExpString,
    pub created_at: Option<crate::value::ExpString>,
    pub deprecated: Option<crate::value::ExpBool>,
    pub description: Option<crate::value::ExpString>,
    pub display_name: Option<crate::value::ExpString>,
    pub last_updated_at: Option<crate::value::ExpString>,
    pub mandatory_parameters: Option<Vec<super::iot::command::CommandParameter_>>,
    pub namespace: Option<crate::value::ExpString>,
    pub payload: Option<super::iot::command::CommandPayload_>,
    pub pending_deletion: Option<crate::value::ExpBool>,
    pub role_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_Command {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::Command" $($field
        $value)*)
    };
}
pub use crate::__aws_iot_Command as Command;
impl crate::template::ToResource for Command_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Command"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CommandId".to_string(),
            crate::value::ToValue::to_value(&self.command_id),
        );
        if let Some(ref value) = self.created_at {
            properties.insert(
                "CreatedAt".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deprecated {
            properties.insert(
                "Deprecated".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.last_updated_at {
            properties.insert(
                "LastUpdatedAt".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.mandatory_parameters {
            properties.insert(
                "MandatoryParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.namespace {
            properties.insert(
                "Namespace".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.payload {
            properties.insert(
                "Payload".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.pending_deletion {
            properties.insert(
                "PendingDeletion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-custommetric.html
pub struct CustomMetric_ {
    pub display_name: Option<crate::value::ExpString>,
    pub metric_name: Option<crate::value::ExpString>,
    pub metric_type: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_CustomMetric {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::CustomMetric"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_CustomMetric as CustomMetric;
impl crate::template::ToResource for CustomMetric_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CustomMetric"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metric_name {
            properties.insert(
                "MetricName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MetricType".to_string(),
            crate::value::ToValue::to_value(&self.metric_type),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-dimension.html
pub struct Dimension_ {
    pub name: Option<crate::value::ExpString>,
    pub string_values: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_Dimension {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::Dimension" $($field
        $value)*)
    };
}
pub use crate::__aws_iot_Dimension as Dimension;
impl crate::template::ToResource for Dimension_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Dimension"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "StringValues".to_string(),
            crate::value::ToValue::to_value(&self.string_values),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-domainconfiguration.html
pub struct DomainConfiguration_ {
    pub application_protocol: Option<crate::value::ExpString>,
    pub authentication_type: Option<crate::value::ExpString>,
    pub authorizer_config: Option<super::iot::domainconfiguration::AuthorizerConfig_>,
    pub client_certificate_config:
        Option<super::iot::domainconfiguration::ClientCertificateConfig_>,
    pub domain_configuration_name: Option<crate::value::ExpString>,
    pub domain_configuration_status: Option<crate::value::ExpString>,
    pub domain_name: Option<crate::value::ExpString>,
    pub server_certificate_arns: Option<Vec<crate::value::ExpString>>,
    pub server_certificate_config:
        Option<super::iot::domainconfiguration::ServerCertificateConfig_>,
    pub service_type: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub tls_config: Option<super::iot::domainconfiguration::TlsConfig_>,
    pub validation_certificate_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_DomainConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::DomainConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_DomainConfiguration as DomainConfiguration;
impl crate::template::ToResource for DomainConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DomainConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.application_protocol {
            properties.insert(
                "ApplicationProtocol".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.authentication_type {
            properties.insert(
                "AuthenticationType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.authorizer_config {
            properties.insert(
                "AuthorizerConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.client_certificate_config {
            properties.insert(
                "ClientCertificateConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_configuration_name {
            properties.insert(
                "DomainConfigurationName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_configuration_status {
            properties.insert(
                "DomainConfigurationStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_name {
            properties.insert(
                "DomainName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.server_certificate_arns {
            properties.insert(
                "ServerCertificateArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.server_certificate_config {
            properties.insert(
                "ServerCertificateConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_type {
            properties.insert(
                "ServiceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tls_config {
            properties.insert(
                "TlsConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.validation_certificate_arn {
            properties.insert(
                "ValidationCertificateArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-encryptionconfiguration.html
pub struct EncryptionConfiguration_ {
    pub encryption_type: crate::value::ExpString,
    pub kms_access_role_arn: Option<crate::value::ExpString>,
    pub kms_key_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_EncryptionConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::EncryptionConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_EncryptionConfiguration as EncryptionConfiguration;
impl crate::template::ToResource for EncryptionConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EncryptionConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "EncryptionType".to_string(),
            crate::value::ToValue::to_value(&self.encryption_type),
        );
        if let Some(ref value) = self.kms_access_role_arn {
            properties.insert(
                "KmsAccessRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_arn {
            properties.insert(
                "KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-fleetmetric.html
pub struct FleetMetric_ {
    pub aggregation_field: Option<crate::value::ExpString>,
    pub aggregation_type: Option<super::iot::fleetmetric::AggregationType_>,
    pub description: Option<crate::value::ExpString>,
    pub index_name: Option<crate::value::ExpString>,
    pub metric_name: crate::value::ExpString,
    pub period: Option<i64>,
    pub query_string: Option<crate::value::ExpString>,
    pub query_version: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub unit: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_FleetMetric {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::FleetMetric"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_FleetMetric as FleetMetric;
impl crate::template::ToResource for FleetMetric_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FleetMetric"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.aggregation_field {
            properties.insert(
                "AggregationField".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.aggregation_type {
            properties.insert(
                "AggregationType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.index_name {
            properties.insert(
                "IndexName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MetricName".to_string(),
            crate::value::ToValue::to_value(&self.metric_name),
        );
        if let Some(ref value) = self.period {
            properties.insert("Period".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.query_string {
            properties.insert(
                "QueryString".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.query_version {
            properties.insert(
                "QueryVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.unit {
            properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-jobtemplate.html
pub struct JobTemplate_ {
    pub abort_config: Option<super::iot::jobtemplate::AbortConfig_>,
    pub description: crate::value::ExpString,
    pub destination_package_versions: Option<Vec<crate::value::ExpString>>,
    pub document: Option<crate::value::ExpString>,
    pub document_source: Option<crate::value::ExpString>,
    pub job_arn: Option<crate::value::ExpString>,
    pub job_executions_retry_config: Option<super::iot::jobtemplate::JobExecutionsRetryConfig_>,
    pub job_executions_rollout_config: Option<super::iot::jobtemplate::JobExecutionsRolloutConfig_>,
    pub job_template_id: crate::value::ExpString,
    pub maintenance_windows: Option<Vec<super::iot::jobtemplate::MaintenanceWindow_>>,
    pub presigned_url_config: Option<super::iot::jobtemplate::PresignedUrlConfig_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub timeout_config: Option<super::iot::jobtemplate::TimeoutConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_JobTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::JobTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_JobTemplate as JobTemplate;
impl crate::template::ToResource for JobTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("JobTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.abort_config {
            properties.insert(
                "AbortConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        if let Some(ref value) = self.destination_package_versions {
            properties.insert(
                "DestinationPackageVersions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.document {
            properties.insert(
                "Document".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.document_source {
            properties.insert(
                "DocumentSource".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_arn {
            properties.insert("JobArn".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.job_executions_retry_config {
            properties.insert(
                "JobExecutionsRetryConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_executions_rollout_config {
            properties.insert(
                "JobExecutionsRolloutConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "JobTemplateId".to_string(),
            crate::value::ToValue::to_value(&self.job_template_id),
        );
        if let Some(ref value) = self.maintenance_windows {
            properties.insert(
                "MaintenanceWindows".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.presigned_url_config {
            properties.insert(
                "PresignedUrlConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.timeout_config {
            properties.insert(
                "TimeoutConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-logging.html
pub struct Logging_ {
    pub account_id: crate::value::ExpString,
    pub default_log_level: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_Logging {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::Logging" $($field
        $value)*)
    };
}
pub use crate::__aws_iot_Logging as Logging;
impl crate::template::ToResource for Logging_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Logging"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AccountId".to_string(),
            crate::value::ToValue::to_value(&self.account_id),
        );
        properties.insert(
            "DefaultLogLevel".to_string(),
            crate::value::ToValue::to_value(&self.default_log_level),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-mitigationaction.html
pub struct MitigationAction_ {
    pub action_name: Option<crate::value::ExpString>,
    pub action_params: super::iot::mitigationaction::ActionParams_,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_MitigationAction {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::MitigationAction"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_MitigationAction as MitigationAction;
impl crate::template::ToResource for MitigationAction_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MitigationAction"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.action_name {
            properties.insert(
                "ActionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ActionParams".to_string(),
            crate::value::ToValue::to_value(&self.action_params),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policy.html
pub struct Policy_ {
    pub policy_document: serde_json::Value,
    pub policy_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_Policy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::Policy" $($field
        $value)*)
    };
}
pub use crate::__aws_iot_Policy as Policy;
impl crate::template::ToResource for Policy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Policy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PolicyDocument".to_string(),
            crate::value::ToValue::to_value(&self.policy_document),
        );
        if let Some(ref value) = self.policy_name {
            properties.insert(
                "PolicyName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policyprincipalattachment.html
pub struct PolicyPrincipalAttachment_ {
    pub policy_name: crate::value::ExpString,
    pub principal: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_PolicyPrincipalAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::PolicyPrincipalAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_PolicyPrincipalAttachment as PolicyPrincipalAttachment;
impl crate::template::ToResource for PolicyPrincipalAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PolicyPrincipalAttachment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PolicyName".to_string(),
            crate::value::ToValue::to_value(&self.policy_name),
        );
        properties.insert(
            "Principal".to_string(),
            crate::value::ToValue::to_value(&self.principal),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-provisioningtemplate.html
pub struct ProvisioningTemplate_ {
    pub description: Option<crate::value::ExpString>,
    pub enabled: Option<crate::value::ExpBool>,
    pub pre_provisioning_hook: Option<super::iot::provisioningtemplate::ProvisioningHook_>,
    pub provisioning_role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub template_body: crate::value::ExpString,
    pub template_name: Option<crate::value::ExpString>,
    pub template_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_ProvisioningTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::ProvisioningTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_ProvisioningTemplate as ProvisioningTemplate;
impl crate::template::ToResource for ProvisioningTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ProvisioningTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.pre_provisioning_hook {
            properties.insert(
                "PreProvisioningHook".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ProvisioningRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.provisioning_role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TemplateBody".to_string(),
            crate::value::ToValue::to_value(&self.template_body),
        );
        if let Some(ref value) = self.template_name {
            properties.insert(
                "TemplateName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.template_type {
            properties.insert(
                "TemplateType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-resourcespecificlogging.html
pub struct ResourceSpecificLogging_ {
    pub log_level: crate::value::ExpString,
    pub target_name: crate::value::ExpString,
    pub target_type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_ResourceSpecificLogging {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::ResourceSpecificLogging"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_ResourceSpecificLogging as ResourceSpecificLogging;
impl crate::template::ToResource for ResourceSpecificLogging_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourceSpecificLogging"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "LogLevel".to_string(),
            crate::value::ToValue::to_value(&self.log_level),
        );
        properties.insert(
            "TargetName".to_string(),
            crate::value::ToValue::to_value(&self.target_name),
        );
        properties.insert(
            "TargetType".to_string(),
            crate::value::ToValue::to_value(&self.target_type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-rolealias.html
pub struct RoleAlias_ {
    pub credential_duration_seconds: Option<i64>,
    pub role_alias: Option<crate::value::ExpString>,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_RoleAlias {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::RoleAlias" $($field
        $value)*)
    };
}
pub use crate::__aws_iot_RoleAlias as RoleAlias;
impl crate::template::ToResource for RoleAlias_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RoleAlias"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.credential_duration_seconds {
            properties.insert(
                "CredentialDurationSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.role_alias {
            properties.insert(
                "RoleAlias".to_string(),
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
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-scheduledaudit.html
pub struct ScheduledAudit_ {
    pub day_of_month: Option<crate::value::ExpString>,
    pub day_of_week: Option<crate::value::ExpString>,
    pub frequency: crate::value::ExpString,
    pub scheduled_audit_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_check_names: Vec<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_ScheduledAudit {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::ScheduledAudit"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_ScheduledAudit as ScheduledAudit;
impl crate::template::ToResource for ScheduledAudit_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ScheduledAudit"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.day_of_month {
            properties.insert(
                "DayOfMonth".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.day_of_week {
            properties.insert(
                "DayOfWeek".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Frequency".to_string(),
            crate::value::ToValue::to_value(&self.frequency),
        );
        if let Some(ref value) = self.scheduled_audit_name {
            properties.insert(
                "ScheduledAuditName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TargetCheckNames".to_string(),
            crate::value::ToValue::to_value(&self.target_check_names),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-securityprofile.html
pub struct SecurityProfile_ {
    pub additional_metrics_to_retain_v2: Option<Vec<super::iot::securityprofile::MetricToRetain_>>,
    pub alert_targets:
        Option<std::collections::BTreeMap<String, super::iot::securityprofile::AlertTarget_>>,
    pub behaviors: Option<Vec<super::iot::securityprofile::Behavior_>>,
    pub metrics_export_config: Option<super::iot::securityprofile::MetricsExportConfig_>,
    pub security_profile_description: Option<crate::value::ExpString>,
    pub security_profile_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_arns: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_SecurityProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::SecurityProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_SecurityProfile as SecurityProfile;
impl crate::template::ToResource for SecurityProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SecurityProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_metrics_to_retain_v2 {
            properties.insert(
                "AdditionalMetricsToRetainV2".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.alert_targets {
            properties.insert(
                "AlertTargets".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.behaviors {
            properties.insert(
                "Behaviors".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metrics_export_config {
            properties.insert(
                "MetricsExportConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_profile_description {
            properties.insert(
                "SecurityProfileDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_profile_name {
            properties.insert(
                "SecurityProfileName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.target_arns {
            properties.insert(
                "TargetArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-softwarepackage.html
pub struct SoftwarePackage_ {
    pub description: Option<crate::value::ExpString>,
    pub package_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_SoftwarePackage {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::SoftwarePackage"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_SoftwarePackage as SoftwarePackage;
impl crate::template::ToResource for SoftwarePackage_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SoftwarePackage"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.package_name {
            properties.insert(
                "PackageName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-softwarepackageversion.html
pub struct SoftwarePackageVersion_ {
    pub artifact: Option<super::iot::softwarepackageversion::PackageVersionArtifact_>,
    pub attributes: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub description: Option<crate::value::ExpString>,
    pub package_name: crate::value::ExpString,
    pub recipe: Option<crate::value::ExpString>,
    pub sbom: Option<super::iot::softwarepackageversion::Sbom_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub version_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_SoftwarePackageVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::SoftwarePackageVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_SoftwarePackageVersion as SoftwarePackageVersion;
impl crate::template::ToResource for SoftwarePackageVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SoftwarePackageVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.artifact {
            properties.insert(
                "Artifact".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.attributes {
            properties.insert(
                "Attributes".to_string(),
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
            "PackageName".to_string(),
            crate::value::ToValue::to_value(&self.package_name),
        );
        if let Some(ref value) = self.recipe {
            properties.insert("Recipe".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.sbom {
            properties.insert("Sbom".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.version_name {
            properties.insert(
                "VersionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thing.html
pub struct Thing_ {
    pub attribute_payload: Option<super::iot::thing::AttributePayload_>,
    pub thing_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_Thing {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::Thing" $($field
        $value)*)
    };
}
pub use crate::__aws_iot_Thing as Thing;
impl crate::template::ToResource for Thing_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Thing"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.attribute_payload {
            properties.insert(
                "AttributePayload".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.thing_name {
            properties.insert(
                "ThingName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thinggroup.html
pub struct ThingGroup_ {
    pub parent_group_name: Option<crate::value::ExpString>,
    pub query_string: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub thing_group_name: Option<crate::value::ExpString>,
    pub thing_group_properties: Option<super::iot::thinggroup::ThingGroupProperties_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_ThingGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::ThingGroup" $($field
        $value)*)
    };
}
pub use crate::__aws_iot_ThingGroup as ThingGroup;
impl crate::template::ToResource for ThingGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ThingGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.parent_group_name {
            properties.insert(
                "ParentGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.query_string {
            properties.insert(
                "QueryString".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.thing_group_name {
            properties.insert(
                "ThingGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.thing_group_properties {
            properties.insert(
                "ThingGroupProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thingprincipalattachment.html
pub struct ThingPrincipalAttachment_ {
    pub principal: crate::value::ExpString,
    pub thing_name: crate::value::ExpString,
    pub thing_principal_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_ThingPrincipalAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::ThingPrincipalAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_ThingPrincipalAttachment as ThingPrincipalAttachment;
impl crate::template::ToResource for ThingPrincipalAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ThingPrincipalAttachment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Principal".to_string(),
            crate::value::ToValue::to_value(&self.principal),
        );
        properties.insert(
            "ThingName".to_string(),
            crate::value::ToValue::to_value(&self.thing_name),
        );
        if let Some(ref value) = self.thing_principal_type {
            properties.insert(
                "ThingPrincipalType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thingtype.html
pub struct ThingType_ {
    pub deprecate_thing_type: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub thing_type_name: Option<crate::value::ExpString>,
    pub thing_type_properties: Option<super::iot::thingtype::ThingTypeProperties_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_ThingType {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::ThingType" $($field
        $value)*)
    };
}
pub use crate::__aws_iot_ThingType as ThingType;
impl crate::template::ToResource for ThingType_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ThingType"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.deprecate_thing_type {
            properties.insert(
                "DeprecateThingType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.thing_type_name {
            properties.insert(
                "ThingTypeName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.thing_type_properties {
            properties.insert(
                "ThingTypeProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-topicrule.html
pub struct TopicRule_ {
    pub rule_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub topic_rule_payload: super::iot::topicrule::TopicRulePayload_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_TopicRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::TopicRule" $($field
        $value)*)
    };
}
pub use crate::__aws_iot_TopicRule as TopicRule;
impl crate::template::ToResource for TopicRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TopicRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.rule_name {
            properties.insert(
                "RuleName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TopicRulePayload".to_string(),
            crate::value::ToValue::to_value(&self.topic_rule_payload),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-topicruledestination.html
pub struct TopicRuleDestination_ {
    pub http_url_properties: Option<super::iot::topicruledestination::HttpUrlDestinationSummary_>,
    pub status: Option<crate::value::ExpString>,
    pub vpc_properties: Option<super::iot::topicruledestination::VpcDestinationProperties_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iot_TopicRuleDestination {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoT::TopicRuleDestination"
        $($field $value)*)
    };
}
pub use crate::__aws_iot_TopicRuleDestination as TopicRuleDestination;
impl crate::template::ToResource for TopicRuleDestination_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoT"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TopicRuleDestination"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.http_url_properties {
            properties.insert(
                "HttpUrlProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_properties {
            properties.insert(
                "VpcProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
