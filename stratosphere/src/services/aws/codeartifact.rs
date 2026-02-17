pub mod packagegroup {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codeartifact-packagegroup-originconfiguration.html>
    pub struct OriginConfiguration_ {
        pub restrictions: Box<Restrictions_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codeartifact_PackageGroup_OriginConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeArtifact::PackageGroup.OriginConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codeartifact_PackageGroup_OriginConfiguration as OriginConfiguration;
    impl crate::value::ToValue for OriginConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Restrictions".to_string(),
                crate::value::ToValue::to_value(&self.restrictions),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codeartifact-packagegroup-restrictiontype.html>
    pub struct RestrictionType_ {
        pub repositories: Option<Vec<crate::value::ExpString>>,
        pub restriction_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codeartifact_PackageGroup_RestrictionType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeArtifact::PackageGroup.RestrictionType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codeartifact_PackageGroup_RestrictionType as RestrictionType;
    impl crate::value::ToValue for RestrictionType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.repositories {
                properties.insert(
                    "Repositories".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RestrictionMode".to_string(),
                crate::value::ToValue::to_value(&self.restriction_mode),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codeartifact-packagegroup-restrictions.html>
    pub struct Restrictions_ {
        pub external_upstream: Option<Box<RestrictionType_>>,
        pub internal_upstream: Option<Box<RestrictionType_>>,
        pub publish: Option<Box<RestrictionType_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codeartifact_PackageGroup_Restrictions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeArtifact::PackageGroup.Restrictions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codeartifact_PackageGroup_Restrictions as Restrictions;
    impl crate::value::ToValue for Restrictions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.external_upstream {
                properties.insert(
                    "ExternalUpstream".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.internal_upstream {
                properties.insert(
                    "InternalUpstream".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.publish {
                properties.insert(
                    "Publish".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codeartifact-domain.html>
pub struct Domain_ {
    pub domain_name: crate::value::ExpString,
    pub encryption_key: Option<crate::value::ExpString>,
    pub permissions_policy_document: Option<serde_json::Value>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codeartifact_Domain {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeArtifact::Domain"
        $($field $value)*)
    };
}
pub use crate::__aws_codeartifact_Domain as Domain;
impl crate::template::ToResource for Domain_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeArtifact"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Domain"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        if let Some(ref value) = self.encryption_key {
            properties.insert(
                "EncryptionKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.permissions_policy_document {
            properties.insert(
                "PermissionsPolicyDocument".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codeartifact-packagegroup.html>
pub struct PackageGroup_ {
    pub contact_info: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub domain_name: crate::value::ExpString,
    pub domain_owner: Option<crate::value::ExpString>,
    pub origin_configuration: Option<super::codeartifact::packagegroup::OriginConfiguration_>,
    pub pattern: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codeartifact_PackageGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeArtifact::PackageGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_codeartifact_PackageGroup as PackageGroup;
impl crate::template::ToResource for PackageGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeArtifact"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PackageGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.contact_info {
            properties.insert(
                "ContactInfo".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        if let Some(ref value) = self.domain_owner {
            properties.insert(
                "DomainOwner".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.origin_configuration {
            properties.insert(
                "OriginConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Pattern".to_string(),
            crate::value::ToValue::to_value(&self.pattern),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codeartifact-repository.html>
pub struct Repository_ {
    pub description: Option<crate::value::ExpString>,
    pub domain_name: crate::value::ExpString,
    pub domain_owner: Option<crate::value::ExpString>,
    pub external_connections: Option<Vec<crate::value::ExpString>>,
    pub permissions_policy_document: Option<serde_json::Value>,
    pub repository_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub upstreams: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codeartifact_Repository {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeArtifact::Repository"
        $($field $value)*)
    };
}
pub use crate::__aws_codeartifact_Repository as Repository;
impl crate::template::ToResource for Repository_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeArtifact"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Repository"),
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
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        if let Some(ref value) = self.domain_owner {
            properties.insert(
                "DomainOwner".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.external_connections {
            properties.insert(
                "ExternalConnections".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.permissions_policy_document {
            properties.insert(
                "PermissionsPolicyDocument".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RepositoryName".to_string(),
            crate::value::ToValue::to_value(&self.repository_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.upstreams {
            properties.insert(
                "Upstreams".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
