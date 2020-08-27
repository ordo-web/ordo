use proc_macro2::Ident;
use std::fs::{create_dir, remove_dir_all, File};
use std::io::Write;
use std::path::Path;
use syn::DataEnum;

static mut FLAG: bool = false;

pub fn generate_js_actions(name: &Ident, data: &DataEnum) {
    let binding_path = "./ordo-bindings";
    // Delete existing ordo bindings
    unsafe {
        // Check if this is the first macro invocation
        if !FLAG {
            // If yes, check if bindings already exist and delete them if so
            let path = Path::new(binding_path.clone());
            if path.exists() {
                let _ = remove_dir_all(path);
            }
            let _ = create_dir(path);
            FLAG = true;
        }
    }
    // Parse the enum name of the action
    let enum_name = string_between(format!("{:?}", &name), "\"", "\"");
    // Create path to new bindings file
    let path = String::from(binding_path) + "/" + &enum_name + ".js";
    // Create file
    let mut file = match File::create(&path) {
        Err(why) => panic!(
            "Ordo Error: Could not create js bindings for {}: {}",
            &enum_name, why
        ),
        Ok(file) => file,
    };
    // Iterate over all actions
    // Save generated code in the `actions` String
    let mut actions = String::new();
    for variant in data.variants.iter() {
        // Parse name of the specific action
        let action_name = string_between(format!("{:?}", &variant.ident), "\"", "\"");
        // Check if it contains a payload
        let has_param = if variant.fields != syn::Fields::Unit {
            true
        } else {
            false
        };
        // Generate and save js bindings code
        actions.push_str(&generate_code(&enum_name, &action_name, has_param));
    }
    // Write all generated bindings to output file
    match file.write_all(actions.as_bytes()) {
        Err(why) => panic!(
            "Ordo Error: Could not write generated code to output-file {}.js: {}",
            &enum_name, why
        ),
        Ok(_) => {}
    }
}

// Generates the js binding for an action.
fn generate_code(enum_name: &str, name: &str, has_param: bool) -> String {
    let func_name = build_func_name(name.clone());
    if has_param {
        let mut action = String::from("export const ");
        action.push_str(&func_name);
        action.push_str(" = (payload) => {\n   return {\n      ident: '");
        action.push_str(enum_name);
        action.push_str("',\n      action: {\n");
        action.push_str("         type: '");
        action.push_str(&name);
        action.push_str("',\n         payload: payload\n      }\n   }\n};\n\n");
        action
    } else {
        let mut action = String::from("export const ");
        action.push_str(&func_name);
        action.push_str(" = () => {\n   return {\n      ident: '");
        action.push_str(enum_name);
        action.push_str("',\n      action: {\n");
        action.push_str("         type: '");
        action.push_str(&name);
        action.push_str("',\n      }\n   }\n};\n\n");
        action
    }
}

fn build_func_name(name: &str) -> String {
    if name.to_uppercase().eq(name) {
        name.to_lowercase()
    } else {
        // De-Capitalize first letter
        // Src: https://stackoverflow.com/a/38406885/12347616
        let mut c = name.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
        }
    }
}

// Used to parse idents
// See: https://stackoverflow.com/a/37784410/12347616
pub fn string_between(string: String, begin_pattern: &str, end_pattern: &str) -> String {
    let start_bytes = string.find(begin_pattern).unwrap_or(0) + 1;
    let end_bytes = string.rfind(end_pattern).unwrap_or(string.len());
    String::from(&string[start_bytes..end_bytes])
}
