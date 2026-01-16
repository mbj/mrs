///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ram-permission.html
pub struct Permission_ {
    pub name: crate::value::ExpString,
    pub policy_template: serde_json::Value,
    pub resource_type: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ram_Permission {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::RAM::Permission" $($field
        $value)*)
    };
}
pub use crate::__aws_ram_Permission as Permission;
impl crate::template::ToResource for Permission_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Permission"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "PolicyTemplate".to_string(),
            crate::value::ToValue::to_value(&self.policy_template),
        );
        properties.insert(
            "ResourceType".to_string(),
            crate::value::ToValue::to_value(&self.resource_type),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ram-resourceshare.html
pub struct ResourceShare_ {
    pub allow_external_principals: Option<crate::value::ExpBool>,
    pub name: crate::value::ExpString,
    pub permission_arns: Option<Vec<crate::value::ExpString>>,
    pub principals: Option<Vec<crate::value::ExpString>>,
    pub resource_arns: Option<Vec<crate::value::ExpString>>,
    pub sources: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ram_ResourceShare {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::RAM::ResourceShare"
        $($field $value)*)
    };
}
pub use crate::__aws_ram_ResourceShare as ResourceShare;
impl crate::template::ToResource for ResourceShare_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RAM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourceShare"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allow_external_principals {
            properties.insert(
                "AllowExternalPrincipals".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.permission_arns {
            properties.insert(
                "PermissionArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.principals {
            properties.insert(
                "Principals".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_arns {
            properties.insert(
                "ResourceArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sources {
            properties.insert(
                "Sources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
