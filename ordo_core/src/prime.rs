use crate::action::Action;
use crate::log;
use crate::store::Store;
use js_sys::Uint8Array;
use serde_json::value::Value;
use wasm_bindgen::JsValue;
use web_sys::Worker;

pub struct PrimeNode {
    store: Box<dyn Store + 'static>,
    subscriptions: Vec<Box<dyn Fn(&Value)>>,
    ctx: Worker,
}

#[doc(hidden)]
pub fn __build_prime_node(store: impl Store + 'static) -> PrimeNode {
    let ctx = Worker::from(JsValue::from(js_sys::global()));
    let _ = ctx.post_message(&JsValue::from("CTX here speaking")); // TODO remove later
    PrimeNode {
        store: Box::new(store),
        subscriptions: Vec::new(),
        ctx,
    }
}

impl PrimeNode {
    pub fn get_state(&self) -> Value {
        self.store.get_state()
    }

    pub fn dispatch(&mut self, action: impl Action + 'static) {
        let action = Box::new(action);
        // Check if action is valid
        if self.store.dispatch(action) {
            let state: Value = self.get_state();
            // Trigger subscriptions if they exist
            if self.subscriptions.len() > 0 {
                for subscription in self.subscriptions.iter() {
                    subscription(&state);
                }
            }

            // TODO ctx send new state

            let serialized = serde_json::to_vec(&state).unwrap();
            unsafe {
                // Does not work: TypeError: cannot transfer WebAssembly/asm.js ArrayBuffer
                // Get Transferable
                // See: https://github.com/rustwasm/wasm-bindgen/issues/1516
                /*
                let ser = Uint8Array::view(&serialized);
                let ser = ser.buffer();
                let res = self.ctx.post_message_with_transfer(&ser, &Array::of1(&ser));
                match res {
                    Ok(_) => {
                        log("OKKK");
                    }
                    Err(err) => {
                        log(&format!("NOT OKKK: {:?}", err));
                    }
                }*/

                let ser = Uint8Array::view(&serialized);
                let res = self.ctx.post_message(&ser);
                match res {
                    Ok(_) => {
                        log("OKKK");
                    }
                    Err(err) => {
                        log(&format!("NOT OKKK: {:?}", err));
                    }
                }
            }

            // TODO use deserialization in node
            //let val: Value = serde_json::from_slice(&serialized).unwrap();
            //log(&format!("Deserialized value: {:?}", &val));
        }
    }

    pub fn subscribe(&mut self, subscription: impl Fn(&Value) + 'static) {
        let subscription = Box::new(subscription);
        self.subscriptions.push(subscription);
    }
}
