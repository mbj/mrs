///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53profiles-profile.html
pub struct Profile_ {
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53profiles_Profile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Route53Profiles::Profile"
        $($field $value)*)
    };
}
pub use crate::__aws_route53profiles_Profile as Profile;
impl crate::template::ToResource for Profile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53Profiles"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Profile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53profiles-profileassociation.html
pub struct ProfileAssociation_ {
    pub arn: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub profile_id: crate::value::ExpString,
    pub resource_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53profiles_ProfileAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Route53Profiles::ProfileAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_route53profiles_ProfileAssociation as ProfileAssociation;
impl crate::template::ToResource for ProfileAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53Profiles"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ProfileAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.arn {
            properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "ProfileId".to_string(),
            crate::value::ToValue::to_value(&self.profile_id),
        );
        properties.insert(
            "ResourceId".to_string(),
            crate::value::ToValue::to_value(&self.resource_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53profiles-profileresourceassociation.html
pub struct ProfileResourceAssociation_ {
    pub name: crate::value::ExpString,
    pub profile_id: crate::value::ExpString,
    pub resource_arn: crate::value::ExpString,
    pub resource_properties: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53profiles_ProfileResourceAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Route53Profiles::ProfileResourceAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_route53profiles_ProfileResourceAssociation as ProfileResourceAssociation;
impl crate::template::ToResource for ProfileResourceAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53Profiles"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ProfileResourceAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "ProfileId".to_string(),
            crate::value::ToValue::to_value(&self.profile_id),
        );
        properties.insert(
            "ResourceArn".to_string(),
            crate::value::ToValue::to_value(&self.resource_arn),
        );
        if let Some(ref value) = self.resource_properties {
            properties.insert(
                "ResourceProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
