extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_derive(Action)]
pub fn ordo_derive(input: TokenStream) -> TokenStream {
    // Construct a represntation of Rust code as a syntax tree
    // that we can manipulate

    println!("{:?}", &input);

    let ast = syn::parse(input).unwrap();

    println!("{:?}", &ast);

    // Build the trait implementation
    ordo_macro(&ast)
}

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

// TODO create js enums

fn ordo_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        //impl HelloMacro for #name {
        //    fn hello_macro() {
        //        println!("Hello, Macro! My name is {}", stringify!(#name));
        //    }
        //}
        //println!("Hello, Macro! My name is {}", stringify!(#name));

        /**impl Copy for #name{}

        impl Clone for #name {
            fn clone(&self) -> Self {
                *self
            }
        }*/

        impl Action for #name {}
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn action(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: TokenStream2 = item.into();
    let output = quote! {
        #[derive(Action, Clone)]
        #input
    };
    output.into()
}
