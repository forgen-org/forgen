extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, Item, ItemEnum, ItemImpl, ItemStruct, ItemTrait};

#[proc_macro_attribute]
pub fn port(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(item as ItemTrait);

    let expanded = quote! {
        // #[delegate]
        #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
        #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
        #item_fn
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn service(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(item as ItemImpl);

    let expanded = quote! {
        #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
        #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
        #item_fn
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn signal(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    let expanded = match input {
        Item::Enum(item_enum) => expand_enum(item_enum),
        Item::Struct(item_struct) => expand_struct(item_struct),
        _ => panic!("`#[signal]` can only be used with enums and structs"),
    };

    println!("generated code is:\n{}", &expanded.to_string());

    TokenStream::from(expanded)
}

fn expand_enum(item_enum: ItemEnum) -> proc_macro2::TokenStream {
    let name = &item_enum.ident;

    quote! {
        #[derive(Debug, tsify::Tsify, serde::Serialize, serde::Deserialize, uniffi::Enum)]
        #[tsify(into_wasm_abi, from_wasm_abi)]
        #[serde(untagged)]
        #item_enum

        impl From<#name> for wasm_bindgen::JsValue {
            fn from(signal: #name) -> wasm_bindgen::JsValue {
                serde_wasm_bindgen::to_value(&signal).unwrap()
            }
        }
    }
}

fn expand_struct(item_struct: ItemStruct) -> proc_macro2::TokenStream {
    let name = &item_struct.ident;
    let fields = &item_struct.fields;

    match fields {
        Fields::Named(_) => quote! {
            #[derive(Debug, tsify::Tsify, serde::Serialize, serde::Deserialize, uniffi::Record)]
            #[tsify(into_wasm_abi, from_wasm_abi)]
            #item_struct

            impl From<#name> for wasm_bindgen::JsValue {
                fn from(signal: #name) -> wasm_bindgen::JsValue {
                    serde_wasm_bindgen::to_value(&signal).unwrap()
                }
            }
        },
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() != 1 {
                panic!("Newtype structs must have exactly one field");
            }
            let newtype = &fields.unnamed.iter().next().unwrap().ty;
            quote! {
                uniffi::custom_newtype!(#name, #newtype);

                #[derive(Debug, tsify::Tsify, serde::Serialize, serde::Deserialize)]
                #[tsify(into_wasm_abi, from_wasm_abi)]
                #item_struct

                impl From<#name> for wasm_bindgen::JsValue {
                    fn from(signal: #name) -> wasm_bindgen::JsValue {
                        serde_wasm_bindgen::to_value(&signal).unwrap()
                    }
                }
            }
        }
        Fields::Unit => quote! {
            #[derive(Debug, tsify::Tsify, serde::Serialize, serde::Deserialize, uniffi::Record)]
            #[tsify(into_wasm_abi, from_wasm_abi)]
            #item_struct

            impl From<#name> for wasm_bindgen::JsValue {
                fn from(signal: #name) -> wasm_bindgen::JsValue {
                    serde_wasm_bindgen::to_value(&signal).unwrap()
                }
            }
        },
    }
}
