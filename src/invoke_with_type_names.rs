use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{LitInt, Path, Result, Token};

#[derive(Debug, Clone)]
struct Args {
    macro_path: Path,
    comma: Token![,],
    names: usize,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            macro_path: input.parse()?,
            comma: input.parse()?,
            names: input.parse::<LitInt>()?.base10_parse()?,
        })
    }
}

pub fn invoke_with_type_names(args: proc_macro::TokenStream) -> Result<TokenStream> {
    #[allow(unused_variables)]
    let Args {
        macro_path,
        comma,
        names,
    } = syn::parse::<Args>(args)?;

    let names = (0..names).map(|n| format_ident!("T{n}"));
    Ok(quote! {
        #macro_path! {
            #(#names)*
        }
    })
}
