pub mod connector {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorscep-connector-intuneconfiguration.html
    pub struct IntuneConfiguration_ {
        pub azure_application_id: crate::value::ExpString,
        pub domain: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorscep_Connector_IntuneConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorSCEP::Connector.IntuneConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorscep_Connector_IntuneConfiguration as IntuneConfiguration;
    impl crate::value::ToValue for IntuneConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AzureApplicationId".to_string(),
                crate::value::ToValue::to_value(&self.azure_application_id),
            );
            properties.insert(
                "Domain".to_string(),
                crate::value::ToValue::to_value(&self.domain),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorscep-connector-mobiledevicemanagement.html
    pub struct MobileDeviceManagement_ {
        pub intune: Box<IntuneConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorscep_Connector_MobileDeviceManagement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorSCEP::Connector.MobileDeviceManagement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorscep_Connector_MobileDeviceManagement as MobileDeviceManagement;
    impl crate::value::ToValue for MobileDeviceManagement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Intune".to_string(),
                crate::value::ToValue::to_value(&self.intune),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pcaconnectorscep-connector-openidconfiguration.html
    pub struct OpenIdConfiguration_ {
        pub audience: Option<crate::value::ExpString>,
        pub issuer: Option<crate::value::ExpString>,
        pub subject: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pcaconnectorscep_Connector_OpenIdConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::PCAConnectorSCEP::Connector.OpenIdConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pcaconnectorscep_Connector_OpenIdConfiguration as OpenIdConfiguration;
    impl crate::value::ToValue for OpenIdConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audience {
                properties.insert(
                    "Audience".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.issuer {
                properties.insert("Issuer".to_string(), crate::value::ToValue::to_value(value));
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
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorscep-challenge.html
pub struct Challenge_ {
    pub connector_arn: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pcaconnectorscep_Challenge {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::PCAConnectorSCEP::Challenge"
        $($field $value)*)
    };
}
pub use crate::__aws_pcaconnectorscep_Challenge as Challenge;
impl crate::template::ToResource for Challenge_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("PCAConnectorSCEP"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Challenge"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConnectorArn".to_string(),
            crate::value::ToValue::to_value(&self.connector_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pcaconnectorscep-connector.html
pub struct Connector_ {
    pub certificate_authority_arn: crate::value::ExpString,
    pub mobile_device_management:
        Option<super::pcaconnectorscep::connector::MobileDeviceManagement_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pcaconnectorscep_Connector {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::PCAConnectorSCEP::Connector"
        $($field $value)*)
    };
}
pub use crate::__aws_pcaconnectorscep_Connector as Connector;
impl crate::template::ToResource for Connector_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("PCAConnectorSCEP"),
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
        if let Some(ref value) = self.mobile_device_management {
            properties.insert(
                "MobileDeviceManagement".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
