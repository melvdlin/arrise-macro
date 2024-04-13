use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{LitInt, Result};

pub fn impl_for_tuples(args: proc_macro::TokenStream) -> Result<TokenStream> {
    let arity = syn::parse::<LitInt>(args)?.base10_parse()?;
    let names = (0..arity).map(|n| format_ident!("T{n}"));

    let impls = (0..arity)
        .map(|arity| names.clone().take(arity))
        .map(|names| {
            quote! {
                impl_for_tuple!((#(#names,)*));
            }
        })
        .collect_vec();

    Ok(quote! {
        #(#impls)*
    })
}
