//! This crate provides procedural macro that can be used in combination with the ordo crate
//! to simplify usage.

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

/// Implements the Action trait on an enum.
#[proc_macro_derive(Action)]
pub fn ordo_derive(input: TokenStream) -> TokenStream {
    // Construct a represntation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    ordo_macro(&ast)
}

fn ordo_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Action for #name {}
    };
    gen.into()
}

// TODO create js enums
// #[derive(ordo)]
// pub struct Increment {
//   type: String
//

// const increment = (payload) => {
//  return {
//      type: 'INCREMENT,
//      payload: payload
//  }
// }

/// Implements the Action and Clone traits on an enum.
#[proc_macro_attribute]
pub fn action(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Use TokenStream from proc_macro2 to allow usage in quote! macro
    // See: https://users.rust-lang.org/t/how-to-get-a-proc-macro-tokenstream-out-of-quote-solved/22517/3
    let input: TokenStream2 = item.into();
    let output = quote! {
        #[derive(Action, Clone)]
        #input
    };
    output.into()
}

/// Implement the Clone, Serialize and Deserialize traits on an enum.
#[proc_macro_attribute]
pub fn state(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: TokenStream2 = item.into();
    let output = quote! {
        #[derive(Clone, Serialize, Deserialize)]
        #input
    };
    output.into()
}
