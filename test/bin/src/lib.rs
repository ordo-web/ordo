mod utils;

use crate::utils::set_panic_hook;
use ordo;
use ordo::action::*;
use ordo::connect;
use ordo::console_error;
use ordo::error;
use ordo_derive::{action, state, Action};
use serde::Deserialize;
use serde::Serialize;

use ordo::prime::PrimeNode;
use ordo::reducer::Reducer;
use serde_json::Value;
use wasm_bindgen::__rt::core::any::Any;
use wasm_bindgen::__rt::std::collections::HashMap;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[state]
struct CounterState {
    counter: u8,
}

#[action]
enum CounterAction {
    INCREMENT,
    DECREMENT,
}

#[wasm_bindgen]
pub struct SingleStoreExample {
    _ordo: PrimeNode,
}

#[wasm_bindgen]
impl SingleStoreExample {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SingleStoreExample {
        set_panic_hook();

        // Note: The `parse_[action_name]` functions are automatically generated through the
        // #[action] macro.
        let translation = connect!(CounterAction, parse_CounterAction);

        let state = CounterState { counter: 10 };

        let reducer = Reducer::new(Box::new(
            move |state: CounterState, action: CounterAction| match action {
                CounterAction::INCREMENT => CounterState {
                    counter: state.counter + 1,
                },
                CounterAction::DECREMENT => CounterState {
                    counter: state.counter - 1,
                },
            },
        ));

        let store: PrimeNode = ordo::create_store(state, reducer, translation);

        SingleStoreExample { _ordo: store }
    }
}
