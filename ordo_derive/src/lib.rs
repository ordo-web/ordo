//! This crate provides procedural macro that can be used in combination with the ordo crate
//! to simplify usage.

mod generate;
mod helper;

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use std::collections::HashMap;
use syn::Data;

/// Implements the Action trait on an enum.
/// Also generates the js bindings for the action.
#[proc_macro_derive(Action)]
pub fn ordo_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();
    // Build the trait implementation
    ordo_macro(ast)
}

fn ordo_macro(ast: syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // Panic if the annotated item is not an enum
    if let Data::Enum(data) = &ast.data {
        // Generate the js bindings
        generate::generate_js_actions(name, data);
        // Implement trait
        let gen = helper::generate_utilities(name, data);
        gen.into()
    } else {
        panic!("Ordo Error: Only Enums can be annotated with the #[action] macro");
    }
}

/// Implements the Action and Clone traits on an enum.
#[proc_macro_attribute]
pub fn action(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Use TokenStream from proc_macro2 to allow usage in quote! macro
    // See: https://users.rust-lang.org/t/how-to-get-a-proc-macro-tokenstream-out-of-quote-solved/22517/3
    let input: TokenStream2 = item.into();
    let output = quote! {
        #[derive(Action, Clone, Serialize, Deserialize)]
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
