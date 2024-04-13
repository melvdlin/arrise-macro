use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error};

mod deserialize_error_type_for_tuple;
mod invoke_with_type_names;

/// Generate a tuple with trailing commas of the specified arity
/// and pass it into an invocation of the provided macro.
#[proc_macro]
pub fn invoke_with_tuple(args: TokenStream) -> TokenStream {
    invoke_with_type_names::invoke_with_tuple(args)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Generate a deserialize error type for a given tuple.
#[proc_macro]
pub fn deserialize_error_type_for_tuple(args: TokenStream) -> TokenStream {
    deserialize_error_type_for_tuple::deserialize_error_type_for_tuple(args)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Generate a deserialize error type name for a given tuple.
#[proc_macro]
pub fn deserialize_error_type_name_for_tuple(args: TokenStream) -> TokenStream {
    deserialize_error_type_for_tuple::deserialize_error_type_name_for_tuple(args)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(MyDerive)]
pub fn my_derive(_input: TokenStream) -> TokenStream {
    let tokens = quote! {
        struct Hello;
    };

    tokens.into()
}

/// Example of user-defined [procedural macro attribute][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros
#[proc_macro_attribute]
pub fn my_attribute(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens = quote! {
        #input

        struct Hello;
    };

    tokens.into()
}
