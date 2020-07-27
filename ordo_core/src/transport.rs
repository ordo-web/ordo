use crate::log;
use crate::prime::PrimeNode;
use js_sys::Uint8Array;
use wasm_bindgen::__rt::std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use wasm_bindgen::__rt::core::cell::{Ref, RefCell};
use web_sys::MessageEvent;
use web_sys::Worker;

pub(crate) struct Transport {
    node: PrimeNode,
    ctx: Rc<Worker>,
    initialized: RefCell<bool>,
    _onmessage: Closure<dyn FnMut(MessageEvent)>,
}

impl Transport {
    pub(crate) fn new(node: PrimeNode) -> Transport {
        let ctx = Rc::new(Worker::from(JsValue::from(js_sys::global())));
        let _ = ctx.post_message(&JsValue::from("CTX here speaking")); // TODO remove later

        let initialized = RefCell::new(false);

        let _onmessage = Transport::build_onmessage(node.clone(), ctx.clone());

        Transport {
            node,
            ctx,
            initialized,
            _onmessage,
        }
    }

    pub(crate) fn send(&self, data: Uint8Array) {
        let res = self.ctx.post_message(&data);
        match res {
            Ok(_) => {
                console_log!("OKKK");
            }
            Err(err) => {
                console_log!("NOT OKKK: {:?}", err);
            }
        }
    }

    fn build_onmessage(node: PrimeNode, ctx: Rc<Worker>) -> Closure<dyn FnMut(MessageEvent)> {
        Closure::wrap(Box::new(|event: MessageEvent| {
            let data: JsValue = event.data();
            console_log!("Received data: {:?}", &data);
        }) as Box<dyn FnMut(MessageEvent)>)
    }
}

pub(crate) type TransportWrapper = RefCell<Option<Transport>>;

pub(crate) trait TransportWrapperMethods {
    fn get(&self) -> Ref<Transport>;
}

impl TransportWrapperMethods for TransportWrapper {
    fn get(&self) -> Ref<Transport> {
        Ref::map(self.borrow(), |transport| transport.as_ref().unwrap())
    }
}
