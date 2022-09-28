extern crate proc_macro;
use proc_macro::TokenStream;
use std::fs;
use std::str::FromStr;
use cargo_toml::Manifest;


#[proc_macro]
pub fn package_name(_input: TokenStream) -> TokenStream {
    // let parsed = parse_macro_input!(input as DeriveInput);
    let toml = fs::read("Cargo.toml").unwrap();

    let manifest = Manifest::from_slice(&toml).unwrap();

    let name = manifest.package.unwrap().name;

  TokenStream::from_str(&format!("pub const PACKAGE_NAME: &str = \"{}\";", name)).unwrap()
}
