use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::TypeTuple;

pub fn deserialize_error_type(args: proc_macro::TokenStream) -> syn::Result<TokenStream> {
    let tuple_arg = syn::parse::<TypeTuple>(args)?;
    let tuple_elements = tuple_arg.elems.iter().cloned().collect_vec();
    let typename = format_ident!("Deserialize{}TupleError", tuple_arg.elems.len());

    let error_type = quote! {
        #[derive(Debug, Clone, Eq, PartialEq)]
        pub enum #typename<#(#tuple_elements,)*> {
        #(
            #tuple_elements(#tuple_elements),
        )*
        }

        #(
        impl From<#tuple_elements> for #typename {
            fn from(value: #tuple_elements) -> Self {
                Self::#tuple_elements(value)
            }
        }
        )*
    };

    Ok(error_type)
}
