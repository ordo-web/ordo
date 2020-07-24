use crate::PrimeNode;
use wasm_bindgen::__rt::std::rc::Rc;

pub(crate) struct Transport {
    node: Rc<PrimeNode>,
}

impl Transport {
    pub(crate) fn new(node: Rc<PrimeNode>) -> Transport {
        Transport { node }
    }
}
