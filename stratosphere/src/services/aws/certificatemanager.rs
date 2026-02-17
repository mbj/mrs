pub mod account {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-certificatemanager-account-expiryeventsconfiguration.html>
    pub struct ExpiryEventsConfiguration_ {
        pub days_before_expiry: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_certificatemanager_Account_ExpiryEventsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CertificateManager::Account.ExpiryEventsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_certificatemanager_Account_ExpiryEventsConfiguration as ExpiryEventsConfiguration;
    impl crate::value::ToValue for ExpiryEventsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.days_before_expiry {
                properties.insert(
                    "DaysBeforeExpiry".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod certificate {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-certificatemanager-certificate-domainvalidationoption.html>
    pub struct DomainValidationOption_ {
        pub domain_name: crate::value::ExpString,
        pub hosted_zone_id: Option<crate::value::ExpString>,
        pub validation_domain: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_certificatemanager_Certificate_DomainValidationOption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CertificateManager::Certificate.DomainValidationOption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_certificatemanager_Certificate_DomainValidationOption as DomainValidationOption;
    impl crate::value::ToValue for DomainValidationOption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DomainName".to_string(),
                crate::value::ToValue::to_value(&self.domain_name),
            );
            if let Some(ref value) = self.hosted_zone_id {
                properties.insert(
                    "HostedZoneId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.validation_domain {
                properties.insert(
                    "ValidationDomain".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-account.html>
pub struct Account_ {
    pub expiry_events_configuration: super::certificatemanager::account::ExpiryEventsConfiguration_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_certificatemanager_Account {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CertificateManager::Account"
        $($field $value)*)
    };
}
pub use crate::__aws_certificatemanager_Account as Account;
impl crate::template::ToResource for Account_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CertificateManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Account"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ExpiryEventsConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.expiry_events_configuration),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-certificate.html>
pub struct Certificate_ {
    pub certificate_authority_arn: Option<crate::value::ExpString>,
    pub certificate_export: Option<crate::value::ExpString>,
    pub certificate_transparency_logging_preference: Option<crate::value::ExpString>,
    pub domain_name: crate::value::ExpString,
    pub domain_validation_options:
        Option<Vec<super::certificatemanager::certificate::DomainValidationOption_>>,
    pub key_algorithm: Option<crate::value::ExpString>,
    pub subject_alternative_names: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub validation_method: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_certificatemanager_Certificate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CertificateManager::Certificate"
        $($field $value)*)
    };
}
pub use crate::__aws_certificatemanager_Certificate as Certificate;
impl crate::template::ToResource for Certificate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CertificateManager"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Certificate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.certificate_authority_arn {
            properties.insert(
                "CertificateAuthorityArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate_export {
            properties.insert(
                "CertificateExport".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate_transparency_logging_preference {
            properties.insert(
                "CertificateTransparencyLoggingPreference".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        if let Some(ref value) = self.domain_validation_options {
            properties.insert(
                "DomainValidationOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.key_algorithm {
            properties.insert(
                "KeyAlgorithm".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subject_alternative_names {
            properties.insert(
                "SubjectAlternativeNames".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.validation_method {
            properties.insert(
                "ValidationMethod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
