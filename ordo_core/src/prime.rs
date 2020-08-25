use crate::action::{Action, BabelError, TranslationLayer};
use crate::log;
use crate::store::Store;
use crate::transport::{Transport, TransportWrapper, TransportWrapperMethods};
use crate::utils::value_to_uint8array;
use js_sys::Uint8Array;
use serde_json::value::Value;
use wasm_bindgen::__rt::core::any::Any;
use wasm_bindgen::__rt::core::cell::{Cell, RefCell};
use wasm_bindgen::__rt::std::rc::Rc;
use wasm_bindgen_futures::spawn_local;

pub type RefStore = Rc<RefCell<dyn Store + 'static>>;
pub type PrimeNode = Rc<Prime>;

pub struct Prime {
    store: RefStore,
    transport: TransportWrapper,
    babel: TranslationLayer,
    subscriptions: RefCell<Vec<Box<dyn Fn(&Value)>>>,
}

#[doc(hidden)]
pub fn __build_prime_node(store: impl Store + 'static, babel: TranslationLayer) -> PrimeNode {
    let store = Rc::new(RefCell::new(store));

    let prime_node = Rc::new(Prime {
        store,
        subscriptions: RefCell::new(Vec::new()),
        transport: RefCell::new(None),
        babel,
    });

    let transport = Transport::new(prime_node.clone());
    prime_node.transport.replace(Some(transport));
    prime_node
}

impl Prime {
    pub fn get_state(&self) -> Value {
        self.store.borrow_mut().get_state()
    }

    pub fn dispatch(self: &Rc<Self>, action: impl Action + 'static) {
        let this = self.clone();
        let action = Box::new(action);
        spawn_local(async move {
            this.dispatch_internal(action).await;
        });
    }

    pub(crate) async fn dispatch_internal(&self, action: Box<dyn Any>) {
        // Check if action is valid
        if self.store.borrow_mut().dispatch(action).await {
            let state: Value = self.get_state();
            // Trigger subscriptions if they exist
            if self.subscriptions.borrow().len() > 0 {
                for subscription in self.subscriptions.borrow_mut().iter() {
                    subscription(&state);
                }
            }

            //let val = serde_json::from_value(test).unwrap();
            //let val = parse_value_to_type!(test, "String");

            // TODO ctx send new state
            self.transport.get().send(value_to_uint8array(&state));

            // TODO use deserialization in node
            //let val: Value = serde_json::from_slice(&serialized).unwrap();
            //log(&format!("Deserialized value: {:?}", &val));
        }
    }

    // TODO implement unsubscribe
    pub fn subscribe(&self, subscription: impl Fn(&Value) + 'static) {
        let subscription = Box::new(subscription);
        self.subscriptions.borrow_mut().push(subscription);
    }

    pub(crate) fn initialized(&self) -> bool {
        self.transport.get().initialized()
    }

    pub(crate) fn set_initialized(&self, initialized: bool) {
        self.transport.get().set_initialized(initialized);
    }

    pub(crate) fn send_state(&self) {
        self.transport
            .get()
            .send(value_to_uint8array(&self.get_state()));
    }

    pub(crate) fn value_to_action(
        &self,
        name: &str,
        val: Value,
    ) -> Result<Box<dyn Any>, BabelError> {
        self.babel.value_to_action(name, val)
    }
}
