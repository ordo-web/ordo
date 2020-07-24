use crate::log;
use crate::PrimeNode;
use wasm_bindgen::__rt::std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use web_sys::MessageEvent;
use web_sys::Worker;

pub(crate) struct Transport {
    node: Rc<PrimeNode>,
    ctx: Worker,
    _onmessage: Closure<dyn FnMut(MessageEvent)>,
}

impl Transport {
    pub(crate) fn new(node: Rc<PrimeNode>) -> Transport {
        let ctx = Worker::from(JsValue::from(js_sys::global()));
        let _ = ctx.post_message(&JsValue::from("CTX here speaking")); // TODO remove later

        let cb = Closure::wrap(Box::new(|event: MessageEvent| {
            let data: JsValue = event.data();
            console_log!("Received data: {:?}", &data);
        }) as Box<dyn FnMut(MessageEvent)>);

        Transport {
            node,
            ctx,
            _onmessage: cb,
        }
    }
}
