mod abi;
mod constants;

extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{ItemMod, parse_macro_input};

#[proc_macro_attribute]
pub fn contract_module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut module = parse_macro_input!(item as ItemMod);
    let mod_name = &module.ident;

    eprintln!("🛠  contract_module running on module: {}", mod_name);

    _attr
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
