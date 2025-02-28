use proc_macro::TokenStream;
use syn::punctuated::Punctuated;

use quote::quote;
use syn;

#[proc_macro]
pub fn gen_service_enum(input: TokenStream) -> TokenStream {
    if !input.is_empty() {
        panic!("Macro does not take inputs!")
    }

    let instance = stratosphere_resource_specification::instance();

    let iter = instance.services().map(|name|
        syn::Variant {
            attrs: vec![],
            ident: syn::Ident::new(name.as_ref(), proc_macro2::Span::call_site()),
            discriminant: None,
            fields: syn::Fields::Unit,
        }
    );

    let variants : Punctuated<syn::Variant, syn::Token![,]> = iter.collect();

    quote::quote! {
      /// AWS Services supported by stratosphere
      #[derive(strum::IntoStaticStr)]
      enum Service { #variants }
    }.into()
}
