pub mod environment {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-finspace-environment-attributemapitems.html>
    pub struct AttributeMapItems_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_finspace_Environment_AttributeMapItems {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FinSpace::Environment.AttributeMapItems"
            $($field $value)*)
        };
    }
    pub use crate::__aws_finspace_Environment_AttributeMapItems as AttributeMapItems;
    impl crate::value::ToValue for AttributeMapItems_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-finspace-environment-federationparameters.html>
    pub struct FederationParameters_ {
        pub application_call_back_url: Option<crate::value::ExpString>,
        pub attribute_map: Option<Vec<AttributeMapItems_>>,
        pub federation_provider_name: Option<crate::value::ExpString>,
        pub federation_urn: Option<crate::value::ExpString>,
        pub saml_metadata_document: Option<crate::value::ExpString>,
        pub saml_metadata_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_finspace_Environment_FederationParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FinSpace::Environment.FederationParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_finspace_Environment_FederationParameters as FederationParameters;
    impl crate::value::ToValue for FederationParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_call_back_url {
                properties.insert(
                    "ApplicationCallBackURL".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.attribute_map {
                properties.insert(
                    "AttributeMap".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.federation_provider_name {
                properties.insert(
                    "FederationProviderName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.federation_urn {
                properties.insert(
                    "FederationURN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.saml_metadata_document {
                properties.insert(
                    "SamlMetadataDocument".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.saml_metadata_url {
                properties.insert(
                    "SamlMetadataURL".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-finspace-environment-superuserparameters.html>
    pub struct SuperuserParameters_ {
        pub email_address: Option<crate::value::ExpString>,
        pub first_name: Option<crate::value::ExpString>,
        pub last_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_finspace_Environment_SuperuserParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FinSpace::Environment.SuperuserParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_finspace_Environment_SuperuserParameters as SuperuserParameters;
    impl crate::value::ToValue for SuperuserParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.email_address {
                properties.insert(
                    "EmailAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.first_name {
                properties.insert(
                    "FirstName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.last_name {
                properties.insert(
                    "LastName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-finspace-environment.html>
pub struct Environment_ {
    pub description: Option<crate::value::ExpString>,
    pub federation_mode: Option<crate::value::ExpString>,
    pub federation_parameters: Option<super::finspace::environment::FederationParameters_>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub superuser_parameters: Option<super::finspace::environment::SuperuserParameters_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_finspace_Environment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::FinSpace::Environment"
        $($field $value)*)
    };
}
pub use crate::__aws_finspace_Environment as Environment;
impl crate::template::ToResource for Environment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FinSpace"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Environment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.federation_mode {
            properties.insert(
                "FederationMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.federation_parameters {
            properties.insert(
                "FederationParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.superuser_parameters {
            properties.insert(
                "SuperuserParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
