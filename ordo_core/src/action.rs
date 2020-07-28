use serde_json::Value;
use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::collections::HashMap;

pub trait Action {}

pub type ParseActionFn = fn(Value) -> Box<dyn Action>;

pub struct Babel {
    store: RefCell<HashMap<&'static str, ParseActionFn>>,
}

impl Babel {
    pub fn new(store: HashMap<&'static str, ParseActionFn>) -> Babel {
        Babel {
            store: RefCell::new(store),
        }
    }
}

#[macro_export]
macro_rules! babel {
    ( $( $action: ident, $func: expr ),* ) => {
        {
            let mut store: HashMap<&'static str, $crate::action::ParseActionFn> = HashMap::new();
            $(
                let _ = store.insert(stringify!($action), $func);
            )*
            $crate::action::Babel::new(store)
        }
    };
}
