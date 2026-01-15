pub mod connector {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-connector-vpcinformation.html
    pub struct VpcInformation_ {
        pub ip_address_type: Option<crate::value::ExpString>,
        pub security_group_ids: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Connector_VpcInformation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Connector.VpcInformation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Connector_VpcInformation as VpcInformation;
    impl crate::value::ToValue for VpcInformation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ip_address_type {
                properties.insert(
                    "IpAddressType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.into()
        }
    }
}
pub mod template {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-applicationpolicies.html
    pub struct ApplicationPolicies_ {
        pub critical: Option<crate::value::ExpBool>,
        pub policies: Vec<ApplicationPolicy_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_ApplicationPolicies {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.ApplicationPolicies"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_ApplicationPolicies as ApplicationPolicies;
    impl crate::value::ToValue for ApplicationPolicies_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.critical {
                properties.insert(
                    "Critical".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Policies".to_string(),
                crate::value::ToValue::to_value(&self.policies),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-applicationpolicy.html
    pub struct ApplicationPolicy_ {
        pub policy_object_identifier: Option<crate::value::ExpString>,
        pub policy_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_ApplicationPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.ApplicationPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_ApplicationPolicy as ApplicationPolicy;
    impl crate::value::ToValue for ApplicationPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.policy_object_identifier {
                properties.insert(
                    "PolicyObjectIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.policy_type {
                properties.insert(
                    "PolicyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-certificatevalidity.html
    pub struct CertificateValidity_ {
        pub renewal_period: Box<ValidityPeriod_>,
        pub validity_period: Box<ValidityPeriod_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_CertificateValidity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.CertificateValidity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_CertificateValidity as CertificateValidity;
    impl crate::value::ToValue for CertificateValidity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RenewalPeriod".to_string(),
                crate::value::ToValue::to_value(&self.renewal_period),
            );
            properties.insert(
                "ValidityPeriod".to_string(),
                crate::value::ToValue::to_value(&self.validity_period),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv2.html
    pub struct EnrollmentFlagsV2_ {
        pub enable_key_reuse_on_nt_token_keyset_storage_full: Option<crate::value::ExpBool>,
        pub include_symmetric_algorithms: Option<crate::value::ExpBool>,
        pub no_security_extension: Option<crate::value::ExpBool>,
        pub remove_invalid_certificate_from_personal_store: Option<crate::value::ExpBool>,
        pub user_interaction_required: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_EnrollmentFlagsV2 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.EnrollmentFlagsV2"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_EnrollmentFlagsV2 as EnrollmentFlagsV2;
    impl crate::value::ToValue for EnrollmentFlagsV2_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_key_reuse_on_nt_token_keyset_storage_full {
                properties.insert(
                    "EnableKeyReuseOnNtTokenKeysetStorageFull".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_symmetric_algorithms {
                properties.insert(
                    "IncludeSymmetricAlgorithms".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.no_security_extension {
                properties.insert(
                    "NoSecurityExtension".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remove_invalid_certificate_from_personal_store {
                properties.insert(
                    "RemoveInvalidCertificateFromPersonalStore".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_interaction_required {
                properties.insert(
                    "UserInteractionRequired".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv3.html
    pub struct EnrollmentFlagsV3_ {
        pub enable_key_reuse_on_nt_token_keyset_storage_full: Option<crate::value::ExpBool>,
        pub include_symmetric_algorithms: Option<crate::value::ExpBool>,
        pub no_security_extension: Option<crate::value::ExpBool>,
        pub remove_invalid_certificate_from_personal_store: Option<crate::value::ExpBool>,
        pub user_interaction_required: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_EnrollmentFlagsV3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.EnrollmentFlagsV3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_EnrollmentFlagsV3 as EnrollmentFlagsV3;
    impl crate::value::ToValue for EnrollmentFlagsV3_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_key_reuse_on_nt_token_keyset_storage_full {
                properties.insert(
                    "EnableKeyReuseOnNtTokenKeysetStorageFull".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_symmetric_algorithms {
                properties.insert(
                    "IncludeSymmetricAlgorithms".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.no_security_extension {
                properties.insert(
                    "NoSecurityExtension".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remove_invalid_certificate_from_personal_store {
                properties.insert(
                    "RemoveInvalidCertificateFromPersonalStore".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_interaction_required {
                properties.insert(
                    "UserInteractionRequired".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-enrollmentflagsv4.html
    pub struct EnrollmentFlagsV4_ {
        pub enable_key_reuse_on_nt_token_keyset_storage_full: Option<crate::value::ExpBool>,
        pub include_symmetric_algorithms: Option<crate::value::ExpBool>,
        pub no_security_extension: Option<crate::value::ExpBool>,
        pub remove_invalid_certificate_from_personal_store: Option<crate::value::ExpBool>,
        pub user_interaction_required: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_EnrollmentFlagsV4 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.EnrollmentFlagsV4"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_EnrollmentFlagsV4 as EnrollmentFlagsV4;
    impl crate::value::ToValue for EnrollmentFlagsV4_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_key_reuse_on_nt_token_keyset_storage_full {
                properties.insert(
                    "EnableKeyReuseOnNtTokenKeysetStorageFull".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_symmetric_algorithms {
                properties.insert(
                    "IncludeSymmetricAlgorithms".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.no_security_extension {
                properties.insert(
                    "NoSecurityExtension".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remove_invalid_certificate_from_personal_store {
                properties.insert(
                    "RemoveInvalidCertificateFromPersonalStore".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_interaction_required {
                properties.insert(
                    "UserInteractionRequired".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-extensionsv2.html
    pub struct ExtensionsV2_ {
        pub application_policies: Option<Box<ApplicationPolicies_>>,
        pub key_usage: Box<KeyUsage_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_ExtensionsV2 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.ExtensionsV2"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_ExtensionsV2 as ExtensionsV2;
    impl crate::value::ToValue for ExtensionsV2_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_policies {
                properties.insert(
                    "ApplicationPolicies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "KeyUsage".to_string(),
                crate::value::ToValue::to_value(&self.key_usage),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-extensionsv3.html
    pub struct ExtensionsV3_ {
        pub application_policies: Option<Box<ApplicationPolicies_>>,
        pub key_usage: Box<KeyUsage_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_ExtensionsV3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.ExtensionsV3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_ExtensionsV3 as ExtensionsV3;
    impl crate::value::ToValue for ExtensionsV3_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_policies {
                properties.insert(
                    "ApplicationPolicies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "KeyUsage".to_string(),
                crate::value::ToValue::to_value(&self.key_usage),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-extensionsv4.html
    pub struct ExtensionsV4_ {
        pub application_policies: Option<Box<ApplicationPolicies_>>,
        pub key_usage: Box<KeyUsage_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_ExtensionsV4 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.ExtensionsV4"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_ExtensionsV4 as ExtensionsV4;
    impl crate::value::ToValue for ExtensionsV4_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_policies {
                properties.insert(
                    "ApplicationPolicies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "KeyUsage".to_string(),
                crate::value::ToValue::to_value(&self.key_usage),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-generalflagsv2.html
    pub struct GeneralFlagsV2_ {
        pub auto_enrollment: Option<crate::value::ExpBool>,
        pub machine_type: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_GeneralFlagsV2 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.GeneralFlagsV2"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_GeneralFlagsV2 as GeneralFlagsV2;
    impl crate::value::ToValue for GeneralFlagsV2_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_enrollment {
                properties.insert(
                    "AutoEnrollment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.machine_type {
                properties.insert(
                    "MachineType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-generalflagsv3.html
    pub struct GeneralFlagsV3_ {
        pub auto_enrollment: Option<crate::value::ExpBool>,
        pub machine_type: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_GeneralFlagsV3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.GeneralFlagsV3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_GeneralFlagsV3 as GeneralFlagsV3;
    impl crate::value::ToValue for GeneralFlagsV3_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_enrollment {
                properties.insert(
                    "AutoEnrollment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.machine_type {
                properties.insert(
                    "MachineType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-generalflagsv4.html
    pub struct GeneralFlagsV4_ {
        pub auto_enrollment: Option<crate::value::ExpBool>,
        pub machine_type: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_GeneralFlagsV4 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.GeneralFlagsV4"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_GeneralFlagsV4 as GeneralFlagsV4;
    impl crate::value::ToValue for GeneralFlagsV4_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_enrollment {
                properties.insert(
                    "AutoEnrollment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.machine_type {
                properties.insert(
                    "MachineType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusage.html
    pub struct KeyUsage_ {
        pub critical: Option<crate::value::ExpBool>,
        pub usage_flags: Box<KeyUsageFlags_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_KeyUsage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.KeyUsage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_KeyUsage as KeyUsage;
    impl crate::value::ToValue for KeyUsage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.critical {
                properties.insert(
                    "Critical".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "UsageFlags".to_string(),
                crate::value::ToValue::to_value(&self.usage_flags),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusageflags.html
    pub struct KeyUsageFlags_ {
        pub data_encipherment: Option<crate::value::ExpBool>,
        pub digital_signature: Option<crate::value::ExpBool>,
        pub key_agreement: Option<crate::value::ExpBool>,
        pub key_encipherment: Option<crate::value::ExpBool>,
        pub non_repudiation: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_KeyUsageFlags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.KeyUsageFlags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_KeyUsageFlags as KeyUsageFlags;
    impl crate::value::ToValue for KeyUsageFlags_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_encipherment {
                properties.insert(
                    "DataEncipherment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.digital_signature {
                properties.insert(
                    "DigitalSignature".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_agreement {
                properties.insert(
                    "KeyAgreement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_encipherment {
                properties.insert(
                    "KeyEncipherment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.non_repudiation {
                properties.insert(
                    "NonRepudiation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusageproperty.html
    pub struct KeyUsageProperty_ {
        pub property_flags: Option<Box<KeyUsagePropertyFlags_>>,
        pub property_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_KeyUsageProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.KeyUsageProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_KeyUsageProperty as KeyUsageProperty;
    impl crate::value::ToValue for KeyUsageProperty_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.property_flags {
                properties.insert(
                    "PropertyFlags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_type {
                properties.insert(
                    "PropertyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-keyusagepropertyflags.html
    pub struct KeyUsagePropertyFlags_ {
        pub decrypt: Option<crate::value::ExpBool>,
        pub key_agreement: Option<crate::value::ExpBool>,
        pub sign: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_KeyUsagePropertyFlags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.KeyUsagePropertyFlags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_KeyUsagePropertyFlags as KeyUsagePropertyFlags;
    impl crate::value::ToValue for KeyUsagePropertyFlags_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.decrypt {
                properties.insert(
                    "Decrypt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_agreement {
                properties.insert(
                    "KeyAgreement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sign {
                properties.insert("Sign".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv2.html
    pub struct PrivateKeyAttributesV2_ {
        pub crypto_providers: Option<Vec<crate::value::ExpString>>,
        pub key_spec: crate::value::ExpString,
        pub minimal_key_length: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_PrivateKeyAttributesV2 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.PrivateKeyAttributesV2"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_PrivateKeyAttributesV2 as PrivateKeyAttributesV2;
    impl crate::value::ToValue for PrivateKeyAttributesV2_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.crypto_providers {
                properties.insert(
                    "CryptoProviders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "KeySpec".to_string(),
                crate::value::ToValue::to_value(&self.key_spec),
            );
            properties.insert(
                "MinimalKeyLength".to_string(),
                crate::value::ToValue::to_value(&self.minimal_key_length),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv3.html
    pub struct PrivateKeyAttributesV3_ {
        pub algorithm: crate::value::ExpString,
        pub crypto_providers: Option<Vec<crate::value::ExpString>>,
        pub key_spec: crate::value::ExpString,
        pub key_usage_property: Box<KeyUsageProperty_>,
        pub minimal_key_length: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_PrivateKeyAttributesV3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.PrivateKeyAttributesV3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_PrivateKeyAttributesV3 as PrivateKeyAttributesV3;
    impl crate::value::ToValue for PrivateKeyAttributesV3_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Algorithm".to_string(),
                crate::value::ToValue::to_value(&self.algorithm),
            );
            if let Some(ref value) = self.crypto_providers {
                properties.insert(
                    "CryptoProviders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "KeySpec".to_string(),
                crate::value::ToValue::to_value(&self.key_spec),
            );
            properties.insert(
                "KeyUsageProperty".to_string(),
                crate::value::ToValue::to_value(&self.key_usage_property),
            );
            properties.insert(
                "MinimalKeyLength".to_string(),
                crate::value::ToValue::to_value(&self.minimal_key_length),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyattributesv4.html
    pub struct PrivateKeyAttributesV4_ {
        pub algorithm: Option<crate::value::ExpString>,
        pub crypto_providers: Option<Vec<crate::value::ExpString>>,
        pub key_spec: crate::value::ExpString,
        pub key_usage_property: Option<Box<KeyUsageProperty_>>,
        pub minimal_key_length: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_PrivateKeyAttributesV4 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.PrivateKeyAttributesV4"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_PrivateKeyAttributesV4 as PrivateKeyAttributesV4;
    impl crate::value::ToValue for PrivateKeyAttributesV4_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.algorithm {
                properties.insert(
                    "Algorithm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.crypto_providers {
                properties.insert(
                    "CryptoProviders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "KeySpec".to_string(),
                crate::value::ToValue::to_value(&self.key_spec),
            );
            if let Some(ref value) = self.key_usage_property {
                properties.insert(
                    "KeyUsageProperty".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MinimalKeyLength".to_string(),
                crate::value::ToValue::to_value(&self.minimal_key_length),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv2.html
    pub struct PrivateKeyFlagsV2_ {
        pub client_version: crate::value::ExpString,
        pub exportable_key: Option<crate::value::ExpBool>,
        pub strong_key_protection_required: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_PrivateKeyFlagsV2 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.PrivateKeyFlagsV2"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_PrivateKeyFlagsV2 as PrivateKeyFlagsV2;
    impl crate::value::ToValue for PrivateKeyFlagsV2_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClientVersion".to_string(),
                crate::value::ToValue::to_value(&self.client_version),
            );
            if let Some(ref value) = self.exportable_key {
                properties.insert(
                    "ExportableKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.strong_key_protection_required {
                properties.insert(
                    "StrongKeyProtectionRequired".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv3.html
    pub struct PrivateKeyFlagsV3_ {
        pub client_version: crate::value::ExpString,
        pub exportable_key: Option<crate::value::ExpBool>,
        pub require_alternate_signature_algorithm: Option<crate::value::ExpBool>,
        pub strong_key_protection_required: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_PrivateKeyFlagsV3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.PrivateKeyFlagsV3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_PrivateKeyFlagsV3 as PrivateKeyFlagsV3;
    impl crate::value::ToValue for PrivateKeyFlagsV3_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClientVersion".to_string(),
                crate::value::ToValue::to_value(&self.client_version),
            );
            if let Some(ref value) = self.exportable_key {
                properties.insert(
                    "ExportableKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_alternate_signature_algorithm {
                properties.insert(
                    "RequireAlternateSignatureAlgorithm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.strong_key_protection_required {
                properties.insert(
                    "StrongKeyProtectionRequired".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-privatekeyflagsv4.html
    pub struct PrivateKeyFlagsV4_ {
        pub client_version: crate::value::ExpString,
        pub exportable_key: Option<crate::value::ExpBool>,
        pub require_alternate_signature_algorithm: Option<crate::value::ExpBool>,
        pub require_same_key_renewal: Option<crate::value::ExpBool>,
        pub strong_key_protection_required: Option<crate::value::ExpBool>,
        pub use_legacy_provider: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_PrivateKeyFlagsV4 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.PrivateKeyFlagsV4"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_PrivateKeyFlagsV4 as PrivateKeyFlagsV4;
    impl crate::value::ToValue for PrivateKeyFlagsV4_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClientVersion".to_string(),
                crate::value::ToValue::to_value(&self.client_version),
            );
            if let Some(ref value) = self.exportable_key {
                properties.insert(
                    "ExportableKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_alternate_signature_algorithm {
                properties.insert(
                    "RequireAlternateSignatureAlgorithm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_same_key_renewal {
                properties.insert(
                    "RequireSameKeyRenewal".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.strong_key_protection_required {
                properties.insert(
                    "StrongKeyProtectionRequired".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_legacy_provider {
                properties.insert(
                    "UseLegacyProvider".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv2.html
    pub struct SubjectNameFlagsV2_ {
        pub require_common_name: Option<crate::value::ExpBool>,
        pub require_directory_path: Option<crate::value::ExpBool>,
        pub require_dns_as_cn: Option<crate::value::ExpBool>,
        pub require_email: Option<crate::value::ExpBool>,
        pub san_require_directory_guid: Option<crate::value::ExpBool>,
        pub san_require_dns: Option<crate::value::ExpBool>,
        pub san_require_domain_dns: Option<crate::value::ExpBool>,
        pub san_require_email: Option<crate::value::ExpBool>,
        pub san_require_spn: Option<crate::value::ExpBool>,
        pub san_require_upn: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_SubjectNameFlagsV2 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.SubjectNameFlagsV2"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_SubjectNameFlagsV2 as SubjectNameFlagsV2;
    impl crate::value::ToValue for SubjectNameFlagsV2_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.require_common_name {
                properties.insert(
                    "RequireCommonName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_directory_path {
                properties.insert(
                    "RequireDirectoryPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_dns_as_cn {
                properties.insert(
                    "RequireDnsAsCn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_email {
                properties.insert(
                    "RequireEmail".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_directory_guid {
                properties.insert(
                    "SanRequireDirectoryGuid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_dns {
                properties.insert(
                    "SanRequireDns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_domain_dns {
                properties.insert(
                    "SanRequireDomainDns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_email {
                properties.insert(
                    "SanRequireEmail".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_spn {
                properties.insert(
                    "SanRequireSpn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_upn {
                properties.insert(
                    "SanRequireUpn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv3.html
    pub struct SubjectNameFlagsV3_ {
        pub require_common_name: Option<crate::value::ExpBool>,
        pub require_directory_path: Option<crate::value::ExpBool>,
        pub require_dns_as_cn: Option<crate::value::ExpBool>,
        pub require_email: Option<crate::value::ExpBool>,
        pub san_require_directory_guid: Option<crate::value::ExpBool>,
        pub san_require_dns: Option<crate::value::ExpBool>,
        pub san_require_domain_dns: Option<crate::value::ExpBool>,
        pub san_require_email: Option<crate::value::ExpBool>,
        pub san_require_spn: Option<crate::value::ExpBool>,
        pub san_require_upn: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_SubjectNameFlagsV3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.SubjectNameFlagsV3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_SubjectNameFlagsV3 as SubjectNameFlagsV3;
    impl crate::value::ToValue for SubjectNameFlagsV3_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.require_common_name {
                properties.insert(
                    "RequireCommonName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_directory_path {
                properties.insert(
                    "RequireDirectoryPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_dns_as_cn {
                properties.insert(
                    "RequireDnsAsCn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_email {
                properties.insert(
                    "RequireEmail".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_directory_guid {
                properties.insert(
                    "SanRequireDirectoryGuid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_dns {
                properties.insert(
                    "SanRequireDns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_domain_dns {
                properties.insert(
                    "SanRequireDomainDns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_email {
                properties.insert(
                    "SanRequireEmail".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_spn {
                properties.insert(
                    "SanRequireSpn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_upn {
                properties.insert(
                    "SanRequireUpn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-subjectnameflagsv4.html
    pub struct SubjectNameFlagsV4_ {
        pub require_common_name: Option<crate::value::ExpBool>,
        pub require_directory_path: Option<crate::value::ExpBool>,
        pub require_dns_as_cn: Option<crate::value::ExpBool>,
        pub require_email: Option<crate::value::ExpBool>,
        pub san_require_directory_guid: Option<crate::value::ExpBool>,
        pub san_require_dns: Option<crate::value::ExpBool>,
        pub san_require_domain_dns: Option<crate::value::ExpBool>,
        pub san_require_email: Option<crate::value::ExpBool>,
        pub san_require_spn: Option<crate::value::ExpBool>,
        pub san_require_upn: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_SubjectNameFlagsV4 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.SubjectNameFlagsV4"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_SubjectNameFlagsV4 as SubjectNameFlagsV4;
    impl crate::value::ToValue for SubjectNameFlagsV4_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.require_common_name {
                properties.insert(
                    "RequireCommonName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_directory_path {
                properties.insert(
                    "RequireDirectoryPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_dns_as_cn {
                properties.insert(
                    "RequireDnsAsCn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_email {
                properties.insert(
                    "RequireEmail".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_directory_guid {
                properties.insert(
                    "SanRequireDirectoryGuid".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_dns {
                properties.insert(
                    "SanRequireDns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_domain_dns {
                properties.insert(
                    "SanRequireDomainDns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_email {
                properties.insert(
                    "SanRequireEmail".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_spn {
                properties.insert(
                    "SanRequireSpn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.san_require_upn {
                properties.insert(
                    "SanRequireUpn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatedefinition.html
    pub struct TemplateDefinition_ {
        pub template_v2: Option<Box<TemplateV2_>>,
        pub template_v3: Option<Box<TemplateV3_>>,
        pub template_v4: Option<Box<TemplateV4_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_TemplateDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.TemplateDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_TemplateDefinition as TemplateDefinition;
    impl crate::value::ToValue for TemplateDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.template_v2 {
                properties.insert(
                    "TemplateV2".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.template_v3 {
                properties.insert(
                    "TemplateV3".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.template_v4 {
                properties.insert(
                    "TemplateV4".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev2.html
    pub struct TemplateV2_ {
        pub certificate_validity: Box<CertificateValidity_>,
        pub enrollment_flags: Box<EnrollmentFlagsV2_>,
        pub extensions: Box<ExtensionsV2_>,
        pub general_flags: Box<GeneralFlagsV2_>,
        pub private_key_attributes: Box<PrivateKeyAttributesV2_>,
        pub private_key_flags: Box<PrivateKeyFlagsV2_>,
        pub subject_name_flags: Box<SubjectNameFlagsV2_>,
        pub superseded_templates: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_TemplateV2 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.TemplateV2"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_TemplateV2 as TemplateV2;
    impl crate::value::ToValue for TemplateV2_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateValidity".to_string(),
                crate::value::ToValue::to_value(&self.certificate_validity),
            );
            properties.insert(
                "EnrollmentFlags".to_string(),
                crate::value::ToValue::to_value(&self.enrollment_flags),
            );
            properties.insert(
                "Extensions".to_string(),
                crate::value::ToValue::to_value(&self.extensions),
            );
            properties.insert(
                "GeneralFlags".to_string(),
                crate::value::ToValue::to_value(&self.general_flags),
            );
            properties.insert(
                "PrivateKeyAttributes".to_string(),
                crate::value::ToValue::to_value(&self.private_key_attributes),
            );
            properties.insert(
                "PrivateKeyFlags".to_string(),
                crate::value::ToValue::to_value(&self.private_key_flags),
            );
            properties.insert(
                "SubjectNameFlags".to_string(),
                crate::value::ToValue::to_value(&self.subject_name_flags),
            );
            if let Some(ref value) = self.superseded_templates {
                properties.insert(
                    "SupersededTemplates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev3.html
    pub struct TemplateV3_ {
        pub certificate_validity: Box<CertificateValidity_>,
        pub enrollment_flags: Box<EnrollmentFlagsV3_>,
        pub extensions: Box<ExtensionsV3_>,
        pub general_flags: Box<GeneralFlagsV3_>,
        pub hash_algorithm: crate::value::ExpString,
        pub private_key_attributes: Box<PrivateKeyAttributesV3_>,
        pub private_key_flags: Box<PrivateKeyFlagsV3_>,
        pub subject_name_flags: Box<SubjectNameFlagsV3_>,
        pub superseded_templates: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_TemplateV3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.TemplateV3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_TemplateV3 as TemplateV3;
    impl crate::value::ToValue for TemplateV3_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateValidity".to_string(),
                crate::value::ToValue::to_value(&self.certificate_validity),
            );
            properties.insert(
                "EnrollmentFlags".to_string(),
                crate::value::ToValue::to_value(&self.enrollment_flags),
            );
            properties.insert(
                "Extensions".to_string(),
                crate::value::ToValue::to_value(&self.extensions),
            );
            properties.insert(
                "GeneralFlags".to_string(),
                crate::value::ToValue::to_value(&self.general_flags),
            );
            properties.insert(
                "HashAlgorithm".to_string(),
                crate::value::ToValue::to_value(&self.hash_algorithm),
            );
            properties.insert(
                "PrivateKeyAttributes".to_string(),
                crate::value::ToValue::to_value(&self.private_key_attributes),
            );
            properties.insert(
                "PrivateKeyFlags".to_string(),
                crate::value::ToValue::to_value(&self.private_key_flags),
            );
            properties.insert(
                "SubjectNameFlags".to_string(),
                crate::value::ToValue::to_value(&self.subject_name_flags),
            );
            if let Some(ref value) = self.superseded_templates {
                properties.insert(
                    "SupersededTemplates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-templatev4.html
    pub struct TemplateV4_ {
        pub certificate_validity: Box<CertificateValidity_>,
        pub enrollment_flags: Box<EnrollmentFlagsV4_>,
        pub extensions: Box<ExtensionsV4_>,
        pub general_flags: Box<GeneralFlagsV4_>,
        pub hash_algorithm: Option<crate::value::ExpString>,
        pub private_key_attributes: Box<PrivateKeyAttributesV4_>,
        pub private_key_flags: Box<PrivateKeyFlagsV4_>,
        pub subject_name_flags: Box<SubjectNameFlagsV4_>,
        pub superseded_templates: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_TemplateV4 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.TemplateV4"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_TemplateV4 as TemplateV4;
    impl crate::value::ToValue for TemplateV4_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateValidity".to_string(),
                crate::value::ToValue::to_value(&self.certificate_validity),
            );
            properties.insert(
                "EnrollmentFlags".to_string(),
                crate::value::ToValue::to_value(&self.enrollment_flags),
            );
            properties.insert(
                "Extensions".to_string(),
                crate::value::ToValue::to_value(&self.extensions),
            );
            properties.insert(
                "GeneralFlags".to_string(),
                crate::value::ToValue::to_value(&self.general_flags),
            );
            if let Some(ref value) = self.hash_algorithm {
                properties.insert(
                    "HashAlgorithm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PrivateKeyAttributes".to_string(),
                crate::value::ToValue::to_value(&self.private_key_attributes),
            );
            properties.insert(
                "PrivateKeyFlags".to_string(),
                crate::value::ToValue::to_value(&self.private_key_flags),
            );
            properties.insert(
                "SubjectNameFlags".to_string(),
                crate::value::ToValue::to_value(&self.subject_name_flags),
            );
            if let Some(ref value) = self.superseded_templates {
                properties.insert(
                    "SupersededTemplates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-template-validityperiod.html
    pub struct ValidityPeriod_ {
        pub period: f64,
        pub period_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_Template_ValidityPeriod {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::Template.ValidityPeriod"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_Template_ValidityPeriod as ValidityPeriod;
    impl crate::value::ToValue for ValidityPeriod_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Period".to_string(),
                crate::value::ToValue::to_value(&self.period),
            );
            properties.insert(
                "PeriodType".to_string(),
                crate::value::ToValue::to_value(&self.period_type),
            );
            properties.into()
        }
    }
}
pub mod templategroupaccesscontrolentry {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorad-templategroupaccesscontrolentry-accessrights.html
    pub struct AccessRights_ {
        pub auto_enroll: Option<crate::value::ExpString>,
        pub enroll: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorad_TemplateGroupAccessControlEntry_AccessRights {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorAD::TemplateGroupAccessControlEntry.AccessRights"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorad_TemplateGroupAccessControlEntry_AccessRights as AccessRights;
    impl crate::value::ToValue for AccessRights_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_enroll {
                properties.insert(
                    "AutoEnroll".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enroll {
                properties.insert("Enroll".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-connector.html
pub struct Connector_ {
    pub certificate_authority_arn: crate::value::ExpString,
    pub directory_id: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub vpc_information: super::pcaconnectorad::connector::VpcInformation_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pcaconnectorad_Connector {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::PCAConnectorAD::Connector"
        $($field $value)*)
    };
}
pub use crate::__aws_pcaconnectorad_Connector as Connector;
impl crate::template::ToResource for Connector_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("PCAConnectorAD"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Connector"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CertificateAuthorityArn".to_string(),
            crate::value::ToValue::to_value(&self.certificate_authority_arn),
        );
        properties.insert(
            "DirectoryId".to_string(),
            crate::value::ToValue::to_value(&self.directory_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcInformation".to_string(),
            crate::value::ToValue::to_value(&self.vpc_information),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-directoryregistration.html
pub struct DirectoryRegistration_ {
    pub directory_id: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pcaconnectorad_DirectoryRegistration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::PCAConnectorAD::DirectoryRegistration"
        $($field $value)*)
    };
}
pub use crate::__aws_pcaconnectorad_DirectoryRegistration as DirectoryRegistration;
impl crate::template::ToResource for DirectoryRegistration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("PCAConnectorAD"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DirectoryRegistration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DirectoryId".to_string(),
            crate::value::ToValue::to_value(&self.directory_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-serviceprincipalname.html
pub struct ServicePrincipalName_ {
    pub connector_arn: Option<crate::value::ExpString>,
    pub directory_registration_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pcaconnectorad_ServicePrincipalName {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::PCAConnectorAD::ServicePrincipalName"
        $($field $value)*)
    };
}
pub use crate::__aws_pcaconnectorad_ServicePrincipalName as ServicePrincipalName;
impl crate::template::ToResource for ServicePrincipalName_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("PCAConnectorAD"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ServicePrincipalName"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.connector_arn {
            properties.insert(
                "ConnectorArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.directory_registration_arn {
            properties.insert(
                "DirectoryRegistrationArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-template.html
pub struct Template_ {
    pub connector_arn: crate::value::ExpString,
    pub definition: super::pcaconnectorad::template::TemplateDefinition_,
    pub name: crate::value::ExpString,
    pub reenroll_all_certificate_holders: Option<crate::value::ExpBool>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pcaconnectorad_Template {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::PCAConnectorAD::Template"
        $($field $value)*)
    };
}
pub use crate::__aws_pcaconnectorad_Template as Template;
impl crate::template::ToResource for Template_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("PCAConnectorAD"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Template"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConnectorArn".to_string(),
            crate::value::ToValue::to_value(&self.connector_arn),
        );
        properties.insert(
            "Definition".to_string(),
            crate::value::ToValue::to_value(&self.definition),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.reenroll_all_certificate_holders {
            properties.insert(
                "ReenrollAllCertificateHolders".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorad-templategroupaccesscontrolentry.html
pub struct TemplateGroupAccessControlEntry_ {
    pub access_rights: super::pcaconnectorad::templategroupaccesscontrolentry::AccessRights_,
    pub group_display_name: crate::value::ExpString,
    pub group_security_identifier: Option<crate::value::ExpString>,
    pub template_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pcaconnectorad_TemplateGroupAccessControlEntry {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::PCAConnectorAD::TemplateGroupAccessControlEntry"
        $($field $value)*)
    };
}
pub use crate::__aws_pcaconnectorad_TemplateGroupAccessControlEntry as TemplateGroupAccessControlEntry;
impl crate::template::ToResource for TemplateGroupAccessControlEntry_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("PCAConnectorAD"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "TemplateGroupAccessControlEntry",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AccessRights".to_string(),
            crate::value::ToValue::to_value(&self.access_rights),
        );
        properties.insert(
            "GroupDisplayName".to_string(),
            crate::value::ToValue::to_value(&self.group_display_name),
        );
        if let Some(ref value) = self.group_security_identifier {
            properties.insert(
                "GroupSecurityIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.template_arn {
            properties.insert(
                "TemplateArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
