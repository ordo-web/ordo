#[macro_use]
mod macros;
pub mod action;
pub mod prime;
pub mod reducer;
pub mod store;
mod transport;
mod utils;

use crate::action::{Action, Babel};
use crate::prime::__build_prime_node;
use crate::store::__build_single_store;
use crate::utils::set_panic_hook;
use wasm_bindgen::__rt::std::rc::Rc;
use wasm_bindgen::prelude::*;

// Re-exports
pub use crate::prime::Prime;
pub use serde::{Deserialize, Serialize};
pub use serde_json::value::Value;

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

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
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
    reducer: fn(&State, ActionEnum, &Option<Param>) -> State,
    param: Option<Param>,
    babel: Babel,
) -> Rc<Prime> {
    set_panic_hook();
    let store = __build_single_store(state, reducer, param);
    __build_prime_node(store, babel)
}

// To debug macros use ` cargo rustc -- -Z external-macro-backtrace `
// The `-Z external-macro-backtrace` is deprecated
// See: https://github.com/rust-lang/rust/issues/49535

#[macro_export]
macro_rules! create_combined_store {
    ( $babel:expr, ($( $store: expr ),*) ) => {
        {
            let mut stores: Vec<Box<$crate::store::StoreUtility>> = Vec::new();
            $(
                stores.push($store);
            )*
            let combined_store = $crate::store::__build_combined_store(stores);
            // Assign type to annotate return value of macro
            let mut prime_node: $crate::prime::PrimeNode =
                $crate::prime::__build_prime_node(combined_store, $babel);
            prime_node
        }
    };
}

#[macro_export]
macro_rules! reducer {
    ( $name:expr, $state:expr, $reducer:expr, $param:expr ) => {{
        let tmp = $crate::store::__build_single_store($state, $reducer, $param);
        let store = Box::new((String::from($name), tmp));
        store
    }};
}

#[macro_export]
macro_rules! console_error {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (error(&format_args!($($t)*).to_string()))
}
