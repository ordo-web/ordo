use crate::cache::Cache;
use crate::transport::{Transport, TransportWrapper, TransportWrapperMethods};
use serde_json::Value;
use wasm_bindgen::JsValue;
use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::rc::Rc;
use web_sys::Worker;

pub(crate) type AdapterNode = Rc<Adapter>;

pub(crate) struct Adapter {
    store: Cache,
    transport: TransportWrapper,
}

impl Adapter {
    pub(crate) fn new(ctx: Worker) -> AdapterNode {
        let store = Cache::new();
        let transport = RefCell::new(None);

        let adapter_node = Rc::new(Adapter { store, transport });

        let transport = Transport::new(adapter_node.clone(), ctx);
        adapter_node.transport.replace(Some(transport));
        adapter_node
    }

    pub(crate) fn get_state(&self) -> JsValue {
        self.store.get_state()
    }

    pub(crate) fn initialized(&self) -> bool {
        self.transport.get().initialized()
    }

    pub(crate) fn set_initialized(&self, initialized: bool) {
        self.transport.get().set_initialized(initialized);
    }

    pub(crate) fn update_state(&self, new_state: Value) {
        self.store.update_state(new_state);
    }

    pub(crate) fn send_value(&self, data: JsValue) {
        self.transport.get().send_value(data);
    }
}
