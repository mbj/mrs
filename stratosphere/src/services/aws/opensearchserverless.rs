pub mod collection {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchserverless-collection-encryptionconfig.html
    pub struct EncryptionConfig_ {
        pub aws_owned_key: Option<crate::value::ExpBool>,
        pub kms_key_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchserverless_Collection_EncryptionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpenSearchServerless::Collection.EncryptionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchserverless_Collection_EncryptionConfig as EncryptionConfig;
    impl crate::value::ToValue for EncryptionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_owned_key {
                properties.insert(
                    "AWSOwnedKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod index {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchserverless-index-index.html
    pub struct Index_ {
        pub knn: Option<crate::value::ExpBool>,
        pub knn_algo_param_ef_search: Option<i32>,
        pub refresh_interval: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchserverless_Index_Index {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpenSearchServerless::Index.Index"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchserverless_Index_Index as Index;
    impl crate::value::ToValue for Index_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.knn {
                properties.insert("Knn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.knn_algo_param_ef_search {
                properties.insert(
                    "KnnAlgoParamEfSearch".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.refresh_interval {
                properties.insert(
                    "RefreshInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchserverless-index-indexsettings.html
    pub struct IndexSettings_ {
        pub index: Option<Box<Index_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchserverless_Index_IndexSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpenSearchServerless::Index.IndexSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchserverless_Index_IndexSettings as IndexSettings;
    impl crate::value::ToValue for IndexSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.index {
                properties.insert("Index".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchserverless-index-mappings.html
    pub struct Mappings_ {
        pub properties: Option<std::collections::BTreeMap<String, PropertyMapping_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchserverless_Index_Mappings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpenSearchServerless::Index.Mappings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchserverless_Index_Mappings as Mappings;
    impl crate::value::ToValue for Mappings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.properties {
                properties.insert(
                    "Properties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchserverless-index-method.html
    pub struct Method_ {
        pub engine: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub parameters: Option<Box<Parameters_>>,
        pub space_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchserverless_Index_Method {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpenSearchServerless::Index.Method"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchserverless_Index_Method as Method;
    impl crate::value::ToValue for Method_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.engine {
                properties.insert("Engine".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.space_type {
                properties.insert(
                    "SpaceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchserverless-index-parameters.html
    pub struct Parameters_ {
        pub ef_construction: Option<i32>,
        pub m: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchserverless_Index_Parameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpenSearchServerless::Index.Parameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchserverless_Index_Parameters as Parameters;
    impl crate::value::ToValue for Parameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ef_construction {
                properties.insert(
                    "EfConstruction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.m {
                properties.insert("M".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchserverless-index-propertymapping.html
    pub struct PropertyMapping_ {
        pub dimension: Option<i32>,
        pub index: Option<crate::value::ExpBool>,
        pub method: Option<Box<Method_>>,
        pub properties: Option<std::collections::BTreeMap<String, PropertyMapping_>>,
        pub r#type: crate::value::ExpString,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchserverless_Index_PropertyMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpenSearchServerless::Index.PropertyMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchserverless_Index_PropertyMapping as PropertyMapping;
    impl crate::value::ToValue for PropertyMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimension {
                properties.insert(
                    "Dimension".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.index {
                properties.insert("Index".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.method {
                properties.insert("Method".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.properties {
                properties.insert(
                    "Properties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod securityconfig {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchserverless-securityconfig-iamfederationconfigoptions.html
    pub struct IamFederationConfigOptions_ {
        pub group_attribute: Option<crate::value::ExpString>,
        pub user_attribute: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchserverless_SecurityConfig_IamFederationConfigOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpenSearchServerless::SecurityConfig.IamFederationConfigOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchserverless_SecurityConfig_IamFederationConfigOptions as IamFederationConfigOptions;
    impl crate::value::ToValue for IamFederationConfigOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.group_attribute {
                properties.insert(
                    "GroupAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_attribute {
                properties.insert(
                    "UserAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchserverless-securityconfig-iamidentitycenterconfigoptions.html
    pub struct IamIdentityCenterConfigOptions_ {
        pub application_arn: Option<crate::value::ExpString>,
        pub application_description: Option<crate::value::ExpString>,
        pub application_name: Option<crate::value::ExpString>,
        pub group_attribute: Option<crate::value::ExpString>,
        pub instance_arn: crate::value::ExpString,
        pub user_attribute: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchserverless_SecurityConfig_IamIdentityCenterConfigOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpenSearchServerless::SecurityConfig.IamIdentityCenterConfigOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchserverless_SecurityConfig_IamIdentityCenterConfigOptions as IamIdentityCenterConfigOptions;
    impl crate::value::ToValue for IamIdentityCenterConfigOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_arn {
                properties.insert(
                    "ApplicationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.application_description {
                properties.insert(
                    "ApplicationDescription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.application_name {
                properties.insert(
                    "ApplicationName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.group_attribute {
                properties.insert(
                    "GroupAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InstanceArn".to_string(),
                crate::value::ToValue::to_value(&self.instance_arn),
            );
            if let Some(ref value) = self.user_attribute {
                properties.insert(
                    "UserAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opensearchserverless-securityconfig-samlconfigoptions.html
    pub struct SamlConfigOptions_ {
        pub group_attribute: Option<crate::value::ExpString>,
        pub metadata: crate::value::ExpString,
        pub open_search_serverless_entity_id: Option<crate::value::ExpString>,
        pub session_timeout: Option<i32>,
        pub user_attribute: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opensearchserverless_SecurityConfig_SamlConfigOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpenSearchServerless::SecurityConfig.SamlConfigOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opensearchserverless_SecurityConfig_SamlConfigOptions as SamlConfigOptions;
    impl crate::value::ToValue for SamlConfigOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.group_attribute {
                properties.insert(
                    "GroupAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Metadata".to_string(),
                crate::value::ToValue::to_value(&self.metadata),
            );
            if let Some(ref value) = self.open_search_serverless_entity_id {
                properties.insert(
                    "OpenSearchServerlessEntityId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.session_timeout {
                properties.insert(
                    "SessionTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_attribute {
                properties.insert(
                    "UserAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchserverless-accesspolicy.html
pub struct AccessPolicy_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub policy: crate::value::ExpString,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opensearchserverless_AccessPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::OpenSearchServerless::AccessPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_opensearchserverless_AccessPolicy as AccessPolicy;
impl crate::template::ToResource for AccessPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpenSearchServerless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AccessPolicy"),
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
            "Policy".to_string(),
            crate::value::ToValue::to_value(&self.policy),
        );
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchserverless-collection.html
pub struct Collection_ {
    pub collection_group_name: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub encryption_config: Option<super::opensearchserverless::collection::EncryptionConfig_>,
    pub name: crate::value::ExpString,
    pub standby_replicas: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opensearchserverless_Collection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::OpenSearchServerless::Collection"
        $($field $value)*)
    };
}
pub use crate::__aws_opensearchserverless_Collection as Collection;
impl crate::template::ToResource for Collection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpenSearchServerless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Collection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.collection_group_name {
            properties.insert(
                "CollectionGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_config {
            properties.insert(
                "EncryptionConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.standby_replicas {
            properties.insert(
                "StandbyReplicas".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchserverless-index.html
pub struct Index_ {
    pub collection_endpoint: crate::value::ExpString,
    pub index_name: crate::value::ExpString,
    pub mappings: Option<super::opensearchserverless::index::Mappings_>,
    pub settings: Option<super::opensearchserverless::index::IndexSettings_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opensearchserverless_Index {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::OpenSearchServerless::Index"
        $($field $value)*)
    };
}
pub use crate::__aws_opensearchserverless_Index as Index;
impl crate::template::ToResource for Index_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpenSearchServerless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Index"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CollectionEndpoint".to_string(),
            crate::value::ToValue::to_value(&self.collection_endpoint),
        );
        properties.insert(
            "IndexName".to_string(),
            crate::value::ToValue::to_value(&self.index_name),
        );
        if let Some(ref value) = self.mappings {
            properties.insert(
                "Mappings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.settings {
            properties.insert(
                "Settings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchserverless-lifecyclepolicy.html
pub struct LifecyclePolicy_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub policy: crate::value::ExpString,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opensearchserverless_LifecyclePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::OpenSearchServerless::LifecyclePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_opensearchserverless_LifecyclePolicy as LifecyclePolicy;
impl crate::template::ToResource for LifecyclePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpenSearchServerless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LifecyclePolicy"),
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
            "Policy".to_string(),
            crate::value::ToValue::to_value(&self.policy),
        );
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchserverless-securityconfig.html
pub struct SecurityConfig_ {
    pub description: Option<crate::value::ExpString>,
    pub iam_federation_options:
        Option<super::opensearchserverless::securityconfig::IamFederationConfigOptions_>,
    pub iam_identity_center_options:
        Option<super::opensearchserverless::securityconfig::IamIdentityCenterConfigOptions_>,
    pub name: Option<crate::value::ExpString>,
    pub saml_options: Option<super::opensearchserverless::securityconfig::SamlConfigOptions_>,
    pub r#type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opensearchserverless_SecurityConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::OpenSearchServerless::SecurityConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_opensearchserverless_SecurityConfig as SecurityConfig;
impl crate::template::ToResource for SecurityConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpenSearchServerless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SecurityConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iam_federation_options {
            properties.insert(
                "IamFederationOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iam_identity_center_options {
            properties.insert(
                "IamIdentityCenterOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.saml_options {
            properties.insert(
                "SamlOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchserverless-securitypolicy.html
pub struct SecurityPolicy_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub policy: crate::value::ExpString,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opensearchserverless_SecurityPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::OpenSearchServerless::SecurityPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_opensearchserverless_SecurityPolicy as SecurityPolicy;
impl crate::template::ToResource for SecurityPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpenSearchServerless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SecurityPolicy"),
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
            "Policy".to_string(),
            crate::value::ToValue::to_value(&self.policy),
        );
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opensearchserverless-vpcendpoint.html
pub struct VpcEndpoint_ {
    pub name: crate::value::ExpString,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opensearchserverless_VpcEndpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::OpenSearchServerless::VpcEndpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_opensearchserverless_VpcEndpoint as VpcEndpoint;
impl crate::template::ToResource for VpcEndpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpenSearchServerless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VpcEndpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SubnetIds".to_string(),
            crate::value::ToValue::to_value(&self.subnet_ids),
        );
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
