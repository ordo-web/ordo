use proc_macro2::Ident;
use proc_macro2::TokenStream;
use syn::DataEnum;

pub fn generate_utilities(name: &Ident, data: &DataEnum) -> TokenStream {
    let mut gen = TokenStream::new();

    let concatenated = format!("parse_{}", name);
    let func_name = syn::Ident::new(&concatenated, name.span());

    gen.extend(quote! {
        impl Action for #name {}

        #[allow(non_snake_case)]
        pub fn #func_name(val: Value) -> Result<Box<dyn Any>, ()> {
            log("HEY!");
            log(&format!("The val: {:?}", &val));
            match serde_json::from_value::< #name >(val) {
                Ok(val) => {
                    log("DA!");
                    Ok(Box::new(val))
                },
                Err(err) => {
                    let name = stringify!(#name);
                    console_error!("Ordo Critical-Error: Conversion of Action {} dispatched from JavaScript context failed. \nCause: {}", &name, &err);
                    Err(())
                }
            }
        }
    });
    gen
}
