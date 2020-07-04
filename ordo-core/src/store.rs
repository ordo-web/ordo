use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Store {

}

impl Store {
    pub fn createStore() {

    }
}


#[wasm_bindgen]
impl Store {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Store {
        Store {}
    }
}