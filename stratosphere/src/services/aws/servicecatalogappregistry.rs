///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalogappregistry-application.html>
pub struct Application_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalogappregistry_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalogAppRegistry::Application"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalogappregistry_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName(
                    "ServiceCatalogAppRegistry",
                ),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
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
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalogappregistry-attributegroup.html>
pub struct AttributeGroup_ {
    pub attributes: serde_json::Value,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalogappregistry_AttributeGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalogAppRegistry::AttributeGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalogappregistry_AttributeGroup as AttributeGroup;
impl crate::template::ToResource for AttributeGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName(
                    "ServiceCatalogAppRegistry",
                ),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AttributeGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Attributes".to_string(),
            crate::value::ToValue::to_value(&self.attributes),
        );
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
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalogappregistry-attributegroupassociation.html>
pub struct AttributeGroupAssociation_ {
    pub application: crate::value::ExpString,
    pub attribute_group: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalogappregistry_AttributeGroupAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalogAppRegistry::AttributeGroupAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalogappregistry_AttributeGroupAssociation as AttributeGroupAssociation;
impl crate::template::ToResource for AttributeGroupAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName(
                    "ServiceCatalogAppRegistry",
                ),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AttributeGroupAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Application".to_string(),
            crate::value::ToValue::to_value(&self.application),
        );
        properties.insert(
            "AttributeGroup".to_string(),
            crate::value::ToValue::to_value(&self.attribute_group),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicecatalogappregistry-resourceassociation.html>
pub struct ResourceAssociation_ {
    pub application: crate::value::ExpString,
    pub resource: crate::value::ExpString,
    pub resource_type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicecatalogappregistry_ResourceAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceCatalogAppRegistry::ResourceAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_servicecatalogappregistry_ResourceAssociation as ResourceAssociation;
impl crate::template::ToResource for ResourceAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName(
                    "ServiceCatalogAppRegistry",
                ),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourceAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Application".to_string(),
            crate::value::ToValue::to_value(&self.application),
        );
        properties.insert(
            "Resource".to_string(),
            crate::value::ToValue::to_value(&self.resource),
        );
        properties.insert(
            "ResourceType".to_string(),
            crate::value::ToValue::to_value(&self.resource_type),
        );
        properties
    }
}
