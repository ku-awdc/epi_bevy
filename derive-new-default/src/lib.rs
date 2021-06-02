extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};


#[proc_macro_derive(DefaultNew)]
pub fn derive_default_new(input: TokenStream) -> TokenStream {

    let DeriveInput { ident, data, ..} = parse_macro_input!(input);

    match data {
        syn::Data::Struct(data_struct) => {
            match data_struct.fields {
                syn::Fields::Named(fields_named) => todo!(),
                syn::Fields::Unnamed(fields_unnamed) => {
                    fields_unnamed
                },
                syn::Fields::Unit => todo!(),
            }
        },
        syn::Data::Enum(data_enum) => todo!(),
        syn::Data::Union(_) => todo!(),
    };

    let output = quote! {
        impl #ident {
            pub fn new_default(value: T) -> Self {
                Self(value, PhantomData)
            }
        }
    };

    TokenStream::from(output)
}