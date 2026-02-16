pub mod certificate {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-apipassthrough.html
    pub struct ApiPassthrough_ {
        pub extensions: Option<Box<Extensions_>>,
        pub subject: Option<Box<Subject_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_Certificate_ApiPassthrough {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::Certificate.ApiPassthrough"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_Certificate_ApiPassthrough as ApiPassthrough;
    impl crate::value::ToValue for ApiPassthrough_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.extensions {
                properties.insert(
                    "Extensions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subject {
                properties.insert(
                    "Subject".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-customattribute.html
    pub struct CustomAttribute_ {
        pub object_identifier: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_Certificate_CustomAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::Certificate.CustomAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_Certificate_CustomAttribute as CustomAttribute;
    impl crate::value::ToValue for CustomAttribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ObjectIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.object_identifier),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-customextension.html
    pub struct CustomExtension_ {
        pub critical: Option<crate::value::ExpBool>,
        pub object_identifier: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_Certificate_CustomExtension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::Certificate.CustomExtension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_Certificate_CustomExtension as CustomExtension;
    impl crate::value::ToValue for CustomExtension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.critical {
                properties.insert(
                    "Critical".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ObjectIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.object_identifier),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-edipartyname.html
    pub struct EdiPartyName_ {
        pub name_assigner: crate::value::ExpString,
        pub party_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_Certificate_EdiPartyName {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::Certificate.EdiPartyName"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_Certificate_EdiPartyName as EdiPartyName;
    impl crate::value::ToValue for EdiPartyName_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "NameAssigner".to_string(),
                crate::value::ToValue::to_value(&self.name_assigner),
            );
            properties.insert(
                "PartyName".to_string(),
                crate::value::ToValue::to_value(&self.party_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-extendedkeyusage.html
    pub struct ExtendedKeyUsage_ {
        pub extended_key_usage_object_identifier: Option<crate::value::ExpString>,
        pub extended_key_usage_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_Certificate_ExtendedKeyUsage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::Certificate.ExtendedKeyUsage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_Certificate_ExtendedKeyUsage as ExtendedKeyUsage;
    impl crate::value::ToValue for ExtendedKeyUsage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.extended_key_usage_object_identifier {
                properties.insert(
                    "ExtendedKeyUsageObjectIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.extended_key_usage_type {
                properties.insert(
                    "ExtendedKeyUsageType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-extensions.html
    pub struct Extensions_ {
        pub certificate_policies: Option<Vec<PolicyInformation_>>,
        pub custom_extensions: Option<Vec<CustomExtension_>>,
        pub extended_key_usage: Option<Vec<ExtendedKeyUsage_>>,
        pub key_usage: Option<Box<KeyUsage_>>,
        pub subject_alternative_names: Option<Vec<GeneralName_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_Certificate_Extensions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::Certificate.Extensions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_Certificate_Extensions as Extensions;
    impl crate::value::ToValue for Extensions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_policies {
                properties.insert(
                    "CertificatePolicies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_extensions {
                properties.insert(
                    "CustomExtensions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.extended_key_usage {
                properties.insert(
                    "ExtendedKeyUsage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_usage {
                properties.insert(
                    "KeyUsage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subject_alternative_names {
                properties.insert(
                    "SubjectAlternativeNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-generalname.html
    pub struct GeneralName_ {
        pub directory_name: Option<Box<Subject_>>,
        pub dns_name: Option<crate::value::ExpString>,
        pub edi_party_name: Option<Box<EdiPartyName_>>,
        pub ip_address: Option<crate::value::ExpString>,
        pub other_name: Option<Box<OtherName_>>,
        pub registered_id: Option<crate::value::ExpString>,
        pub rfc822_name: Option<crate::value::ExpString>,
        pub uniform_resource_identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_Certificate_GeneralName {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::Certificate.GeneralName"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_Certificate_GeneralName as GeneralName;
    impl crate::value::ToValue for GeneralName_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.directory_name {
                properties.insert(
                    "DirectoryName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dns_name {
                properties.insert(
                    "DnsName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.edi_party_name {
                properties.insert(
                    "EdiPartyName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ip_address {
                properties.insert(
                    "IpAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.other_name {
                properties.insert(
                    "OtherName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.registered_id {
                properties.insert(
                    "RegisteredId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rfc822_name {
                properties.insert(
                    "Rfc822Name".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uniform_resource_identifier {
                properties.insert(
                    "UniformResourceIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-keyusage.html
    pub struct KeyUsage_ {
        pub crl_sign: Option<crate::value::ExpBool>,
        pub data_encipherment: Option<crate::value::ExpBool>,
        pub decipher_only: Option<crate::value::ExpBool>,
        pub digital_signature: Option<crate::value::ExpBool>,
        pub encipher_only: Option<crate::value::ExpBool>,
        pub key_agreement: Option<crate::value::ExpBool>,
        pub key_cert_sign: Option<crate::value::ExpBool>,
        pub key_encipherment: Option<crate::value::ExpBool>,
        pub non_repudiation: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_Certificate_KeyUsage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::Certificate.KeyUsage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_Certificate_KeyUsage as KeyUsage;
    impl crate::value::ToValue for KeyUsage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.crl_sign {
                properties.insert(
                    "CRLSign".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_encipherment {
                properties.insert(
                    "DataEncipherment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.decipher_only {
                properties.insert(
                    "DecipherOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.digital_signature {
                properties.insert(
                    "DigitalSignature".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encipher_only {
                properties.insert(
                    "EncipherOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_agreement {
                properties.insert(
                    "KeyAgreement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_cert_sign {
                properties.insert(
                    "KeyCertSign".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-othername.html
    pub struct OtherName_ {
        pub type_id: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_Certificate_OtherName {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::Certificate.OtherName"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_Certificate_OtherName as OtherName;
    impl crate::value::ToValue for OtherName_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TypeId".to_string(),
                crate::value::ToValue::to_value(&self.type_id),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-policyinformation.html
    pub struct PolicyInformation_ {
        pub cert_policy_id: crate::value::ExpString,
        pub policy_qualifiers: Option<Vec<PolicyQualifierInfo_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_Certificate_PolicyInformation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::Certificate.PolicyInformation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_Certificate_PolicyInformation as PolicyInformation;
    impl crate::value::ToValue for PolicyInformation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertPolicyId".to_string(),
                crate::value::ToValue::to_value(&self.cert_policy_id),
            );
            if let Some(ref value) = self.policy_qualifiers {
                properties.insert(
                    "PolicyQualifiers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-policyqualifierinfo.html
    pub struct PolicyQualifierInfo_ {
        pub policy_qualifier_id: crate::value::ExpString,
        pub qualifier: Box<Qualifier_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_Certificate_PolicyQualifierInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::Certificate.PolicyQualifierInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_Certificate_PolicyQualifierInfo as PolicyQualifierInfo;
    impl crate::value::ToValue for PolicyQualifierInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PolicyQualifierId".to_string(),
                crate::value::ToValue::to_value(&self.policy_qualifier_id),
            );
            properties.insert(
                "Qualifier".to_string(),
                crate::value::ToValue::to_value(&self.qualifier),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-qualifier.html
    pub struct Qualifier_ {
        pub cps_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_Certificate_Qualifier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::Certificate.Qualifier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_Certificate_Qualifier as Qualifier;
    impl crate::value::ToValue for Qualifier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CpsUri".to_string(),
                crate::value::ToValue::to_value(&self.cps_uri),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-subject.html
    pub struct Subject_ {
        pub common_name: Option<crate::value::ExpString>,
        pub country: Option<crate::value::ExpString>,
        pub custom_attributes: Option<Vec<CustomAttribute_>>,
        pub distinguished_name_qualifier: Option<crate::value::ExpString>,
        pub generation_qualifier: Option<crate::value::ExpString>,
        pub given_name: Option<crate::value::ExpString>,
        pub initials: Option<crate::value::ExpString>,
        pub locality: Option<crate::value::ExpString>,
        pub organization: Option<crate::value::ExpString>,
        pub organizational_unit: Option<crate::value::ExpString>,
        pub pseudonym: Option<crate::value::ExpString>,
        pub serial_number: Option<crate::value::ExpString>,
        pub state: Option<crate::value::ExpString>,
        pub surname: Option<crate::value::ExpString>,
        pub title: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_Certificate_Subject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::Certificate.Subject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_Certificate_Subject as Subject;
    impl crate::value::ToValue for Subject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.common_name {
                properties.insert(
                    "CommonName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.country {
                properties.insert(
                    "Country".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_attributes {
                properties.insert(
                    "CustomAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.distinguished_name_qualifier {
                properties.insert(
                    "DistinguishedNameQualifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.generation_qualifier {
                properties.insert(
                    "GenerationQualifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.given_name {
                properties.insert(
                    "GivenName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.initials {
                properties.insert(
                    "Initials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.locality {
                properties.insert(
                    "Locality".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.organization {
                properties.insert(
                    "Organization".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.organizational_unit {
                properties.insert(
                    "OrganizationalUnit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pseudonym {
                properties.insert(
                    "Pseudonym".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.serial_number {
                properties.insert(
                    "SerialNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.surname {
                properties.insert(
                    "Surname".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.title {
                properties.insert("Title".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificate-validity.html
    pub struct Validity_ {
        pub r#type: crate::value::ExpString,
        pub value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_Certificate_Validity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::Certificate.Validity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_Certificate_Validity as Validity;
    impl crate::value::ToValue for Validity_ {
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
}
pub mod certificateauthority {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-accessdescription.html
    pub struct AccessDescription_ {
        pub access_location: Box<GeneralName_>,
        pub access_method: Box<AccessMethod_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_CertificateAuthority_AccessDescription {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::CertificateAuthority.AccessDescription"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_CertificateAuthority_AccessDescription as AccessDescription;
    impl crate::value::ToValue for AccessDescription_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccessLocation".to_string(),
                crate::value::ToValue::to_value(&self.access_location),
            );
            properties.insert(
                "AccessMethod".to_string(),
                crate::value::ToValue::to_value(&self.access_method),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-accessmethod.html
    pub struct AccessMethod_ {
        pub access_method_type: Option<crate::value::ExpString>,
        pub custom_object_identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_CertificateAuthority_AccessMethod {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::CertificateAuthority.AccessMethod"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_CertificateAuthority_AccessMethod as AccessMethod;
    impl crate::value::ToValue for AccessMethod_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_method_type {
                properties.insert(
                    "AccessMethodType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_object_identifier {
                properties.insert(
                    "CustomObjectIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-crlconfiguration.html
    pub struct CrlConfiguration_ {
        pub crl_distribution_point_extension_configuration:
            Option<Box<CrlDistributionPointExtensionConfiguration_>>,
        pub crl_type: Option<crate::value::ExpString>,
        pub custom_cname: Option<crate::value::ExpString>,
        pub custom_path: Option<crate::value::ExpString>,
        pub enabled: crate::value::ExpBool,
        pub expiration_in_days: Option<i32>,
        pub s3_bucket_name: Option<crate::value::ExpString>,
        pub s3_object_acl: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_CertificateAuthority_CrlConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::CertificateAuthority.CrlConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_CertificateAuthority_CrlConfiguration as CrlConfiguration;
    impl crate::value::ToValue for CrlConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.crl_distribution_point_extension_configuration {
                properties.insert(
                    "CrlDistributionPointExtensionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.crl_type {
                properties.insert(
                    "CrlType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_cname {
                properties.insert(
                    "CustomCname".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_path {
                properties.insert(
                    "CustomPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.expiration_in_days {
                properties.insert(
                    "ExpirationInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_bucket_name {
                properties.insert(
                    "S3BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_object_acl {
                properties.insert(
                    "S3ObjectAcl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-crldistributionpointextensionconfiguration.html
    pub struct CrlDistributionPointExtensionConfiguration_ {
        pub omit_extension: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_CertificateAuthority_CrlDistributionPointExtensionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::CertificateAuthority.CrlDistributionPointExtensionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_CertificateAuthority_CrlDistributionPointExtensionConfiguration as CrlDistributionPointExtensionConfiguration;
    impl crate::value::ToValue for CrlDistributionPointExtensionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OmitExtension".to_string(),
                crate::value::ToValue::to_value(&self.omit_extension),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-csrextensions.html
    pub struct CsrExtensions_ {
        pub key_usage: Option<Box<KeyUsage_>>,
        pub subject_information_access: Option<Vec<AccessDescription_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_CertificateAuthority_CsrExtensions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::CertificateAuthority.CsrExtensions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_CertificateAuthority_CsrExtensions as CsrExtensions;
    impl crate::value::ToValue for CsrExtensions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key_usage {
                properties.insert(
                    "KeyUsage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subject_information_access {
                properties.insert(
                    "SubjectInformationAccess".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-customattribute.html
    pub struct CustomAttribute_ {
        pub object_identifier: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_CertificateAuthority_CustomAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::CertificateAuthority.CustomAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_CertificateAuthority_CustomAttribute as CustomAttribute;
    impl crate::value::ToValue for CustomAttribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ObjectIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.object_identifier),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-edipartyname.html
    pub struct EdiPartyName_ {
        pub name_assigner: Option<crate::value::ExpString>,
        pub party_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_CertificateAuthority_EdiPartyName {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::CertificateAuthority.EdiPartyName"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_CertificateAuthority_EdiPartyName as EdiPartyName;
    impl crate::value::ToValue for EdiPartyName_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name_assigner {
                properties.insert(
                    "NameAssigner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PartyName".to_string(),
                crate::value::ToValue::to_value(&self.party_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-generalname.html
    pub struct GeneralName_ {
        pub directory_name: Option<Box<Subject_>>,
        pub dns_name: Option<crate::value::ExpString>,
        pub edi_party_name: Option<Box<EdiPartyName_>>,
        pub ip_address: Option<crate::value::ExpString>,
        pub other_name: Option<Box<OtherName_>>,
        pub registered_id: Option<crate::value::ExpString>,
        pub rfc822_name: Option<crate::value::ExpString>,
        pub uniform_resource_identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_CertificateAuthority_GeneralName {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::CertificateAuthority.GeneralName"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_CertificateAuthority_GeneralName as GeneralName;
    impl crate::value::ToValue for GeneralName_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.directory_name {
                properties.insert(
                    "DirectoryName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dns_name {
                properties.insert(
                    "DnsName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.edi_party_name {
                properties.insert(
                    "EdiPartyName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ip_address {
                properties.insert(
                    "IpAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.other_name {
                properties.insert(
                    "OtherName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.registered_id {
                properties.insert(
                    "RegisteredId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rfc822_name {
                properties.insert(
                    "Rfc822Name".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uniform_resource_identifier {
                properties.insert(
                    "UniformResourceIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-keyusage.html
    pub struct KeyUsage_ {
        pub crl_sign: Option<crate::value::ExpBool>,
        pub data_encipherment: Option<crate::value::ExpBool>,
        pub decipher_only: Option<crate::value::ExpBool>,
        pub digital_signature: Option<crate::value::ExpBool>,
        pub encipher_only: Option<crate::value::ExpBool>,
        pub key_agreement: Option<crate::value::ExpBool>,
        pub key_cert_sign: Option<crate::value::ExpBool>,
        pub key_encipherment: Option<crate::value::ExpBool>,
        pub non_repudiation: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_CertificateAuthority_KeyUsage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::CertificateAuthority.KeyUsage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_CertificateAuthority_KeyUsage as KeyUsage;
    impl crate::value::ToValue for KeyUsage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.crl_sign {
                properties.insert(
                    "CRLSign".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_encipherment {
                properties.insert(
                    "DataEncipherment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.decipher_only {
                properties.insert(
                    "DecipherOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.digital_signature {
                properties.insert(
                    "DigitalSignature".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encipher_only {
                properties.insert(
                    "EncipherOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_agreement {
                properties.insert(
                    "KeyAgreement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_cert_sign {
                properties.insert(
                    "KeyCertSign".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-ocspconfiguration.html
    pub struct OcspConfiguration_ {
        pub enabled: crate::value::ExpBool,
        pub ocsp_custom_cname: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_CertificateAuthority_OcspConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::CertificateAuthority.OcspConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_CertificateAuthority_OcspConfiguration as OcspConfiguration;
    impl crate::value::ToValue for OcspConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.ocsp_custom_cname {
                properties.insert(
                    "OcspCustomCname".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-othername.html
    pub struct OtherName_ {
        pub type_id: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_CertificateAuthority_OtherName {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::CertificateAuthority.OtherName"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_CertificateAuthority_OtherName as OtherName;
    impl crate::value::ToValue for OtherName_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TypeId".to_string(),
                crate::value::ToValue::to_value(&self.type_id),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-revocationconfiguration.html
    pub struct RevocationConfiguration_ {
        pub crl_configuration: Option<Box<CrlConfiguration_>>,
        pub ocsp_configuration: Option<Box<OcspConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_CertificateAuthority_RevocationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::CertificateAuthority.RevocationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_CertificateAuthority_RevocationConfiguration as RevocationConfiguration;
    impl crate::value::ToValue for RevocationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.crl_configuration {
                properties.insert(
                    "CrlConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ocsp_configuration {
                properties.insert(
                    "OcspConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-acmpca-certificateauthority-subject.html
    pub struct Subject_ {
        pub common_name: Option<crate::value::ExpString>,
        pub country: Option<crate::value::ExpString>,
        pub custom_attributes: Option<Vec<CustomAttribute_>>,
        pub distinguished_name_qualifier: Option<crate::value::ExpString>,
        pub generation_qualifier: Option<crate::value::ExpString>,
        pub given_name: Option<crate::value::ExpString>,
        pub initials: Option<crate::value::ExpString>,
        pub locality: Option<crate::value::ExpString>,
        pub organization: Option<crate::value::ExpString>,
        pub organizational_unit: Option<crate::value::ExpString>,
        pub pseudonym: Option<crate::value::ExpString>,
        pub serial_number: Option<crate::value::ExpString>,
        pub state: Option<crate::value::ExpString>,
        pub surname: Option<crate::value::ExpString>,
        pub title: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_acmpca_CertificateAuthority_Subject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ACMPCA::CertificateAuthority.Subject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_acmpca_CertificateAuthority_Subject as Subject;
    impl crate::value::ToValue for Subject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.common_name {
                properties.insert(
                    "CommonName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.country {
                properties.insert(
                    "Country".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_attributes {
                properties.insert(
                    "CustomAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.distinguished_name_qualifier {
                properties.insert(
                    "DistinguishedNameQualifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.generation_qualifier {
                properties.insert(
                    "GenerationQualifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.given_name {
                properties.insert(
                    "GivenName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.initials {
                properties.insert(
                    "Initials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.locality {
                properties.insert(
                    "Locality".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.organization {
                properties.insert(
                    "Organization".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.organizational_unit {
                properties.insert(
                    "OrganizationalUnit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pseudonym {
                properties.insert(
                    "Pseudonym".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.serial_number {
                properties.insert(
                    "SerialNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.surname {
                properties.insert(
                    "Surname".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.title {
                properties.insert("Title".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificate.html
pub struct Certificate_ {
    pub api_passthrough: Option<super::acmpca::certificate::ApiPassthrough_>,
    pub certificate_authority_arn: crate::value::ExpString,
    pub certificate_signing_request: crate::value::ExpString,
    pub signing_algorithm: crate::value::ExpString,
    pub template_arn: Option<crate::value::ExpString>,
    pub validity: super::acmpca::certificate::Validity_,
    pub validity_not_before: Option<super::acmpca::certificate::Validity_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_acmpca_Certificate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ACMPCA::Certificate"
        $($field $value)*)
    };
}
pub use crate::__aws_acmpca_Certificate as Certificate;
impl crate::template::ToResource for Certificate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ACMPCA"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Certificate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.api_passthrough {
            properties.insert(
                "ApiPassthrough".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "CertificateAuthorityArn".to_string(),
            crate::value::ToValue::to_value(&self.certificate_authority_arn),
        );
        properties.insert(
            "CertificateSigningRequest".to_string(),
            crate::value::ToValue::to_value(&self.certificate_signing_request),
        );
        properties.insert(
            "SigningAlgorithm".to_string(),
            crate::value::ToValue::to_value(&self.signing_algorithm),
        );
        if let Some(ref value) = self.template_arn {
            properties.insert(
                "TemplateArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Validity".to_string(),
            crate::value::ToValue::to_value(&self.validity),
        );
        if let Some(ref value) = self.validity_not_before {
            properties.insert(
                "ValidityNotBefore".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificateauthority.html
pub struct CertificateAuthority_ {
    pub csr_extensions: Option<super::acmpca::certificateauthority::CsrExtensions_>,
    pub key_algorithm: crate::value::ExpString,
    pub key_storage_security_standard: Option<crate::value::ExpString>,
    pub revocation_configuration:
        Option<super::acmpca::certificateauthority::RevocationConfiguration_>,
    pub signing_algorithm: crate::value::ExpString,
    pub subject: super::acmpca::certificateauthority::Subject_,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
    pub usage_mode: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_acmpca_CertificateAuthority {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ACMPCA::CertificateAuthority"
        $($field $value)*)
    };
}
pub use crate::__aws_acmpca_CertificateAuthority as CertificateAuthority;
impl crate::template::ToResource for CertificateAuthority_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ACMPCA"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CertificateAuthority"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.csr_extensions {
            properties.insert(
                "CsrExtensions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "KeyAlgorithm".to_string(),
            crate::value::ToValue::to_value(&self.key_algorithm),
        );
        if let Some(ref value) = self.key_storage_security_standard {
            properties.insert(
                "KeyStorageSecurityStandard".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.revocation_configuration {
            properties.insert(
                "RevocationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SigningAlgorithm".to_string(),
            crate::value::ToValue::to_value(&self.signing_algorithm),
        );
        properties.insert(
            "Subject".to_string(),
            crate::value::ToValue::to_value(&self.subject),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        if let Some(ref value) = self.usage_mode {
            properties.insert(
                "UsageMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-certificateauthorityactivation.html
pub struct CertificateAuthorityActivation_ {
    pub certificate: crate::value::ExpString,
    pub certificate_authority_arn: crate::value::ExpString,
    pub certificate_chain: Option<crate::value::ExpString>,
    pub status: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_acmpca_CertificateAuthorityActivation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ACMPCA::CertificateAuthorityActivation"
        $($field $value)*)
    };
}
pub use crate::__aws_acmpca_CertificateAuthorityActivation as CertificateAuthorityActivation;
impl crate::template::ToResource for CertificateAuthorityActivation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ACMPCA"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "CertificateAuthorityActivation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Certificate".to_string(),
            crate::value::ToValue::to_value(&self.certificate),
        );
        properties.insert(
            "CertificateAuthorityArn".to_string(),
            crate::value::ToValue::to_value(&self.certificate_authority_arn),
        );
        if let Some(ref value) = self.certificate_chain {
            properties.insert(
                "CertificateChain".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-acmpca-permission.html
pub struct Permission_ {
    pub actions: Vec<crate::value::ExpString>,
    pub certificate_authority_arn: crate::value::ExpString,
    pub principal: crate::value::ExpString,
    pub source_account: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_acmpca_Permission {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ACMPCA::Permission"
        $($field $value)*)
    };
}
pub use crate::__aws_acmpca_Permission as Permission;
impl crate::template::ToResource for Permission_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ACMPCA"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Permission"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Actions".to_string(),
            crate::value::ToValue::to_value(&self.actions),
        );
        properties.insert(
            "CertificateAuthorityArn".to_string(),
            crate::value::ToValue::to_value(&self.certificate_authority_arn),
        );
        properties.insert(
            "Principal".to_string(),
            crate::value::ToValue::to_value(&self.principal),
        );
        if let Some(ref value) = self.source_account {
            properties.insert(
                "SourceAccount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
