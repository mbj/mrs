use std::collections::BTreeMap;

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ResourceSpecification<'a> {
    #[serde(borrow = "'a")]
    pub property_types: PropertyTypeMap<'a>,
    pub resource_specification_version: ResourceSpecificationVersion<'a>,
    pub resource_types: ResourceTypeMap<'a>,
}

impl ResourceSpecification<'_> {
    fn load_from_file() -> ResourceSpecification<'static> {
        serde_json::from_slice(include_bytes!(
            "../CloudFormationResourceSpecification.json"
        ))
        .unwrap()
    }
}

static INSTANCE: std::sync::LazyLock<ResourceSpecification> =
    std::sync::LazyLock::new(ResourceSpecification::load_from_file);

pub fn instance() -> &'static ResourceSpecification<'static> {
    &INSTANCE
}

pub type PropertyTypeMap<'a> = BTreeMap<PropertyTypeName<'a>, PropertyType<'a>>;
pub type ResourceAttributesMap<'a> = BTreeMap<ResourceAttributeName<'a>, ResourceAttribute<'a>>;
pub type ResourceTypeMap<'a> = BTreeMap<ResourceTypeName<'a>, ResourceType<'a>>;
pub type ResourceTypePropertiesMap<'a> =
    BTreeMap<ResourceTypePropertyName<'a>, ResourceTypeProperty<'a>>;

pub type PropertyTypePropertiesMap<'a> = BTreeMap<PropertyName<'a>, PropertyTypeProperty<'a>>;

/// Macro to generate `std::str::FromStr` for zero copy str wrapped newtypes
macro_rules! identifier {
    ($struct: ident) => {
        identifier!($struct, r#"[a-zA-Z]+[a-zA-Z0-9]*"#);
    };
    ($struct: ident, $pattern: literal) => {
        #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
        pub struct $struct<'a>(pub &'a str);

        impl $struct<'_> {
            pub fn as_str(&self) -> &str {
                self.0
            }

            #[allow(unused)]
            const BASE_PATTERN: &'static str = $pattern;
        }

        impl AsRef<str> for $struct<'_> {
            fn as_ref(&self) -> &str {
                self.0
            }
        }

        impl<'a> std::convert::TryFrom<&'a str> for $struct<'a> {
            type Error = String;

            fn try_from(value: &'a str) -> Result<Self, Self::Error> {
                let count = value.chars().count();

                if count < 1 {
                    return Err(concat!(stringify!($struct), " min length: 1 violated").to_string());
                }

                if count > 128 {
                    return Err(
                        concat!(stringify!($struct), " max length: 128 violated",).to_string()
                    );
                }

                let syntax = concat!(r#"\A"#, $pattern, r#"\z"#);

                let pattern = regex_lite::Regex::new(syntax).unwrap();

                if !pattern.is_match(value) {
                    return Err(format!(
                        concat!(
                            stringify!($struct),
                            " does not match pattern: {}, value: {}",
                        ),
                        syntax, value
                    ));
                }

                Ok(Self(value))
            }
        }

        impl std::fmt::Display for $struct<'_> {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                write!(formatter, "{}", self.0)
            }
        }
    };
}

identifier!(
    ResourceAttributeName,
    r#"\A[a-zA-Z]+[a-zA-Z0-9]*(\.[a-zA-Z]+[a-zA-Z0-9]*)*\z"#
);
identifier!(ResourceName);
identifier!(ResourceTypePropertyName);
identifier!(ServiceName);
identifier!(PropertyName);
identifier!(VendorName);

impl quote::ToTokens for ServiceName<'_> {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        let str_value = self.as_str();

        stream.extend(quote::quote! {
            stratosphere::resource_specification::ServiceName(#str_value)
        })
    }
}

impl quote::ToTokens for ResourceName<'_> {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        let str_value = self.as_str();

        stream.extend(quote::quote! {
            stratosphere::resource_specification::ResourceName(#str_value)
        })
    }
}

impl quote::ToTokens for VendorName<'_> {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        let str_value = self.as_str();

        stream.extend(quote::quote! {
            stratosphere::resource_specification::VendorName(#str_value)
        })
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Documentation<'a>(pub &'a str);

impl Documentation<'_> {
    pub fn as_str(&self) -> &str {
        self.0
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ResourceSpecificationVersion<'a>(pub &'a str);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ServiceIdentifier<'a> {
    pub vendor_name: VendorName<'a>,
    pub service_name: ServiceName<'a>,
}

impl ServiceIdentifier<'_> {
    pub fn provides(&self, resource_type: &ResourceTypeName) -> bool {
        *self == resource_type.service
    }
}

impl quote::ToTokens for ServiceIdentifier<'_> {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        let vendor_name = &self.vendor_name;
        let service_name = &self.service_name;

        stream.extend(quote::quote! {
            stratosphere::resource_specification::ServiceIdentifier {
                service_name: #service_name,
                vendor_name: #vendor_name,
            }
        })
    }
}

impl std::fmt::Display for ServiceIdentifier<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}::{}", self.vendor_name, self.service_name)
    }
}

impl<'a> std::convert::TryFrom<&'a str> for ServiceIdentifier<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let pattern = regex_lite::Regex::new(&format!(
            r#"\A(?<vendor_name>{})::(?<service_name>{})\z"#,
            VendorName::BASE_PATTERN,
            ServiceName::BASE_PATTERN,
        ))
        .unwrap();

        if let Some(captures) = pattern.captures(value) {
            Ok(ServiceIdentifier {
                vendor_name: VendorName(captures.name("vendor_name").unwrap().as_str()),
                service_name: ServiceName(captures.name("service_name").unwrap().as_str()),
            })
        } else {
            Err(format!("Invalid value: {value}"))
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ResourceTypeName<'a> {
    pub service: ServiceIdentifier<'a>,
    pub resource_name: ResourceName<'a>,
}

impl serde::Serialize for ResourceTypeName<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl quote::ToTokens for ResourceTypeName<'_> {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        let service = &self.service;
        let resource_name = &self.resource_name;

        stream.extend(quote::quote! {
            stratosphere::resource_specification::ResourceTypeName {
                service: #service,
                resource_name: #resource_name,
            }
        })
    }
}

impl<'a> std::convert::TryFrom<&'a str> for ResourceTypeName<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let pattern = regex_lite::Regex::new(&format!(
            r#"\A(?<vendor_name>{})::(?<service_name>{})::(?<resource_name>{})\z"#,
            VendorName::BASE_PATTERN,
            ServiceName::BASE_PATTERN,
            ResourceName::BASE_PATTERN,
        ))
        .unwrap();

        if let Some(captures) = pattern.captures(value) {
            Ok(ResourceTypeName {
                service: ServiceIdentifier {
                    vendor_name: VendorName(captures.name("vendor_name").unwrap().as_str()),
                    service_name: ServiceName(captures.name("service_name").unwrap().as_str()),
                },
                resource_name: ResourceName(captures.name("resource_name").unwrap().as_str()),
            })
        } else {
            Err(format!("Invalid value: {value}"))
        }
    }
}

impl<'a, 'de: 'a> serde::Deserialize<'de> for ResourceTypeName<'a> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <&'a str as serde::de::Deserialize<'de>>::deserialize(deserializer)
            .and_then(|value| Self::try_from(value).map_err(serde::de::Error::custom))
    }
}

impl std::fmt::Display for ResourceTypeName<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}::{}", self.service, self.resource_name)
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PropertyTypeName<'a> {
    PropertyTypeName(ResourcePropertyTypeName<'a>),
    Tag,
}

impl std::fmt::Display for PropertyTypeName<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::PropertyTypeName(name) => {
                write!(
                    formatter,
                    "{}::{}::{}.{}",
                    name.vendor_name, name.service_name, name.resource_name, name.property_name
                )
            }
            Self::Tag => write!(formatter, "Tag"),
        }
    }
}

impl<'a> std::convert::TryFrom<&'a str> for PropertyTypeName<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value == "Tag" {
            Ok(PropertyTypeName::Tag)
        } else {
            let pattern = regex_lite::Regex::new(&format!(
                r#"\A(?<vendor_name>{})::(?<service_name>{})::(?<resource_name>{})\.(?<property_name>{})\z"#,
                VendorName::BASE_PATTERN,
                ServiceName::BASE_PATTERN,
                ResourceName::BASE_PATTERN,
                PropertyName::BASE_PATTERN
            ))
            .unwrap();

            if let Some(captures) = pattern.captures(value) {
                Ok(PropertyTypeName::PropertyTypeName(
                    ResourcePropertyTypeName {
                        vendor_name: VendorName(captures.name("vendor_name").unwrap().as_str()),
                        service_name: ServiceName(captures.name("service_name").unwrap().as_str()),
                        resource_name: ResourceName(
                            captures.name("resource_name").unwrap().as_str(),
                        ),
                        property_name: PropertyName(
                            captures.name("property_name").unwrap().as_str(),
                        ),
                    },
                ))
            } else {
                Err(format!("Invalid value: {value}"))
            }
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ResourcePropertyTypeName<'a> {
    pub vendor_name: VendorName<'a>,
    pub service_name: ServiceName<'a>,
    pub resource_name: ResourceName<'a>,
    pub property_name: PropertyName<'a>,
}

impl<'a, 'de: 'a> serde::Deserialize<'de> for PropertyTypeName<'a> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <&'a str as serde::de::Deserialize<'de>>::deserialize(deserializer).and_then(|value| {
            std::convert::TryFrom::try_from(value).map_err(serde::de::Error::custom)
        })
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ResourceType<'a> {
    #[serde(borrow = "'a")]
    pub documentation: Documentation<'a>,
    pub attributes: Option<ResourceAttributesMap<'a>>,
    pub additional_properties: Option<bool>,
    pub properties: ResourceTypePropertiesMap<'a>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ResourceAttribute<'a> {
    pub primitive_item_type: Option<PrimitiveItemType>,
    #[serde(borrow = "'a")]
    pub item_type: Option<TypeReference<'a>>,
    pub primitive_type: Option<PrimitiveType>,
    pub r#type: Option<TypeReference<'a>>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct ResourceTypeProperty<'a> {
    #[serde(borrow = "'a")]
    pub documentation: Documentation<'a>,
    pub duplicates_allowed: Option<bool>,
    pub item_type: Option<TypeReference<'a>>,
    pub primitive_type: Option<PrimitiveType>,
    pub primitive_item_type: Option<PrimitiveItemType>,
    pub r#type: Option<TypeReference<'a>>,
    pub required: bool,
    pub update_type: UpdateType,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct PropertyType<'a> {
    #[serde(borrow = "'a")]
    pub documentation: Documentation<'a>,
    pub item_type: Option<TypeReference<'a>>,
    pub properties: Option<PropertyTypePropertiesMap<'a>>,
    pub r#type: Option<TypeReference<'a>>,
    pub primitive_type: Option<PrimitiveType>,
    pub required: Option<bool>,
    pub update_type: Option<UpdateType>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
pub struct PropertyTypeProperty<'a> {
    #[serde(borrow = "'a")]
    pub documentation: Documentation<'a>,
    pub duplicates_allowed: Option<bool>,
    pub item_type: Option<TypeReference<'a>>,
    pub primitive_item_type: Option<PrimitiveItemType>,
    pub primitive_type: Option<PrimitiveType>,
    pub r#type: Option<TypeReference<'a>>,
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

#[derive(Debug)]
pub enum TypeReference<'a> {
    List,
    Map,
    Tag,
    Subproperty(PropertyName<'a>),
}

impl<'a, 'de: 'a> serde::Deserialize<'de> for TypeReference<'a> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <&'a str as serde::de::Deserialize<'de>>::deserialize(deserializer).and_then(|value| {
            if value == "List" {
                Ok(Self::List)
            } else if value == "Map" {
                Ok(Self::Map)
            } else if value == "Tag" {
                Ok(Self::Tag)
            } else {
                match PropertyName::try_from(value) {
                    Ok(value) => Ok(Self::Subproperty(value)),
                    Err(error) => Err(serde::de::Error::custom(format!("Invalid value: {error}"))),
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_resource_specification() {
        eprintln!("{:#?}", &*INSTANCE);
    }
}
