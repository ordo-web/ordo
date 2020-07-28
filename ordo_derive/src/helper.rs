use crate::generate::string_between;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use std::fs::{create_dir, remove_dir_all, File};
use std::io::Write;
use std::path::Path;
use syn::DataEnum;

//static mut FLAG: bool = false;

pub fn generate_utilities(name: &Ident, data: &DataEnum) -> TokenStream {
    let mut gen = TokenStream::new();

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
    let concatenated3 = format!("parse_{}", name);
    let newname = syn::Ident::new(&concatenated, name.span());
    let macro_name = syn::Ident::new(&concatenated2, name.span());
    let func_name = syn::Ident::new(&concatenated3, name.span());

    gen.extend(quote! {
        impl Action for #name {}

        /*pub fn #func_name(name: String, payload: Option<Box<dyn Any>>) -> MyAction2{
            let flag = payload.is_some();
            let payload = payload.unwrap();
            if name.as_str() == "INCREMENT" && flag && payload.is::<String>() {
                MyAction2::INCREMENT(*payload.downcast::<String>().unwrap())
            } else {
                MyAction2::DECREMENT
            }
        }*/
        #[allow(non_snake_case)]
        pub fn #func_name(val: Value) -> Box<dyn Action> {
            match serde_json::from_value::<MyAction2>(val) {
                Ok(val) => {
                    Box::new(val)
                },
                Err(_) => panic!("Conversion for value of type #name failed")
            }
        }
    });
    gen
}
