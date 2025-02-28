#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ResourceSpecification {
    pub resource_types: ResourceTypes,
    pub property_types: PropertyTypes,
    pub resource_specification_version: ResourceSpecificationVersion,
}

impl ResourceSpecification {
    pub fn services(&self) -> impl Iterator<Item = &ServiceName> {
        self.resource_types.keys().map(|resource_type_name| &resource_type_name.service)
    }
}

impl ResourceSpecification {
    fn load_from_file() -> ResourceSpecification {
        serde_json::from_slice(
            include_bytes!("../CloudFormationResourceSpecification.json")
        ).unwrap()
    }
}

static INSTANCE: std::sync::LazyLock<ResourceSpecification> =
    std::sync::LazyLock::new(|| ResourceSpecification::load_from_file());

pub fn instance() -> &'static ResourceSpecification {
    &*INSTANCE
}

pub type PropertyTypes = std::collections::BTreeMap<PropertyTypeName, PropertyType>;
pub type ResourceAttributes = std::collections::BTreeMap<ResourceAttributeName, ResourceAttribute>;
pub type ResourceTypes = std::collections::BTreeMap<ResourceTypeName, ResourceType>;

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ResourceTypeProperties(
    std::collections::BTreeMap<ResourceTypePropertyName, ResourceTypeProperty>,
);

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct PropertyTypeProperties(
    std::collections::BTreeMap<PropertyTypePropertyName, PropertyTypeProperty>,
);

/// Macro to generate `std::str::FromStr` for string wrapped newtypes
macro_rules! identifier {
    ($struct: ident) => {
	#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
	pub struct $struct(String);

        impl AsRef<str> for $struct {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl std::str::FromStr for $struct {
            type Err = String;

            fn from_str(value: &str) -> Result<$struct, Self::Err> {
                let count = value.chars().count();

                if count < 1 {
                    return Err(concat!(
                        stringify!($struct),
                        " min length: 1 violated"
                    ).to_string());
                }

                if count > 128 {
                    return Err(concat!(
                        stringify!($struct),
                        " max length: 128 violated",
                    ).to_string());
                }

                let syntax = r"\A[a-zA-Z]+[a-zA-Z0-9]*\z";

                let pattern = regex_lite::Regex::new(syntax).unwrap();

                if !pattern.is_match(value) {
                    return Err(
                        format!(
                            concat!(
                                stringify!($struct),
                                " does not match pattern: {}",
                            ),
                            syntax
                        )
                    );
                }

                Ok(Self(value.to_string()))
            }
        }

        impl<'de> serde::Deserialize<'de> for $struct {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                <String as serde::de::Deserialize<'de>>::deserialize(deserializer).and_then(
                    |value| {
                        <$struct as std::str::FromStr>::from_str(&value)
                            .map_err(serde::de::Error::custom)
                    },
                )
            }
        }
    };
}

identifier!(ResourceAttributeName);
identifier!(TypeReference);
identifier!(ResourceTypePropertyName);
identifier!(PropertyTypePropertyName);
identifier!(VendorName);
identifier!(ServiceName);

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
pub struct ResourceName(pub String);

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct Documentation(pub String);

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ResourceSpecificationVersion(pub String);

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ResourceTypeName {
    pub vendor: VendorName,
    pub service: ServiceName,
    pub resource: ResourceName,
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
                        vendor: VendorName(parts[0].to_string()),
                        service: ServiceName(parts[1].to_string()),
                        resource: ResourceName(parts[2].to_string()),
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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PropertyTypeName {
    PropertyTypeName {
        vendor: VendorName,
        service: ServiceName,
        resource: ResourceName,
    },
    Tag
}

impl<'de> serde::Deserialize<'de> for PropertyTypeName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct PropertyTypeNameVisitor;

        impl<'de> serde::de::Visitor<'de> for PropertyTypeNameVisitor {
            type Value = PropertyTypeName;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string PropertyTypeName")
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                if value == "Tag" {
                    Ok(PropertyTypeName::Tag)
                } else {
                    // TODO: Change to regexp
                    let parts: Vec<&str> = value.split("::").collect();

                    if parts.len() == 3 {
                        Ok(PropertyTypeName::PropertyTypeName {
                            vendor: VendorName(parts[0].to_string()),
                            service: ServiceName(parts[1].to_string()),
                            resource: ResourceName(parts[2].to_string()),
                        })
                    } else {
                        Err(serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(value),
                            &self,
                        ))
                    }
                }
            }
        }

        deserializer.deserialize_struct(
            "PropertyTypeName",
            &["vendor", "service", "resource"],
            PropertyTypeNameVisitor,
        )
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ResourceType {
    pub documentation: Documentation,
    pub attributes: Option<ResourceAttributes>,
    pub additional_properties: Option<bool>,
    pub properties: ResourceTypeProperties,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ResourceAttribute {
    pub primitive_item_type: Option<PrimitiveItemType>,
    pub item_type: Option<TypeReference>,
    pub primitive_type: Option<PrimitiveType>,
    pub r#type: Option<TypeReference>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ResourceTypeProperty {
    pub documentation: Documentation,
    pub duplicates_allowed: Option<bool>,
    pub item_type: Option<TypeReference>,
    pub primitive_type: Option<PrimitiveType>,
    pub primitive_item_type: Option<PrimitiveItemType>,
    pub r#type: Option<TypeReference>,
    pub required: bool,
    pub update_type: UpdateType,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct PropertyType {
    pub documentation: Documentation,
    pub item_type: Option<TypeReference>,
    pub properties: Option<PropertyTypeProperties>,
    pub r#type: Option<TypeReference>,
    pub primitive_type: Option<PrimitiveType>,
    pub required: Option<bool>,
    pub update_type: Option<UpdateType>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct PropertyTypeProperty {
    pub documentation: Documentation,
    pub duplicates_allowed: Option<bool>,
    pub item_type: Option<TypeReference>,
    pub primitive_item_type: Option<PrimitiveItemType>,
    pub primitive_type: Option<PrimitiveType>,
    pub r#type: Option<TypeReference>,
    pub required: bool,
    pub update_type: UpdateType,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub enum UpdateType {
    Conditional,
    Immutable,
    Mutable,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub enum PrimitiveType {
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
pub enum PrimitiveItemType {
    Double,
    Integer,
    Json,
    String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_resource_specification() {
        eprintln!("{:#?}", &*INSTANCE);
        assert!(false);
    }
}
