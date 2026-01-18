pub mod view {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resourceexplorer2-view-includedproperty.html
    pub struct IncludedProperty_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_resourceexplorer2_View_IncludedProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ResourceExplorer2::View.IncludedProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_resourceexplorer2_View_IncludedProperty as IncludedProperty;
    impl crate::value::ToValue for IncludedProperty_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resourceexplorer2-view-searchfilter.html
    pub struct SearchFilter_ {
        pub filter_string: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_resourceexplorer2_View_SearchFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ResourceExplorer2::View.SearchFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_resourceexplorer2_View_SearchFilter as SearchFilter;
    impl crate::value::ToValue for SearchFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FilterString".to_string(),
                crate::value::ToValue::to_value(&self.filter_string),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resourceexplorer2-defaultviewassociation.html
pub struct DefaultViewAssociation_ {
    pub view_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_resourceexplorer2_DefaultViewAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ResourceExplorer2::DefaultViewAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_resourceexplorer2_DefaultViewAssociation as DefaultViewAssociation;
impl crate::template::ToResource for DefaultViewAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ResourceExplorer2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DefaultViewAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ViewArn".to_string(),
            crate::value::ToValue::to_value(&self.view_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resourceexplorer2-index.html
pub struct Index_ {
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_resourceexplorer2_Index {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ResourceExplorer2::Index"
        $($field $value)*)
    };
}
pub use crate::__aws_resourceexplorer2_Index as Index;
impl crate::template::ToResource for Index_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ResourceExplorer2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Index"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resourceexplorer2-view.html
pub struct View_ {
    pub filters: Option<super::resourceexplorer2::view::SearchFilter_>,
    pub included_properties: Option<Vec<super::resourceexplorer2::view::IncludedProperty_>>,
    pub scope: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub view_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_resourceexplorer2_View {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ResourceExplorer2::View"
        $($field $value)*)
    };
}
pub use crate::__aws_resourceexplorer2_View as View;
impl crate::template::ToResource for View_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ResourceExplorer2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("View"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.filters {
            properties.insert(
                "Filters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.included_properties {
            properties.insert(
                "IncludedProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scope {
            properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "ViewName".to_string(),
            crate::value::ToValue::to_value(&self.view_name),
        );
        properties
    }
}
