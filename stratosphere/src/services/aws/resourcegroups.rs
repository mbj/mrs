pub mod group {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resourcegroups-group-configurationitem.html>
    pub struct ConfigurationItem_ {
        pub parameters: Option<Vec<ConfigurationParameter_>>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_resourcegroups_Group_ConfigurationItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ResourceGroups::Group.ConfigurationItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_resourcegroups_Group_ConfigurationItem as ConfigurationItem;
    impl crate::value::ToValue for ConfigurationItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resourcegroups-group-configurationparameter.html>
    pub struct ConfigurationParameter_ {
        pub name: Option<crate::value::ExpString>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_resourcegroups_Group_ConfigurationParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ResourceGroups::Group.ConfigurationParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_resourcegroups_Group_ConfigurationParameter as ConfigurationParameter;
    impl crate::value::ToValue for ConfigurationParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resourcegroups-group-query.html>
    pub struct Query_ {
        pub resource_type_filters: Option<Vec<crate::value::ExpString>>,
        pub stack_identifier: Option<crate::value::ExpString>,
        pub tag_filters: Option<Vec<TagFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_resourcegroups_Group_Query {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ResourceGroups::Group.Query"
            $($field $value)*)
        };
    }
    pub use crate::__aws_resourcegroups_Group_Query as Query;
    impl crate::value::ToValue for Query_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.resource_type_filters {
                properties.insert(
                    "ResourceTypeFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stack_identifier {
                properties.insert(
                    "StackIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_filters {
                properties.insert(
                    "TagFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resourcegroups-group-resourcequery.html>
    pub struct ResourceQuery_ {
        pub query: Option<Box<Query_>>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_resourcegroups_Group_ResourceQuery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ResourceGroups::Group.ResourceQuery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_resourcegroups_Group_ResourceQuery as ResourceQuery;
    impl crate::value::ToValue for ResourceQuery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.query {
                properties.insert("Query".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-resourcegroups-group-tagfilter.html>
    pub struct TagFilter_ {
        pub key: Option<crate::value::ExpString>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_resourcegroups_Group_TagFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ResourceGroups::Group.TagFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_resourcegroups_Group_TagFilter as TagFilter;
    impl crate::value::ToValue for TagFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resourcegroups-group.html>
pub struct Group_ {
    pub configuration: Option<Vec<super::resourcegroups::group::ConfigurationItem_>>,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub resource_query: Option<super::resourcegroups::group::ResourceQuery_>,
    pub resources: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_resourcegroups_Group {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ResourceGroups::Group"
        $($field $value)*)
    };
}
pub use crate::__aws_resourcegroups_Group as Group;
impl crate::template::ToResource for Group_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ResourceGroups"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Group"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.configuration {
            properties.insert(
                "Configuration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
        if let Some(ref value) = self.resource_query {
            properties.insert(
                "ResourceQuery".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resources {
            properties.insert(
                "Resources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-resourcegroups-tagsynctask.html>
pub struct TagSyncTask_ {
    pub group: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub tag_key: crate::value::ExpString,
    pub tag_value: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_resourcegroups_TagSyncTask {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ResourceGroups::TagSyncTask"
        $($field $value)*)
    };
}
pub use crate::__aws_resourcegroups_TagSyncTask as TagSyncTask;
impl crate::template::ToResource for TagSyncTask_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ResourceGroups"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TagSyncTask"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Group".to_string(),
            crate::value::ToValue::to_value(&self.group),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties.insert(
            "TagKey".to_string(),
            crate::value::ToValue::to_value(&self.tag_key),
        );
        properties.insert(
            "TagValue".to_string(),
            crate::value::ToValue::to_value(&self.tag_value),
        );
        properties
    }
}
