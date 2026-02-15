pub mod signingprofile {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-signer-signingprofile-signaturevalidityperiod.html
    pub struct SignatureValidityPeriod_ {
        pub r#type: Option<crate::value::ExpString>,
        pub value: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_signer_SigningProfile_SignatureValidityPeriod {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Signer::SigningProfile.SignatureValidityPeriod"
            $($field $value)*)
        };
    }
    pub use crate::__aws_signer_SigningProfile_SignatureValidityPeriod as SignatureValidityPeriod;
    impl crate::value::ToValue for SignatureValidityPeriod_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-signer-profilepermission.html
pub struct ProfilePermission_ {
    pub action: crate::value::ExpString,
    pub principal: crate::value::ExpString,
    pub profile_name: crate::value::ExpString,
    pub profile_version: Option<crate::value::ExpString>,
    pub statement_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_signer_ProfilePermission {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Signer::ProfilePermission"
        $($field $value)*)
    };
}
pub use crate::__aws_signer_ProfilePermission as ProfilePermission;
impl crate::template::ToResource for ProfilePermission_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Signer"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ProfilePermission"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Action".to_string(),
            crate::value::ToValue::to_value(&self.action),
        );
        properties.insert(
            "Principal".to_string(),
            crate::value::ToValue::to_value(&self.principal),
        );
        properties.insert(
            "ProfileName".to_string(),
            crate::value::ToValue::to_value(&self.profile_name),
        );
        if let Some(ref value) = self.profile_version {
            properties.insert(
                "ProfileVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "StatementId".to_string(),
            crate::value::ToValue::to_value(&self.statement_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-signer-signingprofile.html
pub struct SigningProfile_ {
    pub platform_id: crate::value::ExpString,
    pub signature_validity_period: Option<super::signer::signingprofile::SignatureValidityPeriod_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_signer_SigningProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Signer::SigningProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_signer_SigningProfile as SigningProfile;
impl crate::template::ToResource for SigningProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Signer"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SigningProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PlatformId".to_string(),
            crate::value::ToValue::to_value(&self.platform_id),
        );
        if let Some(ref value) = self.signature_validity_period {
            properties.insert(
                "SignatureValidityPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
