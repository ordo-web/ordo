use crate::cache::Cache;
use crate::transport::{Transport, TransportWrapper, TransportWrapperMethods};
use js_sys::Function;
use serde_json::Value;
use wasm_bindgen::JsValue;
use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::rc::Rc;
use web_sys::Worker;

pub(crate) type AdapterNode = Rc<Adapter>;

pub(crate) struct Adapter {
    store: Cache,
    transport: TransportWrapper,
    subscriptions: RefCell<Vec<Box<Function>>>,
}

impl Adapter {
    pub(crate) fn new(ctx: Worker) -> AdapterNode {
        let store = Cache::new();
        let transport = RefCell::new(None);
        let subscriptions = RefCell::new(Vec::new());

        let adapter_node = Rc::new(Adapter {
            store,
            transport,
            subscriptions,
        });

        let transport = Transport::new(adapter_node.clone(), ctx);
        adapter_node.transport.replace(Some(transport));
        adapter_node
    }

    pub(crate) fn get_state(&self) -> JsValue {
        self.store.get_state()
    }

    pub(crate) fn update_state(&self, new_state: Value) {
        self.store.update_state(new_state);

        if self.subscriptions.borrow().len() > 0 {
            for subscription in self.subscriptions.borrow_mut().iter() {
                let _ = subscription.call0(&JsValue::null());
            }
        }
    }

    pub(crate) fn dispatch(&self, action: JsValue) {
        self.transport.get().send_value(action);
    }

    pub(crate) fn initialized(&self) -> bool {
        self.transport.get().initialized()
    }

    pub(crate) fn set_initialized(&self, initialized: bool) {
        self.transport.get().set_initialized(initialized);
    }

    pub(crate) fn subscribe(&self, subscription: Function) {
        let subscription = Box::new(subscription);
        self.subscriptions.borrow_mut().push(subscription);
    }

    pub(crate) fn send_value(&self, data: JsValue) {
        self.transport.get().send_value(data);
    }
}
