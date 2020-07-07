pub mod action;
mod node;
mod prime;
mod reducer;
mod store;
mod utils;

use wasm_bindgen::prelude::*;
use web_sys;

use crate::action::Action;
use crate::prime::{build_prime_node, PrimeNode};
use crate::store::{build_single_store, Store};
use serde::ser::Serialize;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_use]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn hi() {
    console_log!("hi!");
}

pub fn create_store<
    State: 'static + Copy + Serialize,
    ActionEnum: 'static + Action + Copy,
    Param: 'static,
>(
    state: State,
    //actions: Vec<String>,
    reducer: fn(&State, ActionEnum, Option<&Param>) -> State,
    param: std::option::Option<Param>,
) -> PrimeNode {
    let store = build_single_store(state, reducer, param);
    build_prime_node(store)
}

// Re-exports
pub use serde_json::value::Value;
