use crate::action::Action;
use crate::store::Store;
use crate::transport::Transport;
use js_sys::Uint8Array;
use serde_json::value::Value;
use wasm_bindgen::__rt::core::cell::{Cell, RefCell};
use wasm_bindgen::__rt::std::rc::Rc;

pub type RefStore = Rc<RefCell<dyn Store + 'static>>;

pub struct PrimeNode {
    store: RefStore,
    subscriptions: RefCell<Vec<Box<dyn Fn(&Value)>>>,
    transport: Cell<Option<Transport>>,
}

#[doc(hidden)]
pub fn __build_prime_node(store: impl Store + 'static) -> Rc<PrimeNode> {
    let store = Rc::new(RefCell::new(store));

    let prime_node = Rc::new(PrimeNode {
        store,
        subscriptions: RefCell::new(Vec::new()),
        transport: Cell::new(None),
    });

    let transport = Transport::new(prime_node.clone());
    prime_node.transport.set(Some(transport));
    prime_node
}

impl PrimeNode {
    pub fn get_state(&self) -> Value {
        self.store.borrow_mut().get_state()
    }

    pub fn dispatch(&self, action: impl Action + 'static) {
        let action = Box::new(action);
        // Check if action is valid
        if self.store.borrow_mut().dispatch(action) {
            let state: Value = self.get_state();
            // Trigger subscriptions if they exist
            if self.subscriptions.borrow().len() > 0 {
                for subscription in self.subscriptions.borrow_mut().iter() {
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
                // TODO move this to seperate transport function
                /*
                let res = self.ctx.post_message(&ser);
                match res {
                    Ok(_) => {
                        log("OKKK");
                    }
                    Err(err) => {
                        log(&format!("NOT OKKK: {:?}", err));
                    }
                }*/
            }

            // TODO use deserialization in node
            //let val: Value = serde_json::from_slice(&serialized).unwrap();
            //log(&format!("Deserialized value: {:?}", &val));
        }
    }

    pub fn subscribe(&self, subscription: impl Fn(&Value) + 'static) {
        let subscription = Box::new(subscription);
        self.subscriptions.borrow_mut().push(subscription);
    }
}
