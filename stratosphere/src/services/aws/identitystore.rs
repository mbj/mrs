pub mod groupmembership {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-identitystore-groupmembership-memberid.html
    pub struct MemberId_ {
        pub user_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_identitystore_GroupMembership_MemberId {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IdentityStore::GroupMembership.MemberId"
            $($field $value)*)
        };
    }
    pub use crate::__aws_identitystore_GroupMembership_MemberId as MemberId;
    impl crate::value::ToValue for MemberId_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "UserId".to_string(),
                crate::value::ToValue::to_value(&self.user_id),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-identitystore-group.html
pub struct Group_ {
    pub description: Option<crate::value::ExpString>,
    pub display_name: crate::value::ExpString,
    pub identity_store_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_identitystore_Group {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IdentityStore::Group"
        $($field $value)*)
    };
}
pub use crate::__aws_identitystore_Group as Group;
impl crate::template::ToResource for Group_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IdentityStore"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Group"),
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
            "DisplayName".to_string(),
            crate::value::ToValue::to_value(&self.display_name),
        );
        properties.insert(
            "IdentityStoreId".to_string(),
            crate::value::ToValue::to_value(&self.identity_store_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-identitystore-groupmembership.html
pub struct GroupMembership_ {
    pub group_id: crate::value::ExpString,
    pub identity_store_id: crate::value::ExpString,
    pub member_id: super::identitystore::groupmembership::MemberId_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_identitystore_GroupMembership {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IdentityStore::GroupMembership"
        $($field $value)*)
    };
}
pub use crate::__aws_identitystore_GroupMembership as GroupMembership;
impl crate::template::ToResource for GroupMembership_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IdentityStore"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GroupMembership"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "GroupId".to_string(),
            crate::value::ToValue::to_value(&self.group_id),
        );
        properties.insert(
            "IdentityStoreId".to_string(),
            crate::value::ToValue::to_value(&self.identity_store_id),
        );
        properties.insert(
            "MemberId".to_string(),
            crate::value::ToValue::to_value(&self.member_id),
        );
        properties
    }
}
