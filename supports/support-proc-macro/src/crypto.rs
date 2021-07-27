use proc_macro::TokenStream;

use proc_macro2::Span;
use proc_macro_roids::{DeriveInputStructExt, FieldExt}; // IdentExt
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, Ident};

// #[proc_macro_derive(BridgeCrypto, attributes(crypto))] // develop test only
#[allow(clippy::cmp_owned)]
pub fn crypto(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = ast.ident.clone();
    let fields = ast.fields();

    let is_enable_fields = fields
        .iter()
        .filter(|field| field.contains_tag(&parse_quote!(crypto), &parse_quote!(is_enable)))
        .filter(|field| field.type_name().to_string() == "bool")
        .collect::<Vec<_>>();
    if is_enable_fields.len() > 1 {
        panic!("There can only be one `is_enable` field");
    }
    let is_enable_field = is_enable_fields.first();

    let decrypt_fields = fields
        .iter()
        .filter(|field| !field.is_phantom_data())
        .filter(|field| field.contains_tag(&parse_quote!(crypto), &parse_quote!(decrypt)))
        .filter(|field| field.type_name().to_string() == "String");

    let decrypt_fields_tokens = decrypt_fields
        .map(|field| {
            let type_name = field.type_name();
            let ident_field_crypto = field.ident.clone().unwrap();
            let variant_name = format!("{}_decrypt", ident_field_crypto);
            let variant_name = Ident::new(&variant_name, Span::call_site());

            let quote_decrypt = quote! {
                let crypto = bridge_primitives::crypto::Crypto::new();
                Ok(crypto.decrypt(password.as_ref(), &self.#ident_field_crypto)?)
            };

            match is_enable_field {
                Some(ief) => {
                    let ident_field_is_enable = ief.ident.clone().unwrap();
                    quote! {
                        pub fn #variant_name(&self, password: impl AsRef<str>) -> anyhow::Result<#type_name> {
                            if !self.#ident_field_is_enable {
                                return Ok(self.#ident_field_crypto.clone());
                            }
                            #quote_decrypt
                        }
                    }
                }
                None => {
                    quote! {
                        pub fn #variant_name(&self, password: impl AsRef<str>) -> anyhow::Result<#type_name> {
                            #quote_decrypt
                        }
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let quote_is_enable = match is_enable_field {
        Some(ief) => {
            let ident_field_is_enable = ief.ident.clone().unwrap();
            quote! {
                pub fn is_enable_crypto(&self) -> bool {
                    self.#ident_field_is_enable
                }
            }
        }
        None => quote! {
            pub fn is_enable_crypto(&self) -> bool { true }
        },
    };

    let output = quote! {
        impl #struct_name {
            #quote_is_enable
            #(#decrypt_fields_tokens)*
        }
    };

    output.into()
}
