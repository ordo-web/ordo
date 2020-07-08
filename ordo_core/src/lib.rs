pub mod action;
mod node;
pub mod prime;
mod reducer;
pub mod store;
mod utils;

use wasm_bindgen::prelude::*;
use web_sys;

use crate::action::Action;
use crate::prime::build_prime_node;
use crate::store::__build_single_store;
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
    State: 'static + Clone + Serialize + Deserialize<'static>,
    ActionEnum: 'static + Action + Clone,
    Param: 'static,
>(
    state: State,
    //actions: Vec<String>,
    reducer: fn(&State, ActionEnum, &Option<Param>) -> State,
    param: Option<Param>,
) -> PrimeNode {
    let store = __build_single_store(state, reducer, param);
    build_prime_node(store)
}

// To debug macros use ` cargo rustc -- -Z external-macro-backtrace `
// The `-Z external-macro-backtrace` is deprecated
// See: https://github.com/rust-lang/rust/issues/49535

#[macro_export]
macro_rules! create_combined_store {
    ( $( $store: expr ),* ) => {
        {
            let mut stores = Vec::new();
            $(
                stores.push($store);
            )*
            let combined_store = $crate::store::__build_combined_store(stores);
            // Assign type to annotate return value of macro
            let mut prime_node: $crate::prime::PrimeNode =
                $crate::prime::build_prime_node(combined_store);
            prime_node
        }
    };
}

#[macro_export]
macro_rules! reducer {
    ( $name:expr, $state:expr, $reducer:expr, $param:expr ) => {{
        let tmp = $crate::store::__build_single_store($state, $reducer, $param);
        let store = (String::from($name), tmp);
        store
    }};
}

// Re-exports
pub use crate::prime::PrimeNode;
use serde::Deserialize;
pub use serde_json::value::Value;
