use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::TypeTuple;

pub fn deserialize_error_type_name_for_tuple(
    args: proc_macro::TokenStream,
) -> syn::Result<TokenStream> {
    let tuple_arg = syn::parse::<TypeTuple>(args)?;
    let typename = format_ident!("Deserialize{}TupleError", tuple_arg.elems.len());
    Ok(quote!(#typename))
}

pub fn deserialize_error_type_for_tuple(args: proc_macro::TokenStream) -> syn::Result<TokenStream> {
    let tuple_arg = syn::parse::<TypeTuple>(args.clone())?;
    let all_tuple_elements = tuple_arg.elems.clone();
    let tuple_elements = all_tuple_elements.iter().cloned().collect_vec();
    let typename = deserialize_error_type_name_for_tuple(args)?;

    let error_type = quote! {
        #[derive(Debug, Clone, Eq, PartialEq)]
        pub enum #typename<#all_tuple_elements> {
        #(
            #tuple_elements(#tuple_elements),
        )*
        }
    };

    Ok(error_type)
}
