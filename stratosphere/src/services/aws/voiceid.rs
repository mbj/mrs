pub mod domain {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-voiceid-domain-serversideencryptionconfiguration.html
    pub struct ServerSideEncryptionConfiguration_ {
        pub kms_key_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_voiceid_Domain_ServerSideEncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VoiceID::Domain.ServerSideEncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_voiceid_Domain_ServerSideEncryptionConfiguration as ServerSideEncryptionConfiguration;
    impl crate::value::ToValue for ServerSideEncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(&self.kms_key_id),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-voiceid-domain.html
pub struct Domain_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub server_side_encryption_configuration:
        super::voiceid::domain::ServerSideEncryptionConfiguration_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_voiceid_Domain {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VoiceID::Domain" $($field
        $value)*)
    };
}
pub use crate::__aws_voiceid_Domain as Domain;
impl crate::template::ToResource for Domain_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VoiceID"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Domain"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
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
        properties.insert(
            "ServerSideEncryptionConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.server_side_encryption_configuration),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
