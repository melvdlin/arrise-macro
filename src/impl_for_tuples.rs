use itertools::Itertools;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use syn::{LitInt, Result};

pub fn impl_for_tuples(args: proc_macro::TokenStream) -> Result<TokenStream> {
    let arity = syn::parse::<LitInt>(args)?.base10_parse()?;
    let (fields, names): (Vec<_>, Vec<_>) = (0..arity)
        .map(|n| (Literal::usize_unsuffixed(n), format_ident!("T{n}")))
        .unzip();

    let impls = (0..arity)
        .map(|arity| (fields.iter().take(arity), names.iter().take(arity)))
        .map(|(fields, names)| {
            quote! {
                impl_for_tuple!((#(#fields: #names,)*));
            }
        })
        .collect_vec();

    Ok(quote! {
        #(#impls)*
    })
}
