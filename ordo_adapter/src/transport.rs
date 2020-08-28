use crate::adapter::AdapterNode;
use crate::log;
use crate::utils::uint8array_to_value;
use js_sys::Uint8Array;
use wasm_bindgen::__rt::core::cell::{Ref, RefCell};
use wasm_bindgen::__rt::std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::MessageEvent;
use web_sys::Worker;

pub(crate) struct Transport {
    #[allow(dead_code)]
    node: AdapterNode,
    ctx: Rc<Worker>,
    initialized: RefCell<bool>,
    _onmessage: Closure<dyn FnMut(MessageEvent)>,
}

impl Transport {
    pub(crate) fn new(node: AdapterNode, ctx: Worker) -> Transport {
        let ctx = Rc::new(ctx);
        let _ = ctx.post_message(&JsValue::undefined());

        let initialized = RefCell::new(false);

        let _onmessage = Transport::build_onmessage(node.clone(), ctx.clone());
        ctx.set_onmessage(Some(_onmessage.as_ref().unchecked_ref()));

        Transport {
            node,
            ctx,
            initialized,
            _onmessage,
        }
    }

    // TODO delete?
    #[allow(dead_code)]
    pub(crate) fn send(&self, data: Uint8Array) {
        let res = self.ctx.post_message(&data);
        match res {
            Ok(_) => {}
            Err(err) => {
                console_log!("UI: Send-Error {:?}", err);
            }
        }
    }

    pub(crate) fn send_value(&self, data: JsValue) {
        // TODO parse to Uint8Array?
        // TODO check for fields name?

        let res = self.ctx.post_message(&data);
        match res {
            Ok(_) => {}
            Err(err) => {
                console_log!("UI: Send-Error {:?}", err);
            }
        }
    }

    fn build_onmessage(node: AdapterNode, _ctx: Rc<Worker>) -> Closure<dyn FnMut(MessageEvent)> {
        let node = node.clone();

        Closure::wrap(Box::new(move |event: MessageEvent| {
            let data: JsValue = event.data();
            console_log!("UI: Received data: {:?}", &data);

            if node.initialized() {
                // TODO update state and call subscriptions
                if data.has_type::<Uint8Array>() {
                    match uint8array_to_value(&data.unchecked_into::<Uint8Array>()) {
                        Ok(state) => {
                            // Update state and call subscriptions
                            node.update_state(state);
                        }
                        Err(_) => {
                            console_log!("UI: Received unsupported data...");
                        }
                    }
                }
            } else {
                if data.has_type::<Uint8Array>() {
                    match uint8array_to_value(&data.unchecked_into::<Uint8Array>()) {
                        Ok(state) => {
                            node.update_state(state);
                            node.set_initialized(true);
                            node.send_value(JsValue::null());
                            console_log!("UI: Initialized!");
                        }
                        Err(_) => {
                            node.send_value(JsValue::undefined());
                            console_log!("UI: Initializing...");
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>)
    }

    pub(crate) fn initialized(&self) -> bool {
        self.initialized.borrow().clone()
    }

    pub(crate) fn set_initialized(&self, initialized: bool) {
        self.initialized.replace(initialized);
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
