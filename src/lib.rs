use proc_macro::{TokenStream};
use quote::quote;
use cargo_toml::Manifest;


#[proc_macro]
pub fn package_name(_input: TokenStream) -> TokenStream {
    // let parsed = parse_macro_input!(input as DeriveInput);
    let manifest = Manifest::from_path("Cargo.toml").unwrap();

    let name = manifest.package.unwrap().name;

    let res = quote!(
        pub const PACKAGE_NAME: &str = stringify!(#name);
    );

    res.into()
}
