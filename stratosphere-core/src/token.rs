use crate::resource_specification::*;
use quote::quote;

pub struct Target<'a> {
    tag_definition: TagDefinition<'a>,
    vendor_map: VendorMap<'a>,
}

impl<'a> Target<'a> {
    pub fn for_services(
        specification: &'a ResourceSpecification<'a>,
        selected_services: std::collections::BTreeSet<ServiceIdentifier<'a>>,
    ) -> Self {
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

            if !selected_services.contains(target_service) {
                continue;
            }

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
                        .or_insert_with(|| {
                            mk_service_definition(resource_type_name, resource_type)
                        });
                })
                .or_insert_with(|| {
                    [(
                        service_name,
                        mk_service_definition(resource_type_name, resource_type),
                    )]
                    .into()
                });
        }

        let mut tag_definition: Option<TagDefinition> = None;

        for (property_type_name, property_type) in &specification.property_types {
            match property_type_name {
                PropertyTypeName::Tag => {
                    tag_definition = Some(TagDefinition(property_type));
                }
                PropertyTypeName::PropertyTypeName(resource_property_type_name) => {
                    let vendor_name = &resource_property_type_name.vendor_name;
                    let service_name = &resource_property_type_name.service_name;

                    let service_identifier = ServiceIdentifier {
                        vendor_name: vendor_name.clone(),
                        service_name: service_name.clone(),
                    };

                    if !selected_services.contains(&service_identifier) {
                        continue;
                    }

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
                        })
                        .or_insert_with(|| panic!("Property without resource"));
                }
            }
        }

        Self {
            tag_definition: tag_definition.expect("Tag property missing"),
            vendor_map,
        }
    }
}

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

pub fn token_stream(target: Target) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    for (vendor_name, service_map) in target.vendor_map {
        let vendor_module_name = VendorModuleName::new(vendor_name);

        let services = service_map_token_stream(vendor_name, service_map);

        let module_stream: proc_macro2::TokenStream = quote! {
            pub mod #vendor_module_name {
                #services
            }
        };

        stream.extend(module_stream);
    }

    let tag_definition = target.tag_definition;

    quote! {
        pub mod cloudformation {
            #tag_definition
            #stream
        }
    }
}

fn service_map_token_stream(
    vendor_name: &VendorName<'_>,
    service_map: ServiceMap,
) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    for (service_name, service_definition) in service_map {
        let service_module_name = ServiceModuleName::new(service_name);

        let properties = resource_property_type_map_token_stream(
            &ServiceIdentifier {
                vendor_name: vendor_name.clone(),
                service_name: service_name.clone(),
            },
            service_definition.resource_property_type_map,
        );
        let resources = resource_type_map_token_stream(
            &service_module_name,
            service_definition.resource_type_map,
        );

        let service_stream: proc_macro2::TokenStream = quote! {
            pub mod #service_module_name {
                #properties
                #resources
            }
        };

        stream.extend(service_stream);
    }
    stream
}

fn resource_property_type_map_token_stream(
    service: &ServiceIdentifier<'_>,
    resource_property_type_map: ResourcePropertyTypeMap,
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
    property_type_map: PropertyTypeMap<'_>,
) -> proc_macro2::TokenStream {
    let resource_module_name = ResourceModuleName::new(resource_name);

    let property_types: Vec<_> = property_type_map
        .into_iter()
        .map(|(property_name, property_type)| {
            property_type_token_stream(
                &PropertyTypeName::PropertyTypeName(ResourcePropertyTypeName {
                    vendor_name: service.vendor_name.clone(),
                    service_name: service.service_name.clone(),
                    resource_name: resource_name.clone(),
                    property_name: property_name.clone(),
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

    quote! {
        #[doc = #documentation]
        pub struct #struct_name {
            #(#fields),*
        }

        #[allow(unused_macros)]
        macro_rules! #macro_name {
            ($($field:ident : $value:expr),* $(,)?) => {
               stratosphere::generator::construct_property_type!(#property_type_name_str $($field $value)*)
            }
        }

        pub(crate) use #macro_name;

        impl stratosphere::value::ToValue for #struct_name {
            fn to_value(&self) -> serde_json::Value {
                let mut properties = serde_json::Map::new();

                #(#to_values)*

                properties.into()
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
        } => quote! { Vec<super::super::super::Tag_> },
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
            quote! { #struct_name }
        }
        PropertyTypeProperty {
            documentation: _,
            duplicates_allowed: None,
            item_type: None,
            primitive_item_type: Some(item_type),
            primitive_type: None,
            r#type: Some(TypeReference::Map),
            required: _,
            update_type: _,
        } => mk_primitive_map_type(item_type),
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
    resource_type_map: ResourceTypeMap,
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
                    stratosphere::value::ToValue::to_value(&self.#field_name)
                );
        }
    } else {
        quote! {
          if let Some(ref value) = self.#field_name {
              properties.insert(
                  #key.to_string(),
                  stratosphere::value::ToValue::to_value(value)
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

    let stream = quote! {
        #[doc = #documentation]
        pub struct #struct_name {
            #(#fields),*
        }

        #[allow(unused_macros)]
        macro_rules! #macro_name {
            ($($field:ident : $value:expr),* $(,)?) => {
               stratosphere::generator::construct_resource_type!(#resource_type_name_str $($field $value)*)
            }
        }

        pub(crate) use #macro_name;

        impl stratosphere::template::ToResource for #struct_name {
            const RESOURCE_TYPE_NAME: stratosphere::resource_specification::ResourceTypeName<'static> =
                #resource_type_name;

            fn to_resource_properties(&self) -> stratosphere::template::ResourceProperties {
                let mut properties = stratosphere::template::ResourceProperties::new();

                #(#to_values)*

                properties
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
            primitive_item_type: None,
            documentation: _,
            update_type: _,
            required: _,
            r#type: Some(TypeReference::List),
        } => mk_list_type(service_module_name, resource_module_name, item_type),
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
        other => panic!("Unsupported property type: {other:#?}"),
    }
}

fn mk_list_type(
    service_module_name: &ServiceModuleName,
    resource_module_name: &ResourceModuleName,
    item_type: &TypeReference,
) -> proc_macro2::TokenStream {
    let item_type = mk_type_reference_type(service_module_name, resource_module_name, item_type);

    quote! { Vec<#item_type> }
}

fn mk_primitive_list_type(primitive_item_type: &PrimitiveItemType) -> proc_macro2::TokenStream {
    let item_type = match primitive_item_type {
        PrimitiveItemType::Json => quote! { serde_json::Value },
        PrimitiveItemType::Double => quote! { f64 },
        PrimitiveItemType::Integer => quote! { i64 },
        PrimitiveItemType::String => quote! { stratosphere::value::ExpString },
    };

    quote! { Vec<#item_type> }
}

fn mk_primitive_map_type(primitive_item_type: &PrimitiveItemType) -> proc_macro2::TokenStream {
    let item_type = match primitive_item_type {
        PrimitiveItemType::Json => quote! { serde_json::Value },
        PrimitiveItemType::Double => quote! { f64 },
        PrimitiveItemType::Integer => quote! { i64 },
        PrimitiveItemType::String => quote! { stratosphere::value::ExpString },
    };

    quote! { std::collections::BTreeMap<String, #item_type> }
}

fn mk_type_reference_type(
    service_module_name: &ServiceModuleName,
    resource_module_name: &ResourceModuleName,
    item_type: &TypeReference,
) -> proc_macro2::TokenStream {
    match item_type {
        TypeReference::Subproperty(name) => {
            mk_subproperty(service_module_name, resource_module_name, name)
        }
        TypeReference::Tag => quote! { super::super::Tag_ },
        // RELEASE-TODO: Fix all these
        unsupported => panic!("Currently unsupported: {unsupported:#?}"),
    }
}

pub struct VendorModuleName(proc_macro2::Ident);

impl VendorModuleName {
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
    pub fn new(resource_name: &ResourceName<'_>) -> Self {
        Self(quote::format_ident!(
            "{}",
            resource_name.as_ref().to_lowercase()
        ))
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
        PrimitiveType::Boolean => quote! { stratosphere::value::ExpBool },
        PrimitiveType::Double => quote! { f64 },
        PrimitiveType::Integer => quote! { i64 },
        PrimitiveType::Long => quote! { i64 },
        PrimitiveType::String => quote! { stratosphere::value::ExpString },
        PrimitiveType::Timestamp => todo!(),
        PrimitiveType::Json => quote! { serde_json::Value },
    }
}

/// Converts a string from CamelCase or PascalCase to snake_case
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

// Release todo: Cover all keywords
pub fn mk_field_name(name: impl AsRef<str>) -> syn::Ident {
    let name = name.as_ref();

    if name == "Type" {
        quote::format_ident!("r#type")
    } else if name == "Match" {
        quote::format_ident!("r#match")
    } else {
        quote::format_ident!("{}", to_snake_case(name))
    }
}

pub fn resource_type_struct_name(resource_type_name: &ResourceTypeName<'_>) -> syn::Ident {
    quote::format_ident!("{}_", resource_type_name.resource_name.as_str())
}

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
}
