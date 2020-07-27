use crate::log;
use crate::utils::set_panic_hook;
use js_sys::Function;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::MessageEvent;
use web_sys::Worker;

#[wasm_bindgen]
pub struct Node {
    ctx: Worker,
    onmessage: Closure<dyn FnMut(MessageEvent)>,
}

impl Node {}

#[wasm_bindgen]
impl Node {
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: Worker) -> Node {
        set_panic_hook();

        // TODO build inner Node Wrapper with Rc, with State and Transport

        let cb = Closure::wrap(Box::new(|event: MessageEvent| {
            let data: JsValue = event.data();
            console_log!("Received data: {:?}", &data);
        }) as Box<dyn FnMut(MessageEvent)>);

        ctx.set_onmessage(Some(cb.as_ref().unchecked_ref()));

        Node { ctx, onmessage: cb }
    }

    pub fn dispatch(action: JsValue) {}

    #[wasm_bindgen(js_name = getState)]
    pub fn get_state() -> JsValue {
        JsValue::null()
    }

    pub fn subscribe(func: &Function) {}
}
