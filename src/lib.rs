extern crate proc_macro;
use proc_macro::TokenStream;
use std::fs;
use std::str::FromStr;
use cargo_toml::Manifest;

use syn::{parse_macro_input, DeriveInput};
use quote::quote;

/// Example of [function-like procedural macro][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros
#[proc_macro]
pub fn package_name(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as DeriveInput);
    let toml = fs::read("Cargo.toml").unwrap();

    let manifest = Manifest::from_slice(&toml).unwrap();

    let name = manifest.package.unwrap().name;

  TokenStream::from_str(&format!("pub const PACKAGE_NAME: &str = {};", name)).unwrap()
}
