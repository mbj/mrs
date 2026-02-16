pub mod fhirdatastore {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-healthlake-fhirdatastore-createdat.html
    pub struct CreatedAt_ {
        pub nanos: i32,
        pub seconds: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_healthlake_FHIRDatastore_CreatedAt {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::HealthLake::FHIRDatastore.CreatedAt"
            $($field $value)*)
        };
    }
    pub use crate::__aws_healthlake_FHIRDatastore_CreatedAt as CreatedAt;
    impl crate::value::ToValue for CreatedAt_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Nanos".to_string(),
                crate::value::ToValue::to_value(&self.nanos),
            );
            properties.insert(
                "Seconds".to_string(),
                crate::value::ToValue::to_value(&self.seconds),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-healthlake-fhirdatastore-identityproviderconfiguration.html
    pub struct IdentityProviderConfiguration_ {
        pub authorization_strategy: crate::value::ExpString,
        pub fine_grained_authorization_enabled: Option<crate::value::ExpBool>,
        pub idp_lambda_arn: Option<crate::value::ExpString>,
        pub metadata: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_healthlake_FHIRDatastore_IdentityProviderConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::HealthLake::FHIRDatastore.IdentityProviderConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_healthlake_FHIRDatastore_IdentityProviderConfiguration as IdentityProviderConfiguration;
    impl crate::value::ToValue for IdentityProviderConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthorizationStrategy".to_string(),
                crate::value::ToValue::to_value(&self.authorization_strategy),
            );
            if let Some(ref value) = self.fine_grained_authorization_enabled {
                properties.insert(
                    "FineGrainedAuthorizationEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.idp_lambda_arn {
                properties.insert(
                    "IdpLambdaArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metadata {
                properties.insert(
                    "Metadata".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-healthlake-fhirdatastore-kmsencryptionconfig.html
    pub struct KmsEncryptionConfig_ {
        pub cmk_type: crate::value::ExpString,
        pub kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_healthlake_FHIRDatastore_KmsEncryptionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::HealthLake::FHIRDatastore.KmsEncryptionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_healthlake_FHIRDatastore_KmsEncryptionConfig as KmsEncryptionConfig;
    impl crate::value::ToValue for KmsEncryptionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CmkType".to_string(),
                crate::value::ToValue::to_value(&self.cmk_type),
            );
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-healthlake-fhirdatastore-preloaddataconfig.html
    pub struct PreloadDataConfig_ {
        pub preload_data_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_healthlake_FHIRDatastore_PreloadDataConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::HealthLake::FHIRDatastore.PreloadDataConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_healthlake_FHIRDatastore_PreloadDataConfig as PreloadDataConfig;
    impl crate::value::ToValue for PreloadDataConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PreloadDataType".to_string(),
                crate::value::ToValue::to_value(&self.preload_data_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-healthlake-fhirdatastore-sseconfiguration.html
    pub struct SseConfiguration_ {
        pub kms_encryption_config: Box<KmsEncryptionConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_healthlake_FHIRDatastore_SseConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::HealthLake::FHIRDatastore.SseConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_healthlake_FHIRDatastore_SseConfiguration as SseConfiguration;
    impl crate::value::ToValue for SseConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KmsEncryptionConfig".to_string(),
                crate::value::ToValue::to_value(&self.kms_encryption_config),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-healthlake-fhirdatastore.html
pub struct FHIRDatastore_ {
    pub datastore_name: Option<crate::value::ExpString>,
    pub datastore_type_version: crate::value::ExpString,
    pub identity_provider_configuration:
        Option<super::healthlake::fhirdatastore::IdentityProviderConfiguration_>,
    pub preload_data_config: Option<super::healthlake::fhirdatastore::PreloadDataConfig_>,
    pub sse_configuration: Option<super::healthlake::fhirdatastore::SseConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_healthlake_FHIRDatastore {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::HealthLake::FHIRDatastore"
        $($field $value)*)
    };
}
pub use crate::__aws_healthlake_FHIRDatastore as FHIRDatastore;
impl crate::template::ToResource for FHIRDatastore_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("HealthLake"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FHIRDatastore"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.datastore_name {
            properties.insert(
                "DatastoreName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DatastoreTypeVersion".to_string(),
            crate::value::ToValue::to_value(&self.datastore_type_version),
        );
        if let Some(ref value) = self.identity_provider_configuration {
            properties.insert(
                "IdentityProviderConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.preload_data_config {
            properties.insert(
                "PreloadDataConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sse_configuration {
            properties.insert(
                "SseConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
