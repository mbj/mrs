pub mod workspace {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-assertionattributes.html
    pub struct AssertionAttributes_ {
        pub email: Option<crate::value::ExpString>,
        pub groups: Option<crate::value::ExpString>,
        pub login: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub org: Option<crate::value::ExpString>,
        pub role: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_grafana_Workspace_AssertionAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Grafana::Workspace.AssertionAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_grafana_Workspace_AssertionAttributes as AssertionAttributes;
    impl crate::value::ToValue for AssertionAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.email {
                properties.insert("Email".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.groups {
                properties.insert("Groups".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.login {
                properties.insert("Login".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.org {
                properties.insert("Org".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.role {
                properties.insert("Role".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-idpmetadata.html
    pub struct IdpMetadata_ {
        pub url: Option<crate::value::ExpString>,
        pub xml: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_grafana_Workspace_IdpMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Grafana::Workspace.IdpMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_grafana_Workspace_IdpMetadata as IdpMetadata;
    impl crate::value::ToValue for IdpMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.xml {
                properties.insert("Xml".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-networkaccesscontrol.html
    pub struct NetworkAccessControl_ {
        pub prefix_list_ids: Option<Vec<crate::value::ExpString>>,
        pub vpce_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_grafana_Workspace_NetworkAccessControl {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Grafana::Workspace.NetworkAccessControl"
            $($field $value)*)
        };
    }
    pub use crate::__aws_grafana_Workspace_NetworkAccessControl as NetworkAccessControl;
    impl crate::value::ToValue for NetworkAccessControl_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.prefix_list_ids {
                properties.insert(
                    "PrefixListIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpce_ids {
                properties.insert(
                    "VpceIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-rolevalues.html
    pub struct RoleValues_ {
        pub admin: Option<Vec<crate::value::ExpString>>,
        pub editor: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_grafana_Workspace_RoleValues {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Grafana::Workspace.RoleValues"
            $($field $value)*)
        };
    }
    pub use crate::__aws_grafana_Workspace_RoleValues as RoleValues;
    impl crate::value::ToValue for RoleValues_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.admin {
                properties.insert("Admin".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.editor {
                properties.insert("Editor".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-samlconfiguration.html
    pub struct SamlConfiguration_ {
        pub allowed_organizations: Option<Vec<crate::value::ExpString>>,
        pub assertion_attributes: Option<Box<AssertionAttributes_>>,
        pub idp_metadata: Box<IdpMetadata_>,
        pub login_validity_duration: Option<f64>,
        pub role_values: Option<Box<RoleValues_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_grafana_Workspace_SamlConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Grafana::Workspace.SamlConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_grafana_Workspace_SamlConfiguration as SamlConfiguration;
    impl crate::value::ToValue for SamlConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_organizations {
                properties.insert(
                    "AllowedOrganizations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.assertion_attributes {
                properties.insert(
                    "AssertionAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IdpMetadata".to_string(),
                crate::value::ToValue::to_value(&self.idp_metadata),
            );
            if let Some(ref value) = self.login_validity_duration {
                properties.insert(
                    "LoginValidityDuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_values {
                properties.insert(
                    "RoleValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-grafana-workspace-vpcconfiguration.html
    pub struct VpcConfiguration_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnet_ids: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_grafana_Workspace_VpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Grafana::Workspace.VpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_grafana_Workspace_VpcConfiguration as VpcConfiguration;
    impl crate::value::ToValue for VpcConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(&self.subnet_ids),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-grafana-workspace.html
pub struct Workspace_ {
    pub account_access_type: crate::value::ExpString,
    pub authentication_providers: Vec<crate::value::ExpString>,
    pub client_token: Option<crate::value::ExpString>,
    pub data_sources: Option<Vec<crate::value::ExpString>>,
    pub description: Option<crate::value::ExpString>,
    pub grafana_version: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub network_access_control: Option<super::grafana::workspace::NetworkAccessControl_>,
    pub notification_destinations: Option<Vec<crate::value::ExpString>>,
    pub organization_role_name: Option<crate::value::ExpString>,
    pub organizational_units: Option<Vec<crate::value::ExpString>>,
    pub permission_type: crate::value::ExpString,
    pub plugin_admin_enabled: Option<crate::value::ExpBool>,
    pub role_arn: Option<crate::value::ExpString>,
    pub saml_configuration: Option<super::grafana::workspace::SamlConfiguration_>,
    pub stack_set_name: Option<crate::value::ExpString>,
    pub vpc_configuration: Option<super::grafana::workspace::VpcConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_grafana_Workspace {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Grafana::Workspace"
        $($field $value)*)
    };
}
pub use crate::__aws_grafana_Workspace as Workspace;
impl crate::template::ToResource for Workspace_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Grafana"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Workspace"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AccountAccessType".to_string(),
            crate::value::ToValue::to_value(&self.account_access_type),
        );
        properties.insert(
            "AuthenticationProviders".to_string(),
            crate::value::ToValue::to_value(&self.authentication_providers),
        );
        if let Some(ref value) = self.client_token {
            properties.insert(
                "ClientToken".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_sources {
            properties.insert(
                "DataSources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.grafana_version {
            properties.insert(
                "GrafanaVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.network_access_control {
            properties.insert(
                "NetworkAccessControl".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.notification_destinations {
            properties.insert(
                "NotificationDestinations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.organization_role_name {
            properties.insert(
                "OrganizationRoleName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.organizational_units {
            properties.insert(
                "OrganizationalUnits".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PermissionType".to_string(),
            crate::value::ToValue::to_value(&self.permission_type),
        );
        if let Some(ref value) = self.plugin_admin_enabled {
            properties.insert(
                "PluginAdminEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.saml_configuration {
            properties.insert(
                "SamlConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.stack_set_name {
            properties.insert(
                "StackSetName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_configuration {
            properties.insert(
                "VpcConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
