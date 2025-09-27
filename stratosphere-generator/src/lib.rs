use std::collections::{BTreeMap, BTreeSet};
use stratosphere_core::resource_specification::*;
use stratosphere_core::token::*;

struct ServiceLiterals(syn::punctuated::Punctuated<syn::LitStr, syn::token::Comma>);

impl syn::parse::Parse for ServiceLiterals {
    fn parse(buffer: &syn::parse::ParseBuffer<'_>) -> Result<Self, syn::Error> {
        Ok(Self(buffer.parse_terminated(
            <syn::LitStr as syn::parse::Parse>::parse,
            syn::Token![,],
        )?))
    }
}

#[proc_macro]
pub fn services(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut service_identifiers = BTreeSet::new();

    let input = syn::parse_macro_input!(input as ServiceLiterals);

    let pairs: Vec<(String, syn::LitStr)> = input
        .0
        .into_iter()
        .map(|literal| (literal.value(), literal))
        .collect::<Vec<_>>();

    for (string, literal) in pairs.iter() {
        match ServiceIdentifier::try_from(string.as_str()) {
            Ok(service) => {
                if !service_identifiers.insert(service) {
                    return proc_macro::TokenStream::from(
                        syn::Error::new(literal.span(), "Duplicate entry").to_compile_error(),
                    );
                }
            }
            Err(error) => {
                return proc_macro::TokenStream::from(
                    syn::Error::new(
                        literal.span(),
                        format!("Cannot parse as service identifier: {error}"),
                    )
                    .to_compile_error(),
                );
            }
        }
    }

    token_stream(Target::for_services(instance(), service_identifiers)).into()
}

#[proc_macro]
pub fn construct_resource_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut iterator = input.into_iter();

    let first = iterator.next().expect("Missing resource type!");

    let name = syn::parse::<syn::LitStr>(first.clone().into())
        .unwrap_or_else(|error| {
            panic!("Could not parse first argument as string: {error}, but got: #{first}")
        })
        .value();

    let resource_type_name = ResourceTypeName::try_from(name.as_ref()).unwrap();

    let resource_type = instance().resource_types.get(&resource_type_name).unwrap();

    let mut outstanding: std::collections::BTreeMap<_, _> = std::iter::FromIterator::from_iter(
        resource_type
            .properties
            .iter()
            .map(|(key, value)| (mk_field_name(key), value.required)),
    );

    let existing: std::collections::BTreeSet<syn::Ident> = outstanding.keys().cloned().collect();

    let mut fields: Vec<proc_macro2::TokenStream> = vec![];

    while let Some(key) = iterator.next() {
        let field_name =
            syn::parse::<syn::Ident>(key.into()).expect("Cannot parse field value as identifier");

        if !existing.contains(&field_name) {
            panic!("#{resource_type_name} has no field #{field_name}")
        }

        let value = syn::parse::<syn::Expr>(
            iterator
                .next()
                .expect("Field #{field_name} has no value")
                .into(),
        )
        .expect("Cannot parse field value as expression");

        let required = match outstanding.remove(&field_name) {
            Some(required) => required,
            None => panic!("#{resource_type_name} field #{field_name} specified more than once!"),
        };

        fields.push(if required {
            quote::quote! { #field_name: #value.into() }
        } else {
            quote::quote! {
                #field_name: Some(#value.into())
            }
        })
    }

    let mut missing_required: Vec<syn::Ident> = vec![];

    for (field_name, required) in outstanding {
        if required {
            missing_required.push(field_name);
        } else {
            fields.push(quote::quote! { #field_name: None })
        }
    }

    if !missing_required.is_empty() {
        panic!("Missing required fields: #{missing_required:#?}");
    }

    let path = {
        let vendor_module_name = VendorModuleName::new(&resource_type_name.service.vendor_name);
        let service_module_name = ServiceModuleName::new(&resource_type_name.service.service_name);

        quote::quote! {
            cloudformation::#vendor_module_name::#service_module_name
        }
    };

    let struct_name = resource_type_struct_name(&resource_type_name);

    quote::quote! {
        #path::#struct_name { #(#fields),* }
    }
    .into()
}

#[proc_macro]
pub fn construct_property_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut iterator = input.into_iter();

    let first = iterator.next().expect("Missing property type!");

    let name = syn::parse::<syn::LitStr>(first.clone().into())
        .unwrap_or_else(|error| {
            panic!("Could not parse first argument as string: {error}, but got: #{first}")
        })
        .value();

    let property_type_name: PropertyTypeName =
        std::convert::TryFrom::try_from(name.as_ref()).unwrap();

    let property_type = instance().property_types.get(&property_type_name).unwrap();

    let mut outstanding = match property_type.properties {
        Some(ref properties) => std::iter::FromIterator::from_iter(
            properties
                .iter()
                .map(|(key, value)| (mk_field_name(key), value.required)),
        ),
        None => BTreeMap::new(),
    };

    let existing: std::collections::BTreeSet<syn::Ident> = outstanding.keys().cloned().collect();

    let mut fields: Vec<proc_macro2::TokenStream> = vec![];

    while let Some(key) = iterator.next() {
        let field_name =
            syn::parse::<syn::Ident>(key.into()).expect("Cannot parse field value as identifier");

        if !existing.contains(&field_name) {
            panic!("#{property_type_name} has no field #{field_name}")
        }

        let value = syn::parse::<syn::Expr>(
            iterator
                .next()
                .expect("Field #{field_name} has no value")
                .into(),
        )
        .expect("Cannot parse field value as expression");

        let required = match outstanding.remove(&field_name) {
            Some(required) => required,
            None => {
                panic!("#{property_type_name} field #{field_name} was specified more than once")
            }
        };

        fields.push(if required {
            quote::quote! { #field_name: #value.into() }
        } else {
            quote::quote! {
                #field_name: Some(#value.into())
            }
        })
    }

    let mut missing_required: Vec<syn::Ident> = vec![];

    for (field_name, required) in outstanding {
        if required {
            missing_required.push(field_name);
        } else {
            fields.push(quote::quote! { #field_name: None })
        }
    }

    if !missing_required.is_empty() {
        panic!("Missing required fields: #{missing_required:#?}");
    }

    let (path, struct_name) = match property_type_name {
        PropertyTypeName::Tag => (
            quote::quote! { crate::cloudformation },
            property_name_struct_name(&PropertyName("Tag")),
        ),
        PropertyTypeName::PropertyTypeName(ref name) => {
            let vendor_module_name = VendorModuleName::new(&name.vendor_name);
            let service_module_name = ServiceModuleName::new(&name.service_name);
            let resource_module_name = ResourceModuleName::new(&name.resource_name);

            (
                quote::quote! {
                    cloudformation::#vendor_module_name::#service_module_name::#resource_module_name
                },
                property_name_struct_name(&name.property_name),
            )
        }
    };

    quote::quote! {
        #path::#struct_name { #(#fields),* }
    }
    .into()
}
