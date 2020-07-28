use serde_json::Value;
use wasm_bindgen::JsValue;
use wasm_bindgen::__rt::core::cell::RefCell;

pub(crate) struct Cache {
    cache: RefCell<Value>,
}

impl Cache {
    pub(crate) fn new() -> Cache {
        Cache {
            cache: RefCell::new(Value::Null),
        }
    }

    pub(crate) fn get_state(&self) -> JsValue {
        match JsValue::from_serde(&self.cache) {
            Ok(state) => state,
            Err(err) => panic!(err),
        }
    }
    
    pub(crate) fn update_state(&self, new_state: Value) {
        self.cache.replace(new_state);
    }
}
