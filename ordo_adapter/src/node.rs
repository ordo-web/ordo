use crate::adapter::Adapter;
use crate::adapter::AdapterNode;
use crate::log;
use crate::sleep;
use crate::utils::set_panic_hook;
use js_sys::Array;
use js_sys::Function;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::Worker;

#[wasm_bindgen]
pub struct Node {
    adapter: AdapterNode,
}

impl Node {}

#[wasm_bindgen]
impl Node {
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: Worker) -> Node {
        set_panic_hook();
        Node {
            adapter: Adapter::new(ctx),
        }
    }

    #[wasm_bindgen(js_name = getState)]
    pub fn get_state(&self) -> JsValue {
        self.adapter.get_state()
    }

    pub fn dispatch(&self, action: JsValue) {
        // Check if action is conform
        match js_sys::Object::try_from(&action) {
            Some(obj) => {
                let keys: Array = js_sys::Object::keys(&obj);
                let found_ident = keys.index_of(&JsValue::from("ident"), 0);
                let found_action = keys.index_of(&JsValue::from("action"), 0);
                if found_ident != -1 && found_action != -1 {
                    self.adapter.dispatch(action);
                }
            }
            None => {
                console_log!("The given action {:?} does not match the specs. Did you use the generated bindings?", &action)
            }
        }
    }

    pub fn subscribe(&self, subscription: Function) {
        self.adapter.subscribe(subscription);
    }

    pub fn unsubscribe(&self, subscription: Function) {
        self.adapter.unsubscribe(subscription);
    }

    pub fn ready(&self) -> Promise {
        let adapter = self.adapter.clone();
        future_to_promise(async move {
            loop {
                if adapter.initialized() {
                    break;
                } else {
                    // Redo handshake
                    adapter.send_value(JsValue::undefined());
                    let _ = JsFuture::from(sleep(10.0)).await;
                }
            }
            Ok(JsValue::null())
        })
    }
}
