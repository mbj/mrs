pub mod application {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sso-application-portaloptionsconfiguration.html>
    pub struct PortalOptionsConfiguration_ {
        pub sign_in_options: Option<Box<SignInOptions_>>,
        pub visibility: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sso_Application_PortalOptionsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSO::Application.PortalOptionsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sso_Application_PortalOptionsConfiguration as PortalOptionsConfiguration;
    impl crate::value::ToValue for PortalOptionsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sign_in_options {
                properties.insert(
                    "SignInOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.visibility {
                properties.insert(
                    "Visibility".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sso-application-signinoptions.html>
    pub struct SignInOptions_ {
        pub application_url: Option<crate::value::ExpString>,
        pub origin: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sso_Application_SignInOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSO::Application.SignInOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sso_Application_SignInOptions as SignInOptions;
    impl crate::value::ToValue for SignInOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_url {
                properties.insert(
                    "ApplicationUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Origin".to_string(),
                crate::value::ToValue::to_value(&self.origin),
            );
            properties.into()
        }
    }
}
pub mod instanceaccesscontrolattributeconfiguration {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sso-instanceaccesscontrolattributeconfiguration-accesscontrolattribute.html>
    pub struct AccessControlAttribute_ {
        pub key: crate::value::ExpString,
        pub value: Box<AccessControlAttributeValue_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sso_InstanceAccessControlAttributeConfiguration_AccessControlAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSO::InstanceAccessControlAttributeConfiguration.AccessControlAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sso_InstanceAccessControlAttributeConfiguration_AccessControlAttribute as AccessControlAttribute;
    impl crate::value::ToValue for AccessControlAttribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sso-instanceaccesscontrolattributeconfiguration-accesscontrolattributevalue.html>
    pub struct AccessControlAttributeValue_ {
        pub source: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sso_InstanceAccessControlAttributeConfiguration_AccessControlAttributeValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSO::InstanceAccessControlAttributeConfiguration.AccessControlAttributeValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sso_InstanceAccessControlAttributeConfiguration_AccessControlAttributeValue as AccessControlAttributeValue;
    impl crate::value::ToValue for AccessControlAttributeValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            properties.into()
        }
    }
}
pub mod permissionset {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sso-permissionset-customermanagedpolicyreference.html>
    pub struct CustomerManagedPolicyReference_ {
        pub name: crate::value::ExpString,
        pub path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sso_PermissionSet_CustomerManagedPolicyReference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSO::PermissionSet.CustomerManagedPolicyReference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sso_PermissionSet_CustomerManagedPolicyReference as CustomerManagedPolicyReference;
    impl crate::value::ToValue for CustomerManagedPolicyReference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sso-permissionset-permissionsboundary.html>
    pub struct PermissionsBoundary_ {
        pub customer_managed_policy_reference: Option<Box<CustomerManagedPolicyReference_>>,
        pub managed_policy_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sso_PermissionSet_PermissionsBoundary {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SSO::PermissionSet.PermissionsBoundary"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sso_PermissionSet_PermissionsBoundary as PermissionsBoundary;
    impl crate::value::ToValue for PermissionsBoundary_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customer_managed_policy_reference {
                properties.insert(
                    "CustomerManagedPolicyReference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.managed_policy_arn {
                properties.insert(
                    "ManagedPolicyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-application.html>
pub struct Application_ {
    pub application_provider_arn: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub instance_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub portal_options: Option<super::sso::application::PortalOptionsConfiguration_>,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sso_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SSO::Application"
        $($field $value)*)
    };
}
pub use crate::__aws_sso_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSO"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationProviderArn".to_string(),
            crate::value::ToValue::to_value(&self.application_provider_arn),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.portal_options {
            properties.insert(
                "PortalOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-applicationassignment.html>
pub struct ApplicationAssignment_ {
    pub application_arn: crate::value::ExpString,
    pub principal_id: crate::value::ExpString,
    pub principal_type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sso_ApplicationAssignment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SSO::ApplicationAssignment"
        $($field $value)*)
    };
}
pub use crate::__aws_sso_ApplicationAssignment as ApplicationAssignment;
impl crate::template::ToResource for ApplicationAssignment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSO"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ApplicationAssignment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationArn".to_string(),
            crate::value::ToValue::to_value(&self.application_arn),
        );
        properties.insert(
            "PrincipalId".to_string(),
            crate::value::ToValue::to_value(&self.principal_id),
        );
        properties.insert(
            "PrincipalType".to_string(),
            crate::value::ToValue::to_value(&self.principal_type),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-assignment.html>
pub struct Assignment_ {
    pub instance_arn: crate::value::ExpString,
    pub permission_set_arn: crate::value::ExpString,
    pub principal_id: crate::value::ExpString,
    pub principal_type: crate::value::ExpString,
    pub target_id: crate::value::ExpString,
    pub target_type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sso_Assignment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SSO::Assignment" $($field
        $value)*)
    };
}
pub use crate::__aws_sso_Assignment as Assignment;
impl crate::template::ToResource for Assignment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSO"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Assignment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        properties.insert(
            "PermissionSetArn".to_string(),
            crate::value::ToValue::to_value(&self.permission_set_arn),
        );
        properties.insert(
            "PrincipalId".to_string(),
            crate::value::ToValue::to_value(&self.principal_id),
        );
        properties.insert(
            "PrincipalType".to_string(),
            crate::value::ToValue::to_value(&self.principal_type),
        );
        properties.insert(
            "TargetId".to_string(),
            crate::value::ToValue::to_value(&self.target_id),
        );
        properties.insert(
            "TargetType".to_string(),
            crate::value::ToValue::to_value(&self.target_type),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-instance.html>
pub struct Instance_ {
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sso_Instance {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SSO::Instance" $($field
        $value)*)
    };
}
pub use crate::__aws_sso_Instance as Instance;
impl crate::template::ToResource for Instance_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSO"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Instance"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-instanceaccesscontrolattributeconfiguration.html>
pub struct InstanceAccessControlAttributeConfiguration_ {
    pub access_control_attributes: Option<
        Vec<super::sso::instanceaccesscontrolattributeconfiguration::AccessControlAttribute_>,
    >,
    pub instance_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sso_InstanceAccessControlAttributeConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SSO::InstanceAccessControlAttributeConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_sso_InstanceAccessControlAttributeConfiguration as InstanceAccessControlAttributeConfiguration;
impl crate::template::ToResource for InstanceAccessControlAttributeConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSO"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "InstanceAccessControlAttributeConfiguration",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_control_attributes {
            properties.insert(
                "AccessControlAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-permissionset.html>
pub struct PermissionSet_ {
    pub customer_managed_policy_references:
        Option<Vec<super::sso::permissionset::CustomerManagedPolicyReference_>>,
    pub description: Option<crate::value::ExpString>,
    pub inline_policy: Option<serde_json::Value>,
    pub instance_arn: crate::value::ExpString,
    pub managed_policies: Option<Vec<crate::value::ExpString>>,
    pub name: crate::value::ExpString,
    pub permissions_boundary: Option<super::sso::permissionset::PermissionsBoundary_>,
    pub relay_state_type: Option<crate::value::ExpString>,
    pub session_duration: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sso_PermissionSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SSO::PermissionSet"
        $($field $value)*)
    };
}
pub use crate::__aws_sso_PermissionSet as PermissionSet;
impl crate::template::ToResource for PermissionSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSO"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PermissionSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.customer_managed_policy_references {
            properties.insert(
                "CustomerManagedPolicyReferences".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.inline_policy {
            properties.insert(
                "InlinePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        if let Some(ref value) = self.managed_policies {
            properties.insert(
                "ManagedPolicies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.permissions_boundary {
            properties.insert(
                "PermissionsBoundary".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.relay_state_type {
            properties.insert(
                "RelayStateType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.session_duration {
            properties.insert(
                "SessionDuration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
