#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct ResourceSpecification {
    resource_types: ResourceTypes,
    property_types: PropertyTypes,
    resource_specification_version: ResourceSpecificationVersion,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ResourceTypes(std::collections::BTreeMap<ResourceTypeName, ResourceType>);

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ResourceTypeName {
    vendor: String,
    service: String,
    resource: String,
}

impl<'de> serde::Deserialize<'de> for ResourceTypeName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ResourceTypeNameVisitor;

        impl<'de> serde::de::Visitor<'de> for ResourceTypeNameVisitor {
            type Value = ResourceTypeName;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string ResourceTypeName")
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                let parts: Vec<&str> = value.split("::").collect();

                if parts.len() == 3 {
                    Ok(ResourceTypeName {
                        vendor: parts[0].to_string(),
                        service: parts[1].to_string(),
                        resource: parts[2].to_string(),
                    })
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(value),
                        &self,
                    ))
                }
            }
        }

        deserializer.deserialize_struct(
            "ResourceTypeName",
            &["vendor", "service", "resource"],
            ResourceTypeNameVisitor,
        )
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
struct ResourceAttributeName(String);

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
struct TypeReference(String);

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
struct PropertyTypeName(String);

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
struct ResourceTypePropertyName(String);

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct ResourceType {
    documentation: Documentation,
    attributes: Option<ResourceAttributes>,
    additional_properties: Option<bool>,
    properties: ResourceTypeProperties,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ResourceAttributes(std::collections::BTreeMap<ResourceAttributeName, ResourceAttribute>);

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct ResourceAttribute {
    primitive_item_type: Option<PrimitiveItemType>,
    item_type: Option<TypeReference>,
    primitive_type: Option<PrimitiveType>,
    r#type: Option<TypeReference>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ResourceTypeProperties(
    std::collections::BTreeMap<ResourceTypePropertyName, ResourceTypeProperty>,
);

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct ResourceTypeProperty {
    documentation: Documentation,
    duplicates_allowed: Option<bool>,
    item_type: Option<TypeReference>,
    primitive_type: Option<PrimitiveType>,
    primitive_item_type: Option<PrimitiveItemType>,
    r#type: Option<TypeReference>,
    required: bool,
    update_type: UpdateType,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct PropertyType {
    documentation: Documentation,
    item_type: Option<TypeReference>,
    properties: Option<PropertyTypeProperties>,
    r#type: Option<TypeReference>,
    primitive_type: Option<PrimitiveType>,
    required: Option<bool>,
    update_type: Option<UpdateType>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct PropertyTypeProperties(std::collections::BTreeMap<String, PropertyTypeProperty>);

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct PropertyTypeProperty {
    documentation: Documentation,
    duplicates_allowed: Option<bool>,
    item_type: Option<TypeReference>,
    primitive_item_type: Option<PrimitiveItemType>,
    primitive_type: Option<PrimitiveType>,
    r#type: Option<TypeReference>,
    required: bool,
    update_type: UpdateType,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
enum UpdateType {
    Conditional,
    Immutable,
    Mutable,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
enum PrimitiveType {
    Boolean,
    Double,
    Integer,
    Json,
    Long,
    String,
    Timestamp,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
enum PrimitiveItemType {
    Double,
    Integer,
    Json,
    String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PropertyTypes(std::collections::BTreeMap<PropertyTypeName, PropertyType>);

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Documentation(String);

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ResourceSpecificationVersion(String);

impl ResourceSpecification {
    fn load_from_file() -> ResourceSpecification {
        match serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_slice(
            &std::fs::read("CloudFormationResourceSpecification.json").unwrap(),
        )) {
            Ok(value) => value,
            Err(error) => panic!("{}: {}", error.path(), error.inner()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_resource_specification() {
        eprintln!(
            "{:#?}",
            ResourceSpecification::load_from_file().resource_types
        );

        assert!(false);
    }
}
