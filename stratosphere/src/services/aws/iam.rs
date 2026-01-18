pub mod group {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group-policy.html
    pub struct Policy_ {
        pub policy_document: serde_json::Value,
        pub policy_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iam_Group_Policy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IAM::Group.Policy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iam_Group_Policy as Policy;
    impl crate::value::ToValue for Policy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PolicyDocument".to_string(),
                crate::value::ToValue::to_value(&self.policy_document),
            );
            properties.insert(
                "PolicyName".to_string(),
                crate::value::ToValue::to_value(&self.policy_name),
            );
            properties.into()
        }
    }
}
pub mod role {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-role-policy.html
    pub struct Policy_ {
        pub policy_document: serde_json::Value,
        pub policy_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iam_Role_Policy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IAM::Role.Policy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iam_Role_Policy as Policy;
    impl crate::value::ToValue for Policy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PolicyDocument".to_string(),
                crate::value::ToValue::to_value(&self.policy_document),
            );
            properties.insert(
                "PolicyName".to_string(),
                crate::value::ToValue::to_value(&self.policy_name),
            );
            properties.into()
        }
    }
}
pub mod samlprovider {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-samlprovider-samlprivatekey.html
    pub struct SAMLPrivateKey_ {
        pub key_id: crate::value::ExpString,
        pub timestamp: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iam_SAMLProvider_SAMLPrivateKey {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IAM::SAMLProvider.SAMLPrivateKey"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iam_SAMLProvider_SAMLPrivateKey as SAMLPrivateKey;
    impl crate::value::ToValue for SAMLPrivateKey_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KeyId".to_string(),
                crate::value::ToValue::to_value(&self.key_id),
            );
            properties.insert(
                "Timestamp".to_string(),
                crate::value::ToValue::to_value(&self.timestamp),
            );
            properties.into()
        }
    }
}
pub mod user {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user-loginprofile.html
    pub struct LoginProfile_ {
        pub password: crate::value::ExpString,
        pub password_reset_required: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iam_User_LoginProfile {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IAM::User.LoginProfile"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iam_User_LoginProfile as LoginProfile;
    impl crate::value::ToValue for LoginProfile_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Password".to_string(),
                crate::value::ToValue::to_value(&self.password),
            );
            if let Some(ref value) = self.password_reset_required {
                properties.insert(
                    "PasswordResetRequired".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user-policy.html
    pub struct Policy_ {
        pub policy_document: serde_json::Value,
        pub policy_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iam_User_Policy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IAM::User.Policy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iam_User_Policy as Policy;
    impl crate::value::ToValue for Policy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PolicyDocument".to_string(),
                crate::value::ToValue::to_value(&self.policy_document),
            );
            properties.insert(
                "PolicyName".to_string(),
                crate::value::ToValue::to_value(&self.policy_name),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html
pub struct AccessKey_ {
    pub serial: Option<i64>,
    pub status: Option<crate::value::ExpString>,
    pub user_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iam_AccessKey {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IAM::AccessKey" $($field
        $value)*)
    };
}
pub use crate::__aws_iam_AccessKey as AccessKey;
impl crate::template::ToResource for AccessKey_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AccessKey"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.serial {
            properties.insert("Serial".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "UserName".to_string(),
            crate::value::ToValue::to_value(&self.user_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-group.html
pub struct Group_ {
    pub group_name: Option<crate::value::ExpString>,
    pub managed_policy_arns: Option<Vec<crate::value::ExpString>>,
    pub path: Option<crate::value::ExpString>,
    pub policies: Option<Vec<super::iam::group::Policy_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iam_Group {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IAM::Group" $($field
        $value)*)
    };
}
pub use crate::__aws_iam_Group as Group;
impl crate::template::ToResource for Group_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Group"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.group_name {
            properties.insert(
                "GroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.managed_policy_arns {
            properties.insert(
                "ManagedPolicyArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.path {
            properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.policies {
            properties.insert(
                "Policies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-grouppolicy.html
pub struct GroupPolicy_ {
    pub group_name: crate::value::ExpString,
    pub policy_document: Option<serde_json::Value>,
    pub policy_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iam_GroupPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IAM::GroupPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_iam_GroupPolicy as GroupPolicy;
impl crate::template::ToResource for GroupPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GroupPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "GroupName".to_string(),
            crate::value::ToValue::to_value(&self.group_name),
        );
        if let Some(ref value) = self.policy_document {
            properties.insert(
                "PolicyDocument".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PolicyName".to_string(),
            crate::value::ToValue::to_value(&self.policy_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html
pub struct InstanceProfile_ {
    pub instance_profile_name: Option<crate::value::ExpString>,
    pub path: Option<crate::value::ExpString>,
    pub roles: Vec<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iam_InstanceProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IAM::InstanceProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_iam_InstanceProfile as InstanceProfile;
impl crate::template::ToResource for InstanceProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InstanceProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.instance_profile_name {
            properties.insert(
                "InstanceProfileName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.path {
            properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Roles".to_string(),
            crate::value::ToValue::to_value(&self.roles),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-managedpolicy.html
pub struct ManagedPolicy_ {
    pub description: Option<crate::value::ExpString>,
    pub groups: Option<Vec<crate::value::ExpString>>,
    pub managed_policy_name: Option<crate::value::ExpString>,
    pub path: Option<crate::value::ExpString>,
    pub policy_document: serde_json::Value,
    pub roles: Option<Vec<crate::value::ExpString>>,
    pub users: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iam_ManagedPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IAM::ManagedPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_iam_ManagedPolicy as ManagedPolicy;
impl crate::template::ToResource for ManagedPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ManagedPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.groups {
            properties.insert("Groups".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.managed_policy_name {
            properties.insert(
                "ManagedPolicyName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.path {
            properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "PolicyDocument".to_string(),
            crate::value::ToValue::to_value(&self.policy_document),
        );
        if let Some(ref value) = self.roles {
            properties.insert("Roles".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.users {
            properties.insert("Users".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-oidcprovider.html
pub struct OIDCProvider_ {
    pub client_id_list: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub thumbprint_list: Option<Vec<crate::value::ExpString>>,
    pub url: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iam_OIDCProvider {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IAM::OIDCProvider"
        $($field $value)*)
    };
}
pub use crate::__aws_iam_OIDCProvider as OIDCProvider;
impl crate::template::ToResource for OIDCProvider_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OIDCProvider"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.client_id_list {
            properties.insert(
                "ClientIdList".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.thumbprint_list {
            properties.insert(
                "ThumbprintList".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.url {
            properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html
pub struct Policy_ {
    pub groups: Option<Vec<crate::value::ExpString>>,
    pub policy_document: serde_json::Value,
    pub policy_name: crate::value::ExpString,
    pub roles: Option<Vec<crate::value::ExpString>>,
    pub users: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iam_Policy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IAM::Policy" $($field
        $value)*)
    };
}
pub use crate::__aws_iam_Policy as Policy;
impl crate::template::ToResource for Policy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Policy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.groups {
            properties.insert("Groups".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "PolicyDocument".to_string(),
            crate::value::ToValue::to_value(&self.policy_document),
        );
        properties.insert(
            "PolicyName".to_string(),
            crate::value::ToValue::to_value(&self.policy_name),
        );
        if let Some(ref value) = self.roles {
            properties.insert("Roles".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.users {
            properties.insert("Users".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html
pub struct Role_ {
    pub assume_role_policy_document: serde_json::Value,
    pub description: Option<crate::value::ExpString>,
    pub managed_policy_arns: Option<Vec<crate::value::ExpString>>,
    pub max_session_duration: Option<i64>,
    pub path: Option<crate::value::ExpString>,
    pub permissions_boundary: Option<crate::value::ExpString>,
    pub policies: Option<Vec<super::iam::role::Policy_>>,
    pub role_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iam_Role {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IAM::Role" $($field
        $value)*)
    };
}
pub use crate::__aws_iam_Role as Role;
impl crate::template::ToResource for Role_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Role"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AssumeRolePolicyDocument".to_string(),
            crate::value::ToValue::to_value(&self.assume_role_policy_document),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.managed_policy_arns {
            properties.insert(
                "ManagedPolicyArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_session_duration {
            properties.insert(
                "MaxSessionDuration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.path {
            properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.permissions_boundary {
            properties.insert(
                "PermissionsBoundary".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policies {
            properties.insert(
                "Policies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.role_name {
            properties.insert(
                "RoleName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-rolepolicy.html
pub struct RolePolicy_ {
    pub policy_document: Option<serde_json::Value>,
    pub policy_name: crate::value::ExpString,
    pub role_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iam_RolePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IAM::RolePolicy" $($field
        $value)*)
    };
}
pub use crate::__aws_iam_RolePolicy as RolePolicy;
impl crate::template::ToResource for RolePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RolePolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.policy_document {
            properties.insert(
                "PolicyDocument".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PolicyName".to_string(),
            crate::value::ToValue::to_value(&self.policy_name),
        );
        properties.insert(
            "RoleName".to_string(),
            crate::value::ToValue::to_value(&self.role_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-samlprovider.html
pub struct SAMLProvider_ {
    pub add_private_key: Option<crate::value::ExpString>,
    pub assertion_encryption_mode: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub private_key_list: Option<Vec<super::iam::samlprovider::SAMLPrivateKey_>>,
    pub remove_private_key: Option<crate::value::ExpString>,
    pub saml_metadata_document: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iam_SAMLProvider {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IAM::SAMLProvider"
        $($field $value)*)
    };
}
pub use crate::__aws_iam_SAMLProvider as SAMLProvider;
impl crate::template::ToResource for SAMLProvider_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SAMLProvider"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.add_private_key {
            properties.insert(
                "AddPrivateKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.assertion_encryption_mode {
            properties.insert(
                "AssertionEncryptionMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.private_key_list {
            properties.insert(
                "PrivateKeyList".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.remove_private_key {
            properties.insert(
                "RemovePrivateKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.saml_metadata_document {
            properties.insert(
                "SamlMetadataDocument".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-servercertificate.html
pub struct ServerCertificate_ {
    pub certificate_body: Option<crate::value::ExpString>,
    pub certificate_chain: Option<crate::value::ExpString>,
    pub path: Option<crate::value::ExpString>,
    pub private_key: Option<crate::value::ExpString>,
    pub server_certificate_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iam_ServerCertificate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IAM::ServerCertificate"
        $($field $value)*)
    };
}
pub use crate::__aws_iam_ServerCertificate as ServerCertificate;
impl crate::template::ToResource for ServerCertificate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ServerCertificate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.certificate_body {
            properties.insert(
                "CertificateBody".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate_chain {
            properties.insert(
                "CertificateChain".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.path {
            properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.private_key {
            properties.insert(
                "PrivateKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.server_certificate_name {
            properties.insert(
                "ServerCertificateName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-servicelinkedrole.html
pub struct ServiceLinkedRole_ {
    pub aws_service_name: Option<crate::value::ExpString>,
    pub custom_suffix: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iam_ServiceLinkedRole {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IAM::ServiceLinkedRole"
        $($field $value)*)
    };
}
pub use crate::__aws_iam_ServiceLinkedRole as ServiceLinkedRole;
impl crate::template::ToResource for ServiceLinkedRole_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ServiceLinkedRole"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.aws_service_name {
            properties.insert(
                "AWSServiceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_suffix {
            properties.insert(
                "CustomSuffix".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-user.html
pub struct User_ {
    pub groups: Option<Vec<crate::value::ExpString>>,
    pub login_profile: Option<super::iam::user::LoginProfile_>,
    pub managed_policy_arns: Option<Vec<crate::value::ExpString>>,
    pub path: Option<crate::value::ExpString>,
    pub permissions_boundary: Option<crate::value::ExpString>,
    pub policies: Option<Vec<super::iam::user::Policy_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iam_User {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IAM::User" $($field
        $value)*)
    };
}
pub use crate::__aws_iam_User as User;
impl crate::template::ToResource for User_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("User"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.groups {
            properties.insert("Groups".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.login_profile {
            properties.insert(
                "LoginProfile".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.managed_policy_arns {
            properties.insert(
                "ManagedPolicyArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.path {
            properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.permissions_boundary {
            properties.insert(
                "PermissionsBoundary".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policies {
            properties.insert(
                "Policies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.user_name {
            properties.insert(
                "UserName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-userpolicy.html
pub struct UserPolicy_ {
    pub policy_document: Option<serde_json::Value>,
    pub policy_name: crate::value::ExpString,
    pub user_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iam_UserPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IAM::UserPolicy" $($field
        $value)*)
    };
}
pub use crate::__aws_iam_UserPolicy as UserPolicy;
impl crate::template::ToResource for UserPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.policy_document {
            properties.insert(
                "PolicyDocument".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PolicyName".to_string(),
            crate::value::ToValue::to_value(&self.policy_name),
        );
        properties.insert(
            "UserName".to_string(),
            crate::value::ToValue::to_value(&self.user_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html
pub struct UserToGroupAddition_ {
    pub group_name: crate::value::ExpString,
    pub users: Vec<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iam_UserToGroupAddition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IAM::UserToGroupAddition"
        $($field $value)*)
    };
}
pub use crate::__aws_iam_UserToGroupAddition as UserToGroupAddition;
impl crate::template::ToResource for UserToGroupAddition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserToGroupAddition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "GroupName".to_string(),
            crate::value::ToValue::to_value(&self.group_name),
        );
        properties.insert(
            "Users".to_string(),
            crate::value::ToValue::to_value(&self.users),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-virtualmfadevice.html
pub struct VirtualMFADevice_ {
    pub path: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub users: Vec<crate::value::ExpString>,
    pub virtual_mfa_device_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iam_VirtualMFADevice {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IAM::VirtualMFADevice"
        $($field $value)*)
    };
}
pub use crate::__aws_iam_VirtualMFADevice as VirtualMFADevice;
impl crate::template::ToResource for VirtualMFADevice_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VirtualMFADevice"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.path {
            properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Users".to_string(),
            crate::value::ToValue::to_value(&self.users),
        );
        if let Some(ref value) = self.virtual_mfa_device_name {
            properties.insert(
                "VirtualMfaDeviceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
