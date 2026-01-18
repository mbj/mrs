pub mod discoverer {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eventschemas-discoverer-tagsentry.html
    pub struct TagsEntry_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eventschemas_Discoverer_TagsEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EventSchemas::Discoverer.TagsEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eventschemas_Discoverer_TagsEntry as TagsEntry;
    impl crate::value::ToValue for TagsEntry_ {
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
}
pub mod registry {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eventschemas-registry-tagsentry.html
    pub struct TagsEntry_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eventschemas_Registry_TagsEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EventSchemas::Registry.TagsEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eventschemas_Registry_TagsEntry as TagsEntry;
    impl crate::value::ToValue for TagsEntry_ {
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
}
pub mod schema {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-eventschemas-schema-tagsentry.html
    pub struct TagsEntry_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_eventschemas_Schema_TagsEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EventSchemas::Schema.TagsEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_eventschemas_Schema_TagsEntry as TagsEntry;
    impl crate::value::ToValue for TagsEntry_ {
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
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eventschemas-discoverer.html
pub struct Discoverer_ {
    pub cross_account: Option<crate::value::ExpBool>,
    pub description: Option<crate::value::ExpString>,
    pub source_arn: crate::value::ExpString,
    pub tags: Option<Vec<super::eventschemas::discoverer::TagsEntry_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_eventschemas_Discoverer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EventSchemas::Discoverer"
        $($field $value)*)
    };
}
pub use crate::__aws_eventschemas_Discoverer as Discoverer;
impl crate::template::ToResource for Discoverer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EventSchemas"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Discoverer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cross_account {
            properties.insert(
                "CrossAccount".to_string(),
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
            "SourceArn".to_string(),
            crate::value::ToValue::to_value(&self.source_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eventschemas-registry.html
pub struct Registry_ {
    pub description: Option<crate::value::ExpString>,
    pub registry_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<super::eventschemas::registry::TagsEntry_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_eventschemas_Registry {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EventSchemas::Registry"
        $($field $value)*)
    };
}
pub use crate::__aws_eventschemas_Registry as Registry;
impl crate::template::ToResource for Registry_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EventSchemas"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Registry"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.registry_name {
            properties.insert(
                "RegistryName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eventschemas-registrypolicy.html
pub struct RegistryPolicy_ {
    pub policy: serde_json::Value,
    pub registry_name: crate::value::ExpString,
    pub revision_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_eventschemas_RegistryPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EventSchemas::RegistryPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_eventschemas_RegistryPolicy as RegistryPolicy;
impl crate::template::ToResource for RegistryPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EventSchemas"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RegistryPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Policy".to_string(),
            crate::value::ToValue::to_value(&self.policy),
        );
        properties.insert(
            "RegistryName".to_string(),
            crate::value::ToValue::to_value(&self.registry_name),
        );
        if let Some(ref value) = self.revision_id {
            properties.insert(
                "RevisionId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-eventschemas-schema.html
pub struct Schema_ {
    pub content: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub registry_name: crate::value::ExpString,
    pub schema_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<super::eventschemas::schema::TagsEntry_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_eventschemas_Schema {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EventSchemas::Schema"
        $($field $value)*)
    };
}
pub use crate::__aws_eventschemas_Schema as Schema;
impl crate::template::ToResource for Schema_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EventSchemas"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Schema"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Content".to_string(),
            crate::value::ToValue::to_value(&self.content),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RegistryName".to_string(),
            crate::value::ToValue::to_value(&self.registry_name),
        );
        if let Some(ref value) = self.schema_name {
            properties.insert(
                "SchemaName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
