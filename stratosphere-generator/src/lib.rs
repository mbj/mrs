use std::collections::BTreeMap;
use stratosphere_core::resource_specification::*;
use stratosphere_core::token::*;

#[proc_macro]
pub fn generate_service(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if !input.is_empty() {
        panic!("Macro does not take inputs!")
    }

    let selected_service = ServiceIdentifier {
        vendor_name: <VendorName as std::convert::TryFrom<&str>>::try_from("AWS").unwrap(),
        service_name: <ServiceName as std::convert::TryFrom<&str>>::try_from("CertificateManager").unwrap(),
    };

    token_stream(Target::for_service(instance(), &selected_service)).into()
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
        Some(ref properties) => {
            std::iter::FromIterator::from_iter(properties.into_iter().map(|(key, value)| {
                (
                    syn::Ident::new(key.as_ref(), proc_macro2::Span::call_site()),
                    value.required,
                )
            }))
        }
        None => BTreeMap::new(),
    };

    let mut fields: Vec<proc_macro2::TokenStream> = vec![];

    while let Some(key) = iterator.next() {
        let ident =
            syn::parse::<syn::Ident>(key.into()).expect("Cannot parse field value as identifier");

        let value =
            syn::parse::<syn::Expr>(iterator.next().expect("Field #{ident} has no value").into())
                .expect("Canntot parse field value as expression");

        let required = match outstanding.remove(&ident) {
            Some(required) => required,
            None => panic!("Field #{ident} specified more than once, or does not exist!"),
        };

        fields.push(if required {
            quote::quote! { #ident: #value.to_exp() }
        } else {
            quote::quote! {
                #ident: Some(#value.to_exp())
            }
        })
    }

    let mut missing_required: Vec<syn::Ident> = vec![];

    for (ident, required) in outstanding {
        if required {
            missing_required.push(ident);
        } else {
            fields.push(quote::quote! { #ident: None })
        }
    }

    if !missing_required.is_empty() {
        panic!("Missing required fields: #{missing_required:#?}");
    }

    let (path, struct_name_str) = match property_type_name {
        PropertyTypeName::Tag => (quote::quote! { crate::cloudformation }, "Tag"),
        PropertyTypeName::PropertyTypeName(ref name) => {
            let vendor_module_name = VendorModuleName::new(&name.vendor_name);
            let service_module_name = ServiceModuleName::new(&name.service_name);
            let resource_module_name = ResourceModuleName::new(&name.resource_name);

            (
                quote::quote! {
                    crate::cloudformation::#vendor_module_name::#service_module_name::#resource_module_name
                },
                name.property_name.as_str(),
            )
        }
    };

    let struct_name = syn::Ident::new(struct_name_str, proc_macro2::Span::call_site());

    quote::quote! {
        #path::#struct_name { #(#fields),* }
    }
    .into()
}
