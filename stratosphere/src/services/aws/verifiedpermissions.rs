pub mod identitysource {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-cognitogroupconfiguration.html
    pub struct CognitoGroupConfiguration_ {
        pub group_entity_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_verifiedpermissions_IdentitySource_CognitoGroupConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VerifiedPermissions::IdentitySource.CognitoGroupConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_verifiedpermissions_IdentitySource_CognitoGroupConfiguration as CognitoGroupConfiguration;
    impl crate::value::ToValue for CognitoGroupConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GroupEntityType".to_string(),
                crate::value::ToValue::to_value(&self.group_entity_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-cognitouserpoolconfiguration.html
    pub struct CognitoUserPoolConfiguration_ {
        pub client_ids: Option<Vec<crate::value::ExpString>>,
        pub group_configuration: Option<Box<CognitoGroupConfiguration_>>,
        pub user_pool_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_verifiedpermissions_IdentitySource_CognitoUserPoolConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VerifiedPermissions::IdentitySource.CognitoUserPoolConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_verifiedpermissions_IdentitySource_CognitoUserPoolConfiguration as CognitoUserPoolConfiguration;
    impl crate::value::ToValue for CognitoUserPoolConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.client_ids {
                properties.insert(
                    "ClientIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.group_configuration {
                properties.insert(
                    "GroupConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "UserPoolArn".to_string(),
                crate::value::ToValue::to_value(&self.user_pool_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-identitysourceconfiguration.html
    pub struct IdentitySourceConfiguration_ {
        pub cognito_user_pool_configuration: Option<Box<CognitoUserPoolConfiguration_>>,
        pub open_id_connect_configuration: Option<Box<OpenIdConnectConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_verifiedpermissions_IdentitySource_IdentitySourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VerifiedPermissions::IdentitySource.IdentitySourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_verifiedpermissions_IdentitySource_IdentitySourceConfiguration as IdentitySourceConfiguration;
    impl crate::value::ToValue for IdentitySourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cognito_user_pool_configuration {
                properties.insert(
                    "CognitoUserPoolConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.open_id_connect_configuration {
                properties.insert(
                    "OpenIdConnectConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-openidconnectaccesstokenconfiguration.html
    pub struct OpenIdConnectAccessTokenConfiguration_ {
        pub audiences: Option<Vec<crate::value::ExpString>>,
        pub principal_id_claim: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_verifiedpermissions_IdentitySource_OpenIdConnectAccessTokenConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VerifiedPermissions::IdentitySource.OpenIdConnectAccessTokenConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_verifiedpermissions_IdentitySource_OpenIdConnectAccessTokenConfiguration as OpenIdConnectAccessTokenConfiguration;
    impl crate::value::ToValue for OpenIdConnectAccessTokenConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audiences {
                properties.insert(
                    "Audiences".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.principal_id_claim {
                properties.insert(
                    "PrincipalIdClaim".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-openidconnectconfiguration.html
    pub struct OpenIdConnectConfiguration_ {
        pub entity_id_prefix: Option<crate::value::ExpString>,
        pub group_configuration: Option<Box<OpenIdConnectGroupConfiguration_>>,
        pub issuer: crate::value::ExpString,
        pub token_selection: Box<OpenIdConnectTokenSelection_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_verifiedpermissions_IdentitySource_OpenIdConnectConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VerifiedPermissions::IdentitySource.OpenIdConnectConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_verifiedpermissions_IdentitySource_OpenIdConnectConfiguration as OpenIdConnectConfiguration;
    impl crate::value::ToValue for OpenIdConnectConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.entity_id_prefix {
                properties.insert(
                    "EntityIdPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.group_configuration {
                properties.insert(
                    "GroupConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Issuer".to_string(),
                crate::value::ToValue::to_value(&self.issuer),
            );
            properties.insert(
                "TokenSelection".to_string(),
                crate::value::ToValue::to_value(&self.token_selection),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-openidconnectgroupconfiguration.html
    pub struct OpenIdConnectGroupConfiguration_ {
        pub group_claim: crate::value::ExpString,
        pub group_entity_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_verifiedpermissions_IdentitySource_OpenIdConnectGroupConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VerifiedPermissions::IdentitySource.OpenIdConnectGroupConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_verifiedpermissions_IdentitySource_OpenIdConnectGroupConfiguration as OpenIdConnectGroupConfiguration;
    impl crate::value::ToValue for OpenIdConnectGroupConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GroupClaim".to_string(),
                crate::value::ToValue::to_value(&self.group_claim),
            );
            properties.insert(
                "GroupEntityType".to_string(),
                crate::value::ToValue::to_value(&self.group_entity_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-openidconnectidentitytokenconfiguration.html
    pub struct OpenIdConnectIdentityTokenConfiguration_ {
        pub client_ids: Option<Vec<crate::value::ExpString>>,
        pub principal_id_claim: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_verifiedpermissions_IdentitySource_OpenIdConnectIdentityTokenConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VerifiedPermissions::IdentitySource.OpenIdConnectIdentityTokenConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_verifiedpermissions_IdentitySource_OpenIdConnectIdentityTokenConfiguration as OpenIdConnectIdentityTokenConfiguration;
    impl crate::value::ToValue for OpenIdConnectIdentityTokenConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.client_ids {
                properties.insert(
                    "ClientIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.principal_id_claim {
                properties.insert(
                    "PrincipalIdClaim".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-openidconnecttokenselection.html
    pub struct OpenIdConnectTokenSelection_ {
        pub access_token_only: Option<Box<OpenIdConnectAccessTokenConfiguration_>>,
        pub identity_token_only: Option<Box<OpenIdConnectIdentityTokenConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_verifiedpermissions_IdentitySource_OpenIdConnectTokenSelection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VerifiedPermissions::IdentitySource.OpenIdConnectTokenSelection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_verifiedpermissions_IdentitySource_OpenIdConnectTokenSelection as OpenIdConnectTokenSelection;
    impl crate::value::ToValue for OpenIdConnectTokenSelection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_token_only {
                properties.insert(
                    "AccessTokenOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.identity_token_only {
                properties.insert(
                    "IdentityTokenOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod policy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-entityidentifier.html
    pub struct EntityIdentifier_ {
        pub entity_id: crate::value::ExpString,
        pub entity_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_verifiedpermissions_Policy_EntityIdentifier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VerifiedPermissions::Policy.EntityIdentifier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_verifiedpermissions_Policy_EntityIdentifier as EntityIdentifier;
    impl crate::value::ToValue for EntityIdentifier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EntityId".to_string(),
                crate::value::ToValue::to_value(&self.entity_id),
            );
            properties.insert(
                "EntityType".to_string(),
                crate::value::ToValue::to_value(&self.entity_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-policydefinition.html
    pub struct PolicyDefinition_ {
        pub r#static: Option<Box<StaticPolicyDefinition_>>,
        pub template_linked: Option<Box<TemplateLinkedPolicyDefinition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_verifiedpermissions_Policy_PolicyDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VerifiedPermissions::Policy.PolicyDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_verifiedpermissions_Policy_PolicyDefinition as PolicyDefinition;
    impl crate::value::ToValue for PolicyDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#static {
                properties.insert("Static".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.template_linked {
                properties.insert(
                    "TemplateLinked".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-staticpolicydefinition.html
    pub struct StaticPolicyDefinition_ {
        pub description: Option<crate::value::ExpString>,
        pub statement: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_verifiedpermissions_Policy_StaticPolicyDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VerifiedPermissions::Policy.StaticPolicyDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_verifiedpermissions_Policy_StaticPolicyDefinition as StaticPolicyDefinition;
    impl crate::value::ToValue for StaticPolicyDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Statement".to_string(),
                crate::value::ToValue::to_value(&self.statement),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-templatelinkedpolicydefinition.html
    pub struct TemplateLinkedPolicyDefinition_ {
        pub policy_template_id: crate::value::ExpString,
        pub principal: Option<Box<EntityIdentifier_>>,
        pub resource: Option<Box<EntityIdentifier_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_verifiedpermissions_Policy_TemplateLinkedPolicyDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VerifiedPermissions::Policy.TemplateLinkedPolicyDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_verifiedpermissions_Policy_TemplateLinkedPolicyDefinition as TemplateLinkedPolicyDefinition;
    impl crate::value::ToValue for TemplateLinkedPolicyDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PolicyTemplateId".to_string(),
                crate::value::ToValue::to_value(&self.policy_template_id),
            );
            if let Some(ref value) = self.principal {
                properties.insert(
                    "Principal".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource {
                properties.insert(
                    "Resource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod policystore {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policystore-deletionprotection.html
    pub struct DeletionProtection_ {
        pub mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_verifiedpermissions_PolicyStore_DeletionProtection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VerifiedPermissions::PolicyStore.DeletionProtection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_verifiedpermissions_PolicyStore_DeletionProtection as DeletionProtection;
    impl crate::value::ToValue for DeletionProtection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Mode".to_string(),
                crate::value::ToValue::to_value(&self.mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policystore-schemadefinition.html
    pub struct SchemaDefinition_ {
        pub cedar_format: Option<crate::value::ExpString>,
        pub cedar_json: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_verifiedpermissions_PolicyStore_SchemaDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VerifiedPermissions::PolicyStore.SchemaDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_verifiedpermissions_PolicyStore_SchemaDefinition as SchemaDefinition;
    impl crate::value::ToValue for SchemaDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cedar_format {
                properties.insert(
                    "CedarFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cedar_json {
                properties.insert(
                    "CedarJson".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policystore-validationsettings.html
    pub struct ValidationSettings_ {
        pub mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_verifiedpermissions_PolicyStore_ValidationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VerifiedPermissions::PolicyStore.ValidationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_verifiedpermissions_PolicyStore_ValidationSettings as ValidationSettings;
    impl crate::value::ToValue for ValidationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Mode".to_string(),
                crate::value::ToValue::to_value(&self.mode),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-identitysource.html
pub struct IdentitySource_ {
    pub configuration: super::verifiedpermissions::identitysource::IdentitySourceConfiguration_,
    pub policy_store_id: crate::value::ExpString,
    pub principal_entity_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_verifiedpermissions_IdentitySource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VerifiedPermissions::IdentitySource"
        $($field $value)*)
    };
}
pub use crate::__aws_verifiedpermissions_IdentitySource as IdentitySource;
impl crate::template::ToResource for IdentitySource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VerifiedPermissions"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IdentitySource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Configuration".to_string(),
            crate::value::ToValue::to_value(&self.configuration),
        );
        properties.insert(
            "PolicyStoreId".to_string(),
            crate::value::ToValue::to_value(&self.policy_store_id),
        );
        if let Some(ref value) = self.principal_entity_type {
            properties.insert(
                "PrincipalEntityType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-policy.html
pub struct Policy_ {
    pub definition: super::verifiedpermissions::policy::PolicyDefinition_,
    pub policy_store_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_verifiedpermissions_Policy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VerifiedPermissions::Policy"
        $($field $value)*)
    };
}
pub use crate::__aws_verifiedpermissions_Policy as Policy;
impl crate::template::ToResource for Policy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VerifiedPermissions"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Policy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Definition".to_string(),
            crate::value::ToValue::to_value(&self.definition),
        );
        properties.insert(
            "PolicyStoreId".to_string(),
            crate::value::ToValue::to_value(&self.policy_store_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-policystore.html
pub struct PolicyStore_ {
    pub deletion_protection: Option<super::verifiedpermissions::policystore::DeletionProtection_>,
    pub description: Option<crate::value::ExpString>,
    pub schema: Option<super::verifiedpermissions::policystore::SchemaDefinition_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub validation_settings: super::verifiedpermissions::policystore::ValidationSettings_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_verifiedpermissions_PolicyStore {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VerifiedPermissions::PolicyStore"
        $($field $value)*)
    };
}
pub use crate::__aws_verifiedpermissions_PolicyStore as PolicyStore;
impl crate::template::ToResource for PolicyStore_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VerifiedPermissions"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PolicyStore"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.deletion_protection {
            properties.insert(
                "DeletionProtection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schema {
            properties.insert("Schema".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "ValidationSettings".to_string(),
            crate::value::ToValue::to_value(&self.validation_settings),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-policytemplate.html
pub struct PolicyTemplate_ {
    pub description: Option<crate::value::ExpString>,
    pub policy_store_id: crate::value::ExpString,
    pub statement: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_verifiedpermissions_PolicyTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VerifiedPermissions::PolicyTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_verifiedpermissions_PolicyTemplate as PolicyTemplate;
impl crate::template::ToResource for PolicyTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VerifiedPermissions"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PolicyTemplate"),
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
            "PolicyStoreId".to_string(),
            crate::value::ToValue::to_value(&self.policy_store_id),
        );
        properties.insert(
            "Statement".to_string(),
            crate::value::ToValue::to_value(&self.statement),
        );
        properties
    }
}
