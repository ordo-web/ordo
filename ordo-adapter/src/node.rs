use crate::adapter::Adapter;
use crate::adapter::AdapterNode;
use crate::sleep;
use crate::utils::set_panic_hook;
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

    pub fn dispatch(action: JsValue) {}

    pub fn subscribe(func: &Function) {}

    pub fn ready(&self) -> Promise {
        let adapter = self.adapter.clone();
        future_to_promise(async move {
            loop {
                if adapter.initialized() {
                    break;
                } else {
                    let _ = JsFuture::from(sleep(10.0)).await;
                }
            }
            Ok(JsValue::null())
        })
    }
}
