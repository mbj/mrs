pub mod skill {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ask-skill-authenticationconfiguration.html>
    pub struct AuthenticationConfiguration_ {
        pub client_id: crate::value::ExpString,
        pub client_secret: crate::value::ExpString,
        pub refresh_token: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __alexa_ask_Skill_AuthenticationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("Alexa::ASK::Skill.AuthenticationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__alexa_ask_Skill_AuthenticationConfiguration as AuthenticationConfiguration;
    impl crate::value::ToValue for AuthenticationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClientId".to_string(),
                crate::value::ToValue::to_value(&self.client_id),
            );
            properties.insert(
                "ClientSecret".to_string(),
                crate::value::ToValue::to_value(&self.client_secret),
            );
            properties.insert(
                "RefreshToken".to_string(),
                crate::value::ToValue::to_value(&self.refresh_token),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ask-skill-overrides.html>
    pub struct Overrides_ {
        pub manifest: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __alexa_ask_Skill_Overrides {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("Alexa::ASK::Skill.Overrides"
            $($field $value)*)
        };
    }
    pub use crate::__alexa_ask_Skill_Overrides as Overrides;
    impl crate::value::ToValue for Overrides_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.manifest {
                properties.insert(
                    "Manifest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ask-skill-skillpackage.html>
    pub struct SkillPackage_ {
        pub overrides: Option<Box<Overrides_>>,
        pub s3_bucket: crate::value::ExpString,
        pub s3_bucket_role: Option<crate::value::ExpString>,
        pub s3_key: crate::value::ExpString,
        pub s3_object_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __alexa_ask_Skill_SkillPackage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("Alexa::ASK::Skill.SkillPackage"
            $($field $value)*)
        };
    }
    pub use crate::__alexa_ask_Skill_SkillPackage as SkillPackage;
    impl crate::value::ToValue for SkillPackage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.overrides {
                properties.insert(
                    "Overrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Bucket".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket),
            );
            if let Some(ref value) = self.s3_bucket_role {
                properties.insert(
                    "S3BucketRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Key".to_string(),
                crate::value::ToValue::to_value(&self.s3_key),
            );
            if let Some(ref value) = self.s3_object_version {
                properties.insert(
                    "S3ObjectVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ask-skill.html>
pub struct Skill_ {
    pub authentication_configuration: super::ask::skill::AuthenticationConfiguration_,
    pub skill_package: super::ask::skill::SkillPackage_,
    pub vendor_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __alexa_ask_Skill {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("Alexa::ASK::Skill" $($field
        $value)*)
    };
}
pub use crate::__alexa_ask_Skill as Skill;
impl crate::template::ToResource for Skill_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ASK"),
                vendor_name: crate::resource_specification::VendorName("Alexa"),
            },
            resource_name: crate::resource_specification::ResourceName("Skill"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AuthenticationConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.authentication_configuration),
        );
        properties.insert(
            "SkillPackage".to_string(),
            crate::value::ToValue::to_value(&self.skill_package),
        );
        properties.insert(
            "VendorId".to_string(),
            crate::value::ToValue::to_value(&self.vendor_id),
        );
        properties
    }
}
