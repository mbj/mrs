#![feature(default_field_values)]

use proc_macro::TokenStream;
use quote::quote;
use stratosphere::resource_specification::*;

struct Service<'a> {
    vendor_name: VendorName<'a>,
    service_name: ServiceName<'a>,
}

impl Service<'_> {
    fn provides(&self, resource_type: &ResourceTypeName) -> bool {
        resource_type.vendor_name == self.vendor_name
            && resource_type.service_name == self.service_name
    }
}

fn ident(name: impl AsRef<str>) -> proc_macro2::Ident {
    syn::Ident::new(name.as_ref(), proc_macro2::Span::call_site())
}

struct ServiceDefinition<'a> {
    resource_type_map: ResourceTypeMap<'a>,
    resource_property_type_map: ResourcePropertyTypeMap<'a>,
}

impl<'a> ServiceDefinition<'a> {
    fn add_resource_property_type(
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

struct TagDefinition<'a>(&'a PropertyType<'a>);

impl quote::ToTokens for TagDefinition<'_> {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        let property_name = PropertyName("Tag");

        stream.extend(property_type_token_stream(&property_name, self.0))
    }
}

type ServiceMap<'a> = std::collections::BTreeMap<&'a ServiceName<'a>, ServiceDefinition<'a>>;

type ResourceTypeMap<'a> =
    std::collections::BTreeMap<&'a ResourceTypeName<'a>, &'a ResourceType<'a>>;

type ResourcePropertyTypeMap<'a> =
    std::collections::BTreeMap<&'a ResourceName<'a>, PropertyTypeMap<'a>>;

type PropertyTypeMap<'a> = std::collections::BTreeMap<&'a PropertyName<'a>, &'a PropertyType<'a>>;

type VendorResourceMap<'a> = std::collections::BTreeMap<&'a VendorName<'a>, ServiceMap<'a>>;

fn vendor_map_token_stream(
    vendor_map: VendorResourceMap,
    tag_definition: TagDefinition,
) -> TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    for (vendor_name, service_map) in vendor_map {
        let vendor_module_name = ident(vendor_name.as_ref().to_lowercase());

        let services = service_map_token_stream(service_map);

        let module_stream: proc_macro2::TokenStream = quote! {
            pub mod #vendor_module_name {
                #services
            }
        };

        stream.extend(module_stream);
    }

    quote! {
        pub mod cloudformation {
            #tag_definition
            #stream
        }
    }
    .into()
}

fn service_map_token_stream(service_map: ServiceMap) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    for (service_name, service_definition) in service_map {
        let service_module_name = ident(service_name.as_ref().to_lowercase());

        let properties =
            resource_property_type_map_token_stream(service_definition.resource_property_type_map);
        let resources = resource_type_map_token_stream(service_definition.resource_type_map);

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
    resource_property_type_map: ResourcePropertyTypeMap,
) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    for (resource_name, property_type_map) in resource_property_type_map {
        stream.extend(property_type_map_token_stream(
            resource_name,
            property_type_map,
        ))
    }

    stream
}

fn property_type_map_token_stream(
    resource_name: &ResourceName<'_>,
    property_type_map: PropertyTypeMap<'_>,
) -> proc_macro2::TokenStream {
    let resource_module_name = ResourceModuleName::new(resource_name);

    let property_types: Vec<_> = property_type_map
        .into_iter()
        .map(|(property_name, property_type)| {
            property_type_token_stream(property_name, property_type)
        })
        .collect();

    quote! {
        pub mod #resource_module_name {
            #(#property_types)*
        }
    }
}

fn property_type_token_stream(
    property_name: &PropertyName<'_>,
    property_type: &PropertyType<'_>,
) -> proc_macro2::TokenStream {
    let struct_name = ident(property_name.as_str());

    let properties = match &property_type.properties {
        Some(properties) => properties,
        None => &PropertyTypePropertiesMap::new(),
    };

    let fields: Vec<_> = properties
        .iter()
        .map(|(property_name, property_type_property)| {
            let field_name = ident(property_name.as_str());

            let field_type = property_type_property_token_stream(property_type_property);

            quote! {
                pub #field_name : #field_type
            }
        })
        .collect();

    let to_values: Vec<_> = properties
        .iter()
        .map(|(property_name, property_type)| {
            let field_name = ident(property_name);
            let key = property_name.as_str();

            if property_type.required {
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
        })
        .collect();

    quote! {
        pub struct #struct_name {
            #(#fields),*
        }

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
        } => quote! { Vec<crate::cloudformation::Tag> },
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
            let name = ident(property_name);
            quote! { Vec<#name> }
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
            let name = ident(property_name);
            quote! { #name }
        }
        other => panic!("Unsupported property type property: {other:#?}"),
    };

    mk_option(property_type_property.required, field_type)
}

fn mk_option(required: bool, stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    if !required {
        quote! { Option<#stream> = None }
    } else {
        stream
    }
}

fn resource_type_map_token_stream(resource_type_map: ResourceTypeMap) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    for (resource_type_name, resource_type) in resource_type_map {
        stream.extend(resource_type_token_stream(
            resource_type_name,
            resource_type,
        ))
    }

    stream
}

fn resource_type_token_stream(
    resource_type_name: &ResourceTypeName<'_>,
    resource_type: &ResourceType<'_>,
) -> proc_macro2::TokenStream {
    let struct_name = ident(resource_type_name.resource_name.as_ref());

    let resource_module_name = ResourceModuleName::new(&resource_type_name.resource_name);

    let documentation = resource_type.documentation.as_str();

    let fields: Vec<_> = resource_type
        .properties
        .iter()
        .map(|(property_name, property_type)| {
            resource_property_type_token_stream(&resource_module_name, property_name, property_type)
        })
        .collect();

    let properties: Vec<_> = resource_type
        .properties
        .iter()
        .map(|(property_name, property_type)| {
            let field_name = ident(property_name);
            let key = property_name.as_str();

            if property_type.required {
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
        })
        .collect();

    let stream = quote! {
        #[doc = #documentation]
        pub struct #struct_name {
            #(#fields),*
        }

        impl stratosphere::template::ToResource for #struct_name {
            const RESOURCE_TYPE_NAME: stratosphere::resource_specification::ResourceTypeName<'static> =
                #resource_type_name;

            fn to_resource_properties(&self) -> stratosphere::template::ResourceProperties {
                let mut properties = stratosphere::template::ResourceProperties::new();

                #(#properties)*

                properties
            }
        }
    };

    stream
}

fn resource_property_type_token_stream(
    resource_module_name: &ResourceModuleName,
    resource_type_property_name: &ResourceTypePropertyName<'_>,
    resource_type_property: &ResourceTypeProperty<'_>,
) -> proc_macro2::TokenStream {
    let field_name = ident(resource_type_property_name);

    let property_type =
        resource_type_property_token_stream(resource_module_name, resource_type_property);

    let field_type = mk_option(resource_type_property.required, property_type);

    quote! {
        pub #field_name: #field_type
    }
}

fn resource_type_property_token_stream(
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
        } => mk_list_type(resource_module_name, item_type),
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
        } => mk_subproperty(resource_module_name, subproperty_name),
        other => panic!("Unsupported property type: {other:#?}"),
    }
}

fn mk_list_type(
    resource_module_name: &ResourceModuleName,
    item_type: &TypeReference,
) -> proc_macro2::TokenStream {
    let item_type = mk_type_reference_type(resource_module_name, item_type);

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

fn mk_type_reference_type(
    resource_module_name: &ResourceModuleName,
    item_type: &TypeReference,
) -> proc_macro2::TokenStream {
    match item_type {
        TypeReference::Subproperty(name) => mk_subproperty(resource_module_name, name),
        TypeReference::Tag => quote! { crate::cloudformation::Tag },
        foo => todo!("{:#?}", foo),
    }
}

struct ResourceModuleName(proc_macro2::Ident);

impl ResourceModuleName {
    fn new(resource_name: &ResourceName<'_>) -> Self {
        Self(ident(resource_name.as_ref().to_lowercase()))
    }
}

impl quote::ToTokens for ResourceModuleName {
    fn to_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        let ident = &self.0;
        stream.extend(quote! { #ident })
    }
}

fn mk_subproperty(
    resource_module_name: &ResourceModuleName,
    property_name: &PropertyName,
) -> proc_macro2::TokenStream {
    let name = ident(property_name.as_str());
    quote! {
        super::ec2::#resource_module_name::#name
    }
}

fn mk_primitive_type(primitive_type: &PrimitiveType) -> proc_macro2::TokenStream {
    match primitive_type {
        PrimitiveType::Boolean => quote! { stratosphere::value::ExpBool },
        PrimitiveType::Double => quote! { f64 },
        // TODO: Verify we do not need a bignum here
        PrimitiveType::Integer => quote! { i64 },
        // TODO: Verify whats that
        PrimitiveType::Long => quote! { i64 },
        PrimitiveType::String => quote! { stratosphere::value::ExpString },
        PrimitiveType::Timestamp => todo!(),
        PrimitiveType::Json => quote! { serde_json::Value },
    }
}

#[proc_macro]
pub fn generate_service(input: TokenStream) -> TokenStream {
    if !input.is_empty() {
        panic!("Macro does not take inputs!")
    }

    let selected_service = Service {
        vendor_name: <VendorName as std::convert::TryFrom<&str>>::try_from("AWS").unwrap(),
        service_name: <ServiceName as std::convert::TryFrom<&str>>::try_from("EC2").unwrap(),
    };

    let specification = stratosphere::resource_specification::instance();

    let mut vendor_map = VendorResourceMap::new();

    for (resource_type_name, resource_type) in &specification.resource_types {
        if selected_service.provides(resource_type_name) {
            let service_name = &resource_type_name.service_name;

            vendor_map
                .entry(&resource_type_name.vendor_name)
                .and_modify(|service_map: &mut ServiceMap| {
                    service_map
                        .entry(service_name)
                        .and_modify(|service_definition| {
                            service_definition
                                .resource_type_map
                                .insert(resource_type_name, resource_type);
                        });
                })
                .or_insert_with(|| {
                    [(
                        service_name,
                        ServiceDefinition {
                            resource_type_map: [(resource_type_name, resource_type)].into(),
                            resource_property_type_map: [].into(),
                        },
                    )]
                    .into()
                });
        }
    }

    let mut tag_definition: Option<TagDefinition> = None;

    for (property_type_name, property_type) in &specification.property_types {
        match property_type_name {
            PropertyTypeName::Tag => {
                tag_definition = Some(TagDefinition(property_type));
            }
            PropertyTypeName::PropertyTypeName(resource_property_type_name) => {
                let service_name = &resource_property_type_name.service_name;

                if selected_service.vendor_name == resource_property_type_name.vendor_name
                    && selected_service.service_name == *service_name
                {
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
    }

    vendor_map_token_stream(
        vendor_map,
        tag_definition.expect("Could not find tag definition"),
    )
}
