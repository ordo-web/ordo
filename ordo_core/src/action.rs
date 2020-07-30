use serde_json::Value;
use wasm_bindgen::__rt::core::any::Any;
use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::collections::HashMap;

pub trait Action {}

pub type ParseActionFn = fn(Value) -> Result<Box<dyn Any>, ()>;

pub struct Babel {
    store: RefCell<HashMap<&'static str, ParseActionFn>>,
}

#[derive(PartialEq)]
pub(crate) enum BabelError {
    ConversionFailed,
    MissingFunc,
}

impl Babel {
    pub fn new(store: HashMap<&'static str, ParseActionFn>) -> Babel {
        Babel {
            store: RefCell::new(store),
        }
    }

    pub(crate) fn value_to_action(
        &self,
        name: &str,
        val: Value,
    ) -> Result<Box<dyn Any>, BabelError> {
        match self.store.borrow().get(name) {
            Some(func) => match (*func)(val) {
                Ok(res) => Ok(res),
                Err(_) => Err(BabelError::ConversionFailed),
            },
            None => Err(BabelError::MissingFunc),
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
