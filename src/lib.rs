extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Meta, DataEnum, MetaList};
use std::str::FromStr;
use syn::spanned::Spanned;

#[proc_macro_derive(From, attributes(default,from_str))]
pub fn from_str_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_from(&input)
}

fn impl_from(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;
    let mut from_str_tokens = vec![];
    let mut from_int_tokens = vec![];
    let mut data_type = proc_macro2::TokenStream::new() ;
    for attr in &input.attrs {
        if attr.path().is_ident("repr") {
            if let Meta::List(meta_list) = &attr.meta {
                data_type = meta_list.tokens.clone();
            }
        }
    }
    match &input.data {
        Data::Struct(_) => {}
        Data::Enum(enum_data) => {
            impl_match_val(&enum_data, &mut from_str_tokens, &mut from_int_tokens, &data_type);
            impl_default(&enum_data, &mut from_str_tokens, &mut from_int_tokens, &data_type);
        }
        Data::Union(_) => {}
    }
    let mut impl_trait_tokens = vec![];
    let token = quote! {
        impl From<&str> for #name {
            fn from(value: &str) -> Self {
                match value {
                    #(#from_str_tokens),*
                }
            }
        }
    };
    impl_trait_tokens.push(token);

    if !data_type.is_empty() {
        let token = quote! {
        impl From<#data_type> for #name {
            fn from(value: #data_type) -> Self {
                match value {
                    #(#from_int_tokens),*
                }
            }
        }
    };
        impl_trait_tokens.push(token);
    }


    let token = quote! {
        #(#impl_trait_tokens)*
    };
    TokenStream::from(token)
}

fn impl_match_val(
    enum_data: &DataEnum,
    from_str_tokens: &mut Vec<proc_macro2::TokenStream>,
    from_int_tokens: &mut Vec<proc_macro2::TokenStream>,
    data_type: &proc_macro2::TokenStream
) {
    let mut index = 0;
    for variant in &enum_data.variants {
        let ident = &variant.ident;
        let token = quote! {
            stringify!(#ident) => Self::#ident
        };
        from_str_tokens.push(token);
        for attr in &variant.attrs {
            if attr.path().is_ident("from_str") {
                if let Ok(meta_list) = attr.meta.require_list() {
                    let expr = &meta_list.tokens;
                    let token = quote! {
                        stringify!(#expr) => Self::#ident
                    };
                    from_str_tokens.push(token);
                }
                break;
            }
        }
        if !data_type.is_empty() {
            if let Some((_, expr)) = &variant.discriminant {
                index = expr.to_token_stream().to_string().parse::<usize>().unwrap();
                let token = quote! {
                    #expr => Self::#ident
                };
                from_int_tokens.push(token);
            } else {
                let expr =  proc_macro2::TokenStream::from_str(index.to_string().as_str()).unwrap();
                let token = quote! {
                    #expr => Self::#ident
                };
                from_int_tokens.push(token);
            }
            let expr = proc_macro2::TokenStream::from_str(index.to_string().as_str()).unwrap();
            let token = quote! {
                stringify!(#expr) => Self::#ident
            };
            from_str_tokens.push(token);
            index += 1;
        }
    }
}

fn impl_default(
    enum_data: &DataEnum,
    from_str_tokens: &mut Vec<proc_macro2::TokenStream>,
    from_int_tokens: &mut Vec<proc_macro2::TokenStream>,
    data_type: &proc_macro2::TokenStream)
{
    let mut has_default = false;
    for variant in &enum_data.variants {
        for attr in &variant.attrs {
            if attr.path().is_ident("default") {
                let ident = &variant.ident;
                let token = quote! {
                     _ => Self::#ident
                };
                from_str_tokens.push(token.clone());
                if !data_type.is_empty() {
                    from_int_tokens.push(token);
                }
                has_default = true;
            }
        }
    }
    if !has_default {
        if let Some(first) = enum_data.variants.first() {
            let ident = &first.ident;
            let token = quote! {
                    _ => Self::#ident
                };
            from_str_tokens.push(token.clone());
            if !data_type.is_empty() {
                from_int_tokens.push(token);
            }
        }
    }
}