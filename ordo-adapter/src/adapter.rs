use crate::cache::Cache;
use crate::transport::{Transport, TransportWrapper, TransportWrapperMethods};
use wasm_bindgen::JsValue;
use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::rc::Rc;

pub(crate) type AdapterNode = Rc<Adapter>;

pub(crate) struct Adapter {
    store: Cache,
    transport: TransportWrapper,
}

impl Adapter {
    pub(crate) fn new() -> AdapterNode {
        let store = Cache::new();
        let transport = RefCell::new(None);

        let adapter_node = Rc::new(Adapter { store, transport });

        let transport = Transport::new(adapter_node.clone());
        adapter_node.transport.replace(Some(transport));
        adapter_node
    }

    pub(crate) fn get_state(&self) -> JsValue {
        self.store.get_state()
    }

    pub(crate) fn initialized(&self) -> bool {
        self.transport.get().initialized()
    }
}
