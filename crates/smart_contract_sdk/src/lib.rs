mod abi;
mod constants;

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, FnArg, ItemMod, Pat, PatType};

#[proc_macro_attribute]
pub fn contract(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let orig_mod = parse_macro_input!(input as ItemMod);
    let mod_name = &orig_mod.ident;

    let mut cleaned = orig_mod.clone();

    cleaned.attrs.retain(|attr| {
        attr.path().get_ident().map(|i| i.to_string()) != Some("contract".to_string())
    });

    if let Some((_, items)) = &mut cleaned.content {
        for item in items.iter_mut() {
            if let syn::Item::Fn(func) = item {
                func.attrs.retain(|attr| {
                    !matches!(
                        attr.path().get_ident().map(|i| i.to_string()).as_deref(),
                        Some("entrypoint") | Some("view") | Some("tx")
                    )
                });
            }
        }
    }

    let mut wrappers = quote! {};
    if let Some((_, items)) = &orig_mod.content {
        for item in items {
            if let syn::Item::Fn(func) = item {
                let fn_name = &func.sig.ident;
                let inputs = &func.sig.inputs;
                let output = &func.sig.output;
                let args: Vec<_> = func
                    .sig
                    .inputs
                    .iter()
                    .filter_map(|arg| {
                        if let FnArg::Typed(PatType { pat, .. }) = arg {
                            if let Pat::Ident(pi) = &**pat {
                                Some(pi.ident.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();
                for attr in &func.attrs {
                    let kind = attr.path().get_ident().unwrap().to_string();
                    match kind.as_str() {
                        "entrypoint" => {
                            wrappers.extend(quote! {
                                #[no_mangle]
                                pub extern "C" fn _start() {
                                    #mod_name::#fn_name();
                                }
                            });
                        }
                        "view" => {
                            let wrapper = format_ident!("__view_{}", fn_name);
                            wrappers.extend(quote! {
                                #[no_mangle]
                                pub extern "C" fn #wrapper(#inputs) #output {
                                    let res = #mod_name::#fn_name(#(#args),*);
                                    res
                                }
                            });
                        }
                        "tx" => {
                            let wrapper = format_ident!("__tx_{}", fn_name);
                            wrappers.extend(quote! {
                                #[no_mangle]
                                pub extern "C" fn #wrapper(#inputs) #output {
                                    #mod_name::#fn_name(#(#args),*);
                                }
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let expanded = quote! {
        #cleaned
        #wrappers
    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn entrypoint(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn view(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn tx(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
