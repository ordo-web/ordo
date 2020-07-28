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
