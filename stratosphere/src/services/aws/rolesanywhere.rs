pub mod profile {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rolesanywhere-profile-attributemapping.html
    pub struct AttributeMapping_ {
        pub certificate_field: crate::value::ExpString,
        pub mapping_rules: Vec<MappingRule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rolesanywhere_Profile_AttributeMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RolesAnywhere::Profile.AttributeMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rolesanywhere_Profile_AttributeMapping as AttributeMapping;
    impl crate::value::ToValue for AttributeMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateField".to_string(),
                crate::value::ToValue::to_value(&self.certificate_field),
            );
            properties.insert(
                "MappingRules".to_string(),
                crate::value::ToValue::to_value(&self.mapping_rules),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rolesanywhere-profile-mappingrule.html
    pub struct MappingRule_ {
        pub specifier: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rolesanywhere_Profile_MappingRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RolesAnywhere::Profile.MappingRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rolesanywhere_Profile_MappingRule as MappingRule;
    impl crate::value::ToValue for MappingRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Specifier".to_string(),
                crate::value::ToValue::to_value(&self.specifier),
            );
            properties.into()
        }
    }
}
pub mod trustanchor {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rolesanywhere-trustanchor-notificationsetting.html
    pub struct NotificationSetting_ {
        pub channel: Option<crate::value::ExpString>,
        pub enabled: crate::value::ExpBool,
        pub event: crate::value::ExpString,
        pub threshold: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rolesanywhere_TrustAnchor_NotificationSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RolesAnywhere::TrustAnchor.NotificationSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rolesanywhere_TrustAnchor_NotificationSetting as NotificationSetting;
    impl crate::value::ToValue for NotificationSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.channel {
                properties.insert(
                    "Channel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.insert(
                "Event".to_string(),
                crate::value::ToValue::to_value(&self.event),
            );
            if let Some(ref value) = self.threshold {
                properties.insert(
                    "Threshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rolesanywhere-trustanchor-source.html
    pub struct Source_ {
        pub source_data: Box<SourceData_>,
        pub source_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rolesanywhere_TrustAnchor_Source {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RolesAnywhere::TrustAnchor.Source"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rolesanywhere_TrustAnchor_Source as Source;
    impl crate::value::ToValue for Source_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SourceData".to_string(),
                crate::value::ToValue::to_value(&self.source_data),
            );
            properties.insert(
                "SourceType".to_string(),
                crate::value::ToValue::to_value(&self.source_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rolesanywhere-trustanchor-sourcedata.html
    pub struct SourceData_ {
        pub acm_pca_arn: Option<crate::value::ExpString>,
        pub x509_certificate_data: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rolesanywhere_TrustAnchor_SourceData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RolesAnywhere::TrustAnchor.SourceData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rolesanywhere_TrustAnchor_SourceData as SourceData;
    impl crate::value::ToValue for SourceData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.acm_pca_arn {
                properties.insert(
                    "AcmPcaArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.x509_certificate_data {
                properties.insert(
                    "X509CertificateData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-crl.html
pub struct CRL_ {
    pub crl_data: crate::value::ExpString,
    pub enabled: Option<crate::value::ExpBool>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub trust_anchor_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rolesanywhere_CRL {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RolesAnywhere::CRL"
        $($field $value)*)
    };
}
pub use crate::__aws_rolesanywhere_CRL as CRL;
impl crate::template::ToResource for CRL_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RolesAnywhere"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CRL"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CrlData".to_string(),
            crate::value::ToValue::to_value(&self.crl_data),
        );
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.trust_anchor_arn {
            properties.insert(
                "TrustAnchorArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-profile.html
pub struct Profile_ {
    pub accept_role_session_name: Option<crate::value::ExpBool>,
    pub attribute_mappings: Option<Vec<super::rolesanywhere::profile::AttributeMapping_>>,
    pub duration_seconds: Option<f64>,
    pub enabled: Option<crate::value::ExpBool>,
    pub managed_policy_arns: Option<Vec<crate::value::ExpString>>,
    pub name: crate::value::ExpString,
    pub require_instance_properties: Option<crate::value::ExpBool>,
    pub role_arns: Vec<crate::value::ExpString>,
    pub session_policy: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rolesanywhere_Profile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RolesAnywhere::Profile"
        $($field $value)*)
    };
}
pub use crate::__aws_rolesanywhere_Profile as Profile;
impl crate::template::ToResource for Profile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RolesAnywhere"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Profile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accept_role_session_name {
            properties.insert(
                "AcceptRoleSessionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.attribute_mappings {
            properties.insert(
                "AttributeMappings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.duration_seconds {
            properties.insert(
                "DurationSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.managed_policy_arns {
            properties.insert(
                "ManagedPolicyArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.require_instance_properties {
            properties.insert(
                "RequireInstanceProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArns".to_string(),
            crate::value::ToValue::to_value(&self.role_arns),
        );
        if let Some(ref value) = self.session_policy {
            properties.insert(
                "SessionPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rolesanywhere-trustanchor.html
pub struct TrustAnchor_ {
    pub enabled: Option<crate::value::ExpBool>,
    pub name: crate::value::ExpString,
    pub notification_settings: Option<Vec<super::rolesanywhere::trustanchor::NotificationSetting_>>,
    pub source: super::rolesanywhere::trustanchor::Source_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rolesanywhere_TrustAnchor {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RolesAnywhere::TrustAnchor"
        $($field $value)*)
    };
}
pub use crate::__aws_rolesanywhere_TrustAnchor as TrustAnchor;
impl crate::template::ToResource for TrustAnchor_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RolesAnywhere"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TrustAnchor"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.notification_settings {
            properties.insert(
                "NotificationSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Source".to_string(),
            crate::value::ToValue::to_value(&self.source),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
