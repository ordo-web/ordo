use crate::adapter::AdapterNode;
use crate::log;
use js_sys::Uint8Array;
use wasm_bindgen::__rt::core::cell::{RefCell, Ref};
use wasm_bindgen::__rt::std::rc::Rc;
use wasm_bindgen::closure::Closure;
use web_sys::MessageEvent;
use web_sys::Worker;
use wasm_bindgen::{JsValue, JsCast};

pub(crate) struct Transport {
    node: AdapterNode,
    ctx: Rc<Worker>,
    initialized: RefCell<bool>,
    _onmessage: Closure<dyn FnMut(MessageEvent)>,
}

impl Transport {
    pub(crate) fn new(node: AdapterNode) -> Transport {
        let ctx = Rc::new(Worker::from(JsValue::from(js_sys::global())));
        // let _ = ctx.post_message(&JsValue::from("CTX here speaking"));

        let initialized = RefCell::new(false);

        let _onmessage = Transport::build_onmessage(node.clone(), ctx.clone());
        ctx.set_onmessage(Some(_onmessage.as_ref().unchecked_ref()));

        Transport {
            node,
            ctx,
            initialized,
            _onmessage
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

    fn build_onmessage(node: AdapterNode, ctx: Rc<Worker>) -> Closure<dyn FnMut(MessageEvent)> {
        Closure::wrap(Box::new(|event: MessageEvent| {
            let data: JsValue = event.data();
            console_log!("UI: Received data: {:?}", &data);
        }) as Box<dyn FnMut(MessageEvent)>)
    }

    pub(crate) fn initialized(&self) -> bool {
        self.initialized.borrow().clone()
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

