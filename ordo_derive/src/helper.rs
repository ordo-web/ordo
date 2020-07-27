use crate::generate::string_between;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use std::fs::{create_dir, remove_dir_all, File};
use std::io::Write;
use std::path::Path;
use syn::DataEnum;

static mut FLAG: bool = false;

pub fn generate_utilities(name: &Ident, data: &DataEnum) -> TokenStream {
    // Delete existing ordo bindings
    unsafe {
        // Check if this is the first macro invocation
        if !FLAG {
            // If yes, check if bindings already exist and delete them if so

            FLAG = true;
        }
    }

    // Parse the enum name of the action
    let enum_name = string_between(format!("{:?}", &name), "\"", "\"");

    // Iterate over all actions
    // Save generated code in the `actions` String
    let mut actions = String::new();
    for variant in data.variants.iter() {
        // Parse name of the specific action
        let action_name = string_between(format!("{:?}", &variant.ident), "\"", "\"");
        // Check if it contains a payload
        let has_param = if variant.fields == syn::Fields::Unit {
            true
        } else {
            false
        };

        // Generate and save js bindings code
    }

    let concatenated = format!("{}Baum", name);
    let concatenated2 = format!("da{}", name);
    let concatenated3 = format!("__func{}", name);
    let newname = syn::Ident::new(&concatenated, name.span());
    let macro_name = syn::Ident::new(&concatenated2, name.span());
    let func_name = syn::Ident::new(&concatenated3, name.span());

    let gen: TokenStream = quote! {
        impl Action for #name {}

        pub static #newname: &str = "a";

        #[macro_use]
        macro_rules! #macro_name {
            ($name: expr, $val: expr) => {{
                if $name.as_str() == "INCREMENT" {
                    MyAction2::INCREMENT($val)
                } else {
                    MyAction2::DECREMENT
                }
            }};
        }

        pub fn #func_name(name: String, payload: Option<Box<dyn Any>>) -> MyAction2{
            let flag = payload.is_some();
            let payload = payload.unwrap();
            if name.as_str() == "INCREMENT" && flag && payload.is::<String>() {
                MyAction2::INCREMENT(*payload.downcast::<String>().unwrap())
            } else {
                MyAction2::DECREMENT
            }
        }
    };
    gen
}
