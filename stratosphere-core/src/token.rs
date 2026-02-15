use crate::resource_specification::*;
use quote::quote;

pub struct ServiceDefinition<'a> {
    pub resource_type_map: ResourceTypeMap<'a>,
    pub resource_property_type_map: ResourcePropertyTypeMap<'a>,
}

impl<'a> ServiceDefinition<'a> {
    pub fn add_resource_property_type(
        &mut self,
        resource_property_type_name: &'a ResourcePropertyTypeName<'a>,
        property_type: &'a PropertyType<'a>,
    ) {
        self.resource_property_type_map
            .entry(&resource_property_type_name.resource_name)
            .and_modify(|property_type_map| {
                property_type_map.insert(&resource_property_type_name.property_name, property_type);
            })
            .or_insert([(&resource_property_type_name.property_name, property_type)].into());
    }
}

pub struct TagDefinition<'a>(pub &'a PropertyType<'a>);

impl quote::ToTokens for TagDefinition<'_> {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        let property_name = PropertyName("Tag");

        stream.extend(property_type_token_stream(
            &PropertyTypeName::Tag,
            &property_name,
            self.0,
        ))
    }
}

pub type ServiceMap<'a> = std::collections::BTreeMap<&'a ServiceName<'a>, ServiceDefinition<'a>>;

type ResourceTypeMap<'a> =
    std::collections::BTreeMap<&'a ResourceTypeName<'a>, &'a ResourceType<'a>>;

type ResourcePropertyTypeMap<'a> =
    std::collections::BTreeMap<&'a ResourceName<'a>, PropertyTypeMap<'a>>;

type PropertyTypeMap<'a> = std::collections::BTreeMap<&'a PropertyName<'a>, &'a PropertyType<'a>>;

pub type VendorMap<'a> = std::collections::BTreeMap<&'a VendorName<'a>, ServiceMap<'a>>;

/// Builds a VendorMap containing all services from the specification
#[must_use]
pub fn build_vendor_map<'a>(specification: &'a ResourceSpecification<'a>) -> VendorMap<'a> {
    fn mk_service_definition<'a>(
        resource_type_name: &'a ResourceTypeName,
        resource_type: &'a ResourceType,
    ) -> ServiceDefinition<'a> {
        ServiceDefinition {
            resource_type_map: [(resource_type_name, resource_type)].into(),
            resource_property_type_map: [].into(),
        }
    }

    let mut vendor_map = VendorMap::new();

    for (resource_type_name, resource_type) in &specification.resource_types {
        let target_service = &resource_type_name.service;
        let vendor_name = &target_service.vendor_name;
        let service_name = &target_service.service_name;

        vendor_map
            .entry(vendor_name)
            .and_modify(|service_map: &mut ServiceMap| {
                service_map
                    .entry(service_name)
                    .and_modify(|service_definition| {
                        service_definition
                            .resource_type_map
                            .insert(resource_type_name, resource_type);
                    })
                    .or_insert_with(|| mk_service_definition(resource_type_name, resource_type));
            })
            .or_insert_with(|| {
                [(
                    service_name,
                    mk_service_definition(resource_type_name, resource_type),
                )]
                .into()
            });
    }

    for (property_type_name, property_type) in &specification.property_types {
        if let PropertyTypeName::PropertyTypeName(resource_property_type_name) = property_type_name
        {
            let service_name = &resource_property_type_name.service_name;

            vendor_map
                .entry(&resource_property_type_name.vendor_name)
                .and_modify(|service_map: &mut ServiceMap| {
                    service_map
                        .entry(service_name)
                        .and_modify(|service_definition| {
                            service_definition.add_resource_property_type(
                                resource_property_type_name,
                                property_type,
                            )
                        });
                });
        }
    }

    vendor_map
}

/// Generates the token stream for a single service file (for pre-generated services)
#[must_use]
pub fn service_file_token_stream(
    service: &ServiceIdentifier<'_>,
    service_definition: &ServiceDefinition<'_>,
) -> proc_macro2::TokenStream {
    let service_module_name = ServiceModuleName::new(&service.service_name);

    let properties = resource_property_type_map_token_stream(
        service,
        &service_definition.resource_property_type_map,
    );
    let resources =
        resource_type_map_token_stream(&service_module_name, &service_definition.resource_type_map);

    quote! {
        #properties
        #resources
    }
}

fn resource_property_type_map_token_stream(
    service: &ServiceIdentifier<'_>,
    resource_property_type_map: &ResourcePropertyTypeMap<'_>,
) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    for (resource_name, property_type_map) in resource_property_type_map {
        stream.extend(property_type_map_token_stream(
            service,
            resource_name,
            property_type_map,
        ))
    }

    stream
}

fn property_type_map_token_stream(
    service: &ServiceIdentifier<'_>,
    resource_name: &ResourceName<'_>,
    property_type_map: &PropertyTypeMap<'_>,
) -> proc_macro2::TokenStream {
    let resource_module_name = ResourceModuleName::new(resource_name);

    let property_types: Vec<_> = property_type_map
        .iter()
        .map(|(property_name, property_type)| {
            property_type_token_stream(
                &PropertyTypeName::PropertyTypeName(ResourcePropertyTypeName {
                    vendor_name: service.vendor_name.clone(),
                    service_name: service.service_name.clone(),
                    resource_name: resource_name.clone(),
                    property_name: (*property_name).clone(),
                }),
                property_name,
                property_type,
            )
        })
        .collect();

    quote! {
        pub mod #resource_module_name {
            #(#property_types)*
        }
    }
}

fn property_type_token_stream(
    property_type_name: &PropertyTypeName<'_>,
    property_name: &PropertyName<'_>,
    property_type: &PropertyType<'_>,
) -> proc_macro2::TokenStream {
    let struct_name = property_name_struct_name(property_name);
    let macro_name = quote::format_ident!("{property_name}");
    let prefixed_macro_name = match property_type_name {
        PropertyTypeName::Tag => quote::format_ident!("__Tag"),
        PropertyTypeName::PropertyTypeName(name) => quote::format_ident!(
            "__{}_{}_{}_{}",
            name.vendor_name.as_ref().to_lowercase(),
            name.service_name.as_ref().to_lowercase(),
            name.resource_name.as_ref(),
            name.property_name.as_ref()
        ),
    };

    let properties = match &property_type.properties {
        Some(properties) => properties,
        None => &PropertyTypePropertiesMap::new(),
    };

    let documentation = property_type.documentation.as_str();

    let fields: Vec<_> = properties
        .iter()
        .map(|(property_name, property_type_property)| {
            let field_name = mk_field_name(property_name.as_str());

            let field_type = property_type_property_token_stream(property_type_property);

            quote! {
                pub #field_name : #field_type
            }
        })
        .collect();

    let to_values: Vec<_> = properties
        .iter()
        .map(|(property_name, property_type)| mk_to_value(property_name, property_type))
        .collect();

    let property_type_name_str: &str = &property_type_name.to_string();

    let to_value_body = if to_values.is_empty() {
        quote! {
            serde_json::Value::Object(serde_json::Map::new())
        }
    } else {
        quote! {
            let mut properties = serde_json::Map::new();
            #(#to_values)*
            properties.into()
        }
    };

    quote! {
        #[doc = #documentation]
        pub struct #struct_name {
            #(#fields),*
        }

        #[doc(hidden)]
        #[macro_export]
        macro_rules! #prefixed_macro_name {
            ($($field:ident : $value:expr),* $(,)?) => {
               stratosphere::generator::construct_property_type!(#property_type_name_str $($field $value)*)
            }
        }

        pub use crate::#prefixed_macro_name as #macro_name;

        impl crate::value::ToValue for #struct_name {
            fn to_value(&self) -> serde_json::Value {
                #to_value_body
            }
        }
    }
}

fn property_type_property_token_stream(
    property_type_property: &PropertyTypeProperty,
) -> proc_macro2::TokenStream {
    let field_type = match property_type_property {
        PropertyTypeProperty {
            documentation: _,
            duplicates_allowed: None,
            item_type: None,
            primitive_item_type: None,
            primitive_type: Some(primitive_type),
            r#type: None,
            required: _,
            update_type: _,
        } => mk_primitive_type(primitive_type),
        PropertyTypeProperty {
            documentation: _,
            duplicates_allowed: _,
            item_type: Some(TypeReference::Tag),
            primitive_item_type: None,
            primitive_type: None,
            r#type: Some(TypeReference::List),
            required: _,
            update_type: _,
        } => quote! { Vec<crate::Tag_> },
        PropertyTypeProperty {
            documentation: _,
            duplicates_allowed: _,
            item_type: Some(TypeReference::Subproperty(property_name)),
            primitive_item_type: None,
            primitive_type: None,
            r#type: Some(TypeReference::List),
            required: _,
            update_type: _,
        } => {
            let struct_name = property_name_struct_name(property_name);
            quote! { Vec<#struct_name> }
        }
        PropertyTypeProperty {
            documentation: _,
            duplicates_allowed: _,
            item_type: None,
            primitive_item_type: Some(primitive_item_type),
            primitive_type: None,
            r#type: Some(TypeReference::List),
            required: _,
            update_type: _,
        } => mk_primitive_list_type(primitive_item_type),
        PropertyTypeProperty {
            documentation: _,
            duplicates_allowed: None,
            item_type: None,
            primitive_item_type: None,
            primitive_type: None,
            r#type: Some(TypeReference::Subproperty(property_name)),
            required: _,
            update_type: _,
        } => {
            let struct_name = property_name_struct_name(property_name);
            // Box to break potential recursive type cycles between property types
            quote! { Box<#struct_name> }
        }
        PropertyTypeProperty {
            documentation: _,
            duplicates_allowed: _,
            item_type: None,
            primitive_item_type: Some(item_type),
            primitive_type: None,
            r#type: Some(TypeReference::Map),
            required: _,
            update_type: _,
        } => mk_primitive_map_type(item_type),
        PropertyTypeProperty {
            documentation: _,
            duplicates_allowed: _,
            item_type: Some(TypeReference::Subproperty(property_name)),
            primitive_item_type: None,
            primitive_type: None,
            r#type: Some(TypeReference::Map),
            required: _,
            update_type: _,
        } => {
            let struct_name = property_name_struct_name(property_name);
            quote! { std::collections::BTreeMap<String, #struct_name> }
        }
        other => panic!("Unsupported property type property: {other:#?}"),
    };

    mk_option(property_type_property.required, field_type)
}

fn mk_option(required: bool, stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    if !required {
        quote! { Option<#stream> }
    } else {
        stream
    }
}

fn resource_type_map_token_stream(
    service_module_name: &ServiceModuleName,
    resource_type_map: &ResourceTypeMap<'_>,
) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    for (resource_type_name, resource_type) in resource_type_map {
        stream.extend(resource_type_token_stream(
            service_module_name,
            resource_type_name,
            resource_type,
        ))
    }

    stream
}

trait IsRequired {
    fn required(&self) -> bool;
}

impl IsRequired for &ResourceTypeProperty<'_> {
    fn required(&self) -> bool {
        self.required
    }
}

impl IsRequired for &PropertyTypeProperty<'_> {
    fn required(&self) -> bool {
        self.required
    }
}

fn mk_to_value(
    property_name: impl AsRef<str>,
    property_type: impl IsRequired,
) -> proc_macro2::TokenStream {
    let field_name = mk_field_name(property_name.as_ref());
    let key = property_name.as_ref();

    if property_type.required() {
        quote! {
            properties
                .insert(
                    #key.to_string(),
                    crate::value::ToValue::to_value(&self.#field_name)
                );
        }
    } else {
        quote! {
          if let Some(ref value) = self.#field_name {
              properties.insert(
                  #key.to_string(),
                  crate::value::ToValue::to_value(value)
              );
          }
        }
    }
}

fn resource_type_token_stream(
    service_module_name: &ServiceModuleName,
    resource_type_name: &ResourceTypeName<'_>,
    resource_type: &ResourceType<'_>,
) -> proc_macro2::TokenStream {
    let resource_name_ref = resource_type_name.resource_name.as_ref();

    let struct_name = quote::format_ident!("{resource_name_ref}_");

    let macro_name = quote::format_ident!("{resource_name_ref}");

    let prefixed_macro_name = quote::format_ident!(
        "__{}_{}_{resource_name_ref}",
        resource_type_name
            .service
            .vendor_name
            .as_ref()
            .to_lowercase(),
        resource_type_name
            .service
            .service_name
            .as_ref()
            .to_lowercase(),
    );

    let resource_module_name = ResourceModuleName::new(&resource_type_name.resource_name);

    let documentation = resource_type.documentation.as_str();

    let fields: Vec<_> = resource_type
        .properties
        .iter()
        .map(|(property_name, property_type)| {
            resource_property_type_token_stream(
                service_module_name,
                &resource_module_name,
                property_name,
                property_type,
            )
        })
        .collect();

    let to_values: Vec<_> = resource_type
        .properties
        .iter()
        .map(|(property_name, property_type)| mk_to_value(property_name, property_type))
        .collect();

    let resource_type_name_str: &str = &resource_type_name.to_string();

    let to_resource_properties_body = if to_values.is_empty() {
        quote! {
            crate::template::ResourceProperties::new()
        }
    } else {
        quote! {
            let mut properties = crate::template::ResourceProperties::new();
            #(#to_values)*
            properties
        }
    };

    let stream = quote! {
        #[doc = #documentation]
        pub struct #struct_name {
            #(#fields),*
        }

        #[doc(hidden)]
        #[macro_export]
        macro_rules! #prefixed_macro_name {
            ($($field:ident : $value:expr),* $(,)?) => {
               stratosphere::generator::construct_resource_type!(#resource_type_name_str $($field $value)*)
            }
        }

        pub use crate::#prefixed_macro_name as #macro_name;

        impl crate::template::ToResource for #struct_name {
            const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
                #resource_type_name;

            fn to_resource_properties(&self) -> crate::template::ResourceProperties {
                #to_resource_properties_body
            }
        }
    };

    stream
}

fn resource_property_type_token_stream(
    service_module_name: &ServiceModuleName,
    resource_module_name: &ResourceModuleName,
    resource_type_property_name: &ResourceTypePropertyName<'_>,
    resource_type_property: &ResourceTypeProperty<'_>,
) -> proc_macro2::TokenStream {
    let field_name = mk_field_name(resource_type_property_name);

    let property_type = resource_type_property_token_stream(
        service_module_name,
        resource_module_name,
        resource_type_property,
    );

    let field_type = mk_option(resource_type_property.required, property_type);

    quote! {
        pub #field_name: #field_type
    }
}

fn resource_type_property_token_stream(
    service_module_name: &ServiceModuleName,
    resource_module_name: &ResourceModuleName,
    resource_type_property: &ResourceTypeProperty<'_>,
) -> proc_macro2::TokenStream {
    match resource_type_property {
        ResourceTypeProperty {
            primitive_type: Some(primitive_type),
            item_type: None,
            duplicates_allowed: None,
            primitive_item_type: None,
            documentation: _,
            update_type: _,
            required: _,
            r#type: None,
        } => mk_primitive_type(primitive_type),
        ResourceTypeProperty {
            primitive_type: None,
            item_type: Some(item_type),
            duplicates_allowed: _,
            primitive_item_type,
            documentation: _,
            update_type: _,
            required: _,
            r#type: Some(TypeReference::List),
        } => mk_list_type(
            service_module_name,
            resource_module_name,
            item_type,
            primitive_item_type.as_ref(),
        ),
        ResourceTypeProperty {
            primitive_type: None,
            item_type: None,
            duplicates_allowed: _,
            primitive_item_type: Some(primitive_item_type),
            documentation: _,
            update_type: _,
            required: _,
            r#type: Some(TypeReference::List),
        } => mk_primitive_list_type(primitive_item_type),
        ResourceTypeProperty {
            primitive_type: None,
            item_type: None,
            duplicates_allowed: None,
            primitive_item_type: None,
            documentation: _,
            update_type: _,
            required: _,
            r#type: Some(TypeReference::Subproperty(subproperty_name)),
        } => mk_subproperty(service_module_name, resource_module_name, subproperty_name),
        ResourceTypeProperty {
            primitive_type: None,
            item_type: Some(item_type),
            duplicates_allowed: _,
            primitive_item_type,
            documentation: _,
            update_type: _,
            required: _,
            r#type: Some(TypeReference::Map),
        } => mk_map_type(
            service_module_name,
            resource_module_name,
            item_type,
            primitive_item_type.as_ref(),
        ),
        ResourceTypeProperty {
            primitive_type: None,
            item_type: None,
            duplicates_allowed: _,
            primitive_item_type: Some(primitive_item_type),
            documentation: _,
            update_type: _,
            required: _,
            r#type: Some(TypeReference::Map),
        } => mk_primitive_map_type(primitive_item_type),
        other => panic!("Unsupported property type: {other:#?}"),
    }
}

fn mk_list_type(
    service_module_name: &ServiceModuleName,
    resource_module_name: &ResourceModuleName,
    item_type: &TypeReference,
    primitive_item_type: Option<&PrimitiveItemType>,
) -> proc_macro2::TokenStream {
    let item_type = mk_type_reference_type(
        service_module_name,
        resource_module_name,
        item_type,
        primitive_item_type,
    );

    quote! { Vec<#item_type> }
}

fn mk_primitive_list_type(primitive_item_type: &PrimitiveItemType) -> proc_macro2::TokenStream {
    let item_type = match primitive_item_type {
        PrimitiveItemType::Json => quote! { serde_json::Value },
        PrimitiveItemType::Double => quote! { f64 },
        PrimitiveItemType::Integer => quote! { i32 },
        PrimitiveItemType::String => quote! { crate::value::ExpString },
    };

    quote! { Vec<#item_type> }
}

fn mk_map_type(
    service_module_name: &ServiceModuleName,
    resource_module_name: &ResourceModuleName,
    item_type: &TypeReference,
    primitive_item_type: Option<&PrimitiveItemType>,
) -> proc_macro2::TokenStream {
    let value_type = mk_type_reference_type(
        service_module_name,
        resource_module_name,
        item_type,
        primitive_item_type,
    );

    quote! { std::collections::BTreeMap<String, #value_type> }
}

fn mk_primitive_map_type(primitive_item_type: &PrimitiveItemType) -> proc_macro2::TokenStream {
    let item_type = match primitive_item_type {
        PrimitiveItemType::Json => quote! { serde_json::Value },
        PrimitiveItemType::Double => quote! { f64 },
        PrimitiveItemType::Integer => quote! { i32 },
        PrimitiveItemType::String => quote! { crate::value::ExpString },
    };

    quote! { std::collections::BTreeMap<String, #item_type> }
}

fn mk_type_reference_type(
    service_module_name: &ServiceModuleName,
    resource_module_name: &ResourceModuleName,
    item_type: &TypeReference,
    primitive_item_type: Option<&PrimitiveItemType>,
) -> proc_macro2::TokenStream {
    match item_type {
        TypeReference::Subproperty(name) => {
            mk_subproperty(service_module_name, resource_module_name, name)
        }
        TypeReference::Tag => quote! { crate::Tag_ },
        TypeReference::List => match primitive_item_type {
            Some(PrimitiveItemType::Json) => quote! { Vec<serde_json::Value> },
            Some(PrimitiveItemType::Double) => quote! { Vec<f64> },
            Some(PrimitiveItemType::Integer) => quote! { Vec<i32> },
            Some(PrimitiveItemType::String) => quote! { Vec<crate::value::ExpString> },
            None => panic!("TypeReference::List requires primitive_item_type to be specified"),
        },
        TypeReference::Map => match primitive_item_type {
            Some(PrimitiveItemType::Json) => {
                quote! { std::collections::BTreeMap<String, serde_json::Value> }
            }
            Some(PrimitiveItemType::Double) => {
                quote! { std::collections::BTreeMap<String, f64> }
            }
            Some(PrimitiveItemType::Integer) => {
                quote! { std::collections::BTreeMap<String, i32> }
            }
            Some(PrimitiveItemType::String) => {
                quote! { std::collections::BTreeMap<String, crate::value::ExpString> }
            }
            None => panic!("TypeReference::Map requires primitive_item_type to be specified"),
        },
    }
}

pub struct VendorModuleName(proc_macro2::Ident);

impl VendorModuleName {
    #[must_use]
    pub fn new(vendor_name: &VendorName<'_>) -> Self {
        Self(quote::format_ident!(
            "{}",
            vendor_name.as_ref().to_lowercase()
        ))
    }
}

impl quote::ToTokens for VendorModuleName {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        let ident = &self.0;
        stream.extend(quote! { #ident })
    }
}

pub struct ServiceModuleName(proc_macro2::Ident);

impl ServiceModuleName {
    #[must_use]
    pub fn new(service_name: &ServiceName<'_>) -> Self {
        Self(quote::format_ident!(
            "{}",
            service_name.as_ref().to_lowercase()
        ))
    }
}

impl quote::ToTokens for ServiceModuleName {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        let ident = &self.0;
        stream.extend(quote! { #ident })
    }
}

pub struct ResourceModuleName(proc_macro2::Ident);

impl ResourceModuleName {
    #[must_use]
    pub fn new(resource_name: &ResourceName<'_>) -> Self {
        Self(resource_name.to_module_ident())
    }
}

impl quote::ToTokens for ResourceModuleName {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        let ident = &self.0;
        stream.extend(quote! { #ident })
    }
}

fn mk_subproperty(
    service_module_name: &ServiceModuleName,
    resource_module_name: &ResourceModuleName,
    property_name: &PropertyName,
) -> proc_macro2::TokenStream {
    let property_name = property_name_struct_name(property_name);

    quote! {
        super::#service_module_name::#resource_module_name::#property_name
    }
}

fn mk_primitive_type(primitive_type: &PrimitiveType) -> proc_macro2::TokenStream {
    match primitive_type {
        PrimitiveType::Boolean => quote! { crate::value::ExpBool },
        PrimitiveType::Double => quote! { f64 },
        PrimitiveType::Integer => quote! { i32 },
        PrimitiveType::Long => quote! { i64 },
        PrimitiveType::String => quote! { crate::value::ExpString },
        PrimitiveType::Timestamp => quote! { chrono::DateTime<chrono::Utc> },
        PrimitiveType::Json => quote! { serde_json::Value },
    }
}

/// Converts a string from CamelCase or PascalCase to snake_case
#[must_use]
pub fn to_snake_case(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let mut it = input.chars().peekable();

    let mut prev_is_lower = false;

    let first = it.next().unwrap();
    result.push(if first.is_uppercase() {
        first.to_lowercase().next().unwrap()
    } else {
        prev_is_lower = true;
        first
    });

    while let Some(ch) = it.next() {
        let is_upper = ch.is_uppercase();
        let next_is_lower = it.peek().is_some_and(|&next_ch| next_ch.is_lowercase());

        if (next_is_lower || prev_is_lower) && is_upper {
            result.push('_');
        }

        result.push(if is_upper {
            ch.to_lowercase().next().unwrap()
        } else {
            ch
        });

        prev_is_lower = !is_upper;
    }

    result
}

/// Creates a safe Rust identifier, escaping with `r#` if the name is a keyword
#[must_use]
pub fn mk_safe_ident(name: impl AsRef<str>) -> syn::Ident {
    let name = name.as_ref();
    if syn::parse_str::<syn::Ident>(name).is_err() {
        quote::format_ident!("r#{}", name)
    } else {
        quote::format_ident!("{}", name)
    }
}

pub fn mk_field_name(name: impl AsRef<str>) -> syn::Ident {
    let name = name.as_ref();
    let snake_case_name = to_snake_case(name);

    if snake_case_name == "self" {
        return quote::format_ident!("self_value");
    }

    mk_safe_ident(snake_case_name)
}

#[must_use]
pub fn resource_type_struct_name(resource_type_name: &ResourceTypeName<'_>) -> syn::Ident {
    quote::format_ident!("{}_", resource_type_name.resource_name.as_str())
}

#[must_use]
pub fn property_name_struct_name(property_name: &PropertyName<'_>) -> syn::Ident {
    quote::format_ident!("{property_name}_")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_snake_case(original: &str, expected: &str) {
        assert_eq!(
            to_snake_case(original),
            expected,
            "Original: {original}, expected: {expected}"
        );
    }

    #[test]
    fn test_pascal_case() {
        assert_snake_case("HelloWorld", "hello_world");
    }

    #[test]
    fn test_camel_case() {
        assert_snake_case("helloWorld", "hello_world");
    }

    #[test]
    fn test_all_upper() {
        assert_snake_case("XMLHTTPRequest", "xmlhttp_request");
    }

    #[test]
    fn test_single_word() {
        assert_snake_case("Hello", "hello");
    }

    #[test]
    fn test_lowercase() {
        assert_snake_case("hello", "hello");
    }

    #[test]
    fn test_keyword_detection_and_escaping() {
        assert_eq!(mk_field_name("Type").to_string(), "r#type");
        assert_eq!(mk_field_name("Match").to_string(), "r#match");
        assert_eq!(mk_field_name("Async").to_string(), "r#async");
        assert_eq!(mk_field_name("Await").to_string(), "r#await");
        assert_eq!(mk_field_name("Const").to_string(), "r#const");
        assert_eq!(mk_field_name("Loop").to_string(), "r#loop");
        assert_eq!(mk_field_name("Return").to_string(), "r#return");
        assert_eq!(mk_field_name("Impl").to_string(), "r#impl");
        assert_eq!(mk_field_name("Mod").to_string(), "r#mod");
        assert_eq!(mk_field_name("Pub").to_string(), "r#pub");
        assert_eq!(mk_field_name("Use").to_string(), "r#use");
        assert_eq!(mk_field_name("Fn").to_string(), "r#fn");
        assert_eq!(mk_field_name("Static").to_string(), "r#static");
        assert_eq!(mk_field_name("Mut").to_string(), "r#mut");
        assert_eq!(mk_field_name("Ref").to_string(), "r#ref");
        assert_eq!(mk_field_name("Dyn").to_string(), "r#dyn");
        assert_eq!(mk_field_name("Self").to_string(), "self_value");
        assert_eq!(mk_field_name("Abstract").to_string(), "r#abstract");
        assert_eq!(mk_field_name("Final").to_string(), "r#final");
        assert_eq!(mk_field_name("Override").to_string(), "r#override");
        assert_eq!(mk_field_name("Yield").to_string(), "r#yield");
        assert_eq!(mk_field_name("Try").to_string(), "r#try");
        assert_eq!(mk_field_name("Gen").to_string(), "gen");
        assert_eq!(mk_field_name("Union").to_string(), "union");
        assert_eq!(mk_field_name("FooBar").to_string(), "foo_bar");
        assert_eq!(mk_field_name("TestValue").to_string(), "test_value");
        assert_eq!(mk_field_name("MyField").to_string(), "my_field");
        assert_eq!(
            mk_field_name("EnableDnsSupport").to_string(),
            "enable_dns_support"
        );
        assert_eq!(mk_field_name("VpcId").to_string(), "vpc_id");
    }
}
