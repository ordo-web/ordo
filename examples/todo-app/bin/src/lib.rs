mod utils;

use crate::utils::set_panic_hook;
use ordo;
use ordo::action::*;
use ordo::connect;
use ordo::console_error;
use ordo::derive::{action, state, Action};
use ordo::error;
use serde::Deserialize;
use serde::Serialize;

use js_sys::Promise;
use ordo::prime::PrimeNode;
use ordo::reducer::Reducer;
use serde_json::Value;
use wasm_bindgen::__rt::core::any::Any;
use wasm_bindgen::__rt::std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};

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

#[wasm_bindgen(
    inline_js = "export function sleep(ms) { return new Promise((resolve)=> setTimeout(resolve, ms)); }"
)]
extern "C" {
    fn sleep(ms: f64) -> Promise;
}

#[derive(Serialize, Deserialize, Clone)]
struct TodoInfo {
    id: String,
    content: String,
}

#[action]
enum TodoAction {
    AddTodo(TodoInfo),
    ToggleTodo(String),
}

#[wasm_bindgen]
pub struct TodoApp {
    _ordo: PrimeNode,
}

#[wasm_bindgen]
impl TodoApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TodoApp {
        set_panic_hook();

        // Note: The `parse_[action_name]` functions are automatically generated through the
        // #[action] macro.
        let translation = connect!(
            CounterAction,
            parse_CounterAction,
            TodoAction,
            parse_TodoAction
        );

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

        TodoApp { _ordo: store }
    }
}

// Single Store

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

#[state]
struct TextState {
    text: String,
}

#[action]
enum TextAction {
    REPLACE(String),
    RESET,
}

#[wasm_bindgen]
pub struct SingleStoreAsyncExample {
    _ordo: PrimeNode,
}

#[wasm_bindgen]
impl SingleStoreAsyncExample {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SingleStoreAsyncExample {
        set_panic_hook();

        // Note: The `parse_[action_name]` functions are automatically generated through the
        // #[action] macro.
        let translation = connect!(TextAction, parse_TextAction);

        let state = TextState {
            text: String::from("Hello!"),
        };

        // Glue code is needed for async block
        // https://www.reddit.com/r/rust/comments/drtxbt/question_how_to_put_async_fn_into_a_map/f6lb4wt?utm_source=share&utm_medium=web2x&context=3
        let reducer = Reducer::new_async(Box::new(move |state: TextState, action: TextAction| {
            Box::new(async move {
                let _ = JsFuture::from(sleep(5.0)).await;
                match action {
                    TextAction::REPLACE(replacement) => TextState { text: replacement },
                    TextAction::RESET => TextState {
                        text: String::from("Hello!"),
                    },
                }
            })
        }));

        let store: PrimeNode = ordo::create_store(state, reducer, translation);

        SingleStoreAsyncExample { _ordo: store }
    }

    #[wasm_bindgen(js_name = testDispatch)]
    pub fn test_dispatch(&self) {
        let ordo = self._ordo.clone();
        spawn_local(async move {
            let _ = JsFuture::from(sleep(1500.0)).await;
            ordo.dispatch(TextAction::REPLACE(String::from("Hello World!")));
            let _ = JsFuture::from(sleep(500.0)).await;
            ordo.dispatch(TextAction::RESET);
        });
    }
}

// Combined store

#[state]
struct VecState {
    vec: Vec<u32>,
}

#[action]
enum VecAction {
    PUSH(u32),
    POP,
}

#[state]
struct StructState {
    number: SomeFloat,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
struct SomeFloat {
    number: f32,
}

#[action]
enum FloatAction {
    MULTIPLY(f32),
    DIVIDE(f32),
}

#[wasm_bindgen]
pub struct CombinedStoreExample {
    _ordo: PrimeNode,
}

#[wasm_bindgen]
impl CombinedStoreExample {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CombinedStoreExample {
        set_panic_hook();

        // Note: The `parse_[action_name]` functions are automatically generated through the
        // #[action] macro.
        let translation = connect!(VecAction, parse_VecAction, FloatAction, parse_FloatAction);

        let vec_state = VecState { vec: Vec::new() };
        let struct_state = StructState {
            number: SomeFloat { number: 100.0 },
        };

        let vec_reducer =
            Reducer::new(Box::new(
                move |state: VecState, action: VecAction| match action {
                    VecAction::PUSH(number) => {
                        let mut vec = state.vec.clone();
                        vec.push(number);
                        VecState { vec }
                    }
                    VecAction::POP => {
                        let mut vec = state.vec.clone();
                        vec.pop();
                        VecState { vec }
                    }
                },
            ));

        let struct_reducer = Reducer::new(Box::new(
            move |state: StructState, action: FloatAction| match action {
                FloatAction::MULTIPLY(number) => StructState {
                    number: SomeFloat {
                        number: state.number.number * number,
                    },
                },
                FloatAction::DIVIDE(number) => StructState {
                    number: SomeFloat {
                        number: state.number.number / number,
                    },
                },
            },
        ));

        let store: PrimeNode = ordo::create_combined_store!(
            translation,
            (
                ordo::config!("vecState", vec_state, vec_reducer),
                ordo::config!("structState", struct_state, struct_reducer)
            )
        );

        CombinedStoreExample { _ordo: store }
    }
}

#[wasm_bindgen]
pub struct CombinedStoreAsyncExample {
    _ordo: PrimeNode,
}

#[wasm_bindgen]
impl CombinedStoreAsyncExample {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CombinedStoreAsyncExample {
        set_panic_hook();

        // Note: The `parse_[action_name]` functions are automatically generated through the
        // #[action] macro.
        let translation = connect!(VecAction, parse_VecAction, FloatAction, parse_FloatAction);

        let vec_state = VecState { vec: Vec::new() };
        let struct_state = StructState {
            number: SomeFloat { number: 100.0 },
        };

        let vec_reducer =
            Reducer::new_async(Box::new(move |state: VecState, action: VecAction| {
                Box::new(async move {
                    let _ = JsFuture::from(sleep(1.0)).await;
                    match action {
                        VecAction::PUSH(number) => {
                            let mut vec = state.vec.clone();
                            vec.push(number);
                            VecState { vec }
                        }
                        VecAction::POP => {
                            let mut vec = state.vec.clone();
                            vec.pop();
                            VecState { vec }
                        }
                    }
                })
            }));

        let struct_reducer =
            Reducer::new_async(Box::new(move |state: StructState, action: FloatAction| {
                Box::new(async move {
                    let _ = JsFuture::from(sleep(1.0)).await;
                    match action {
                        FloatAction::MULTIPLY(number) => StructState {
                            number: SomeFloat {
                                number: state.number.number * number,
                            },
                        },
                        FloatAction::DIVIDE(number) => StructState {
                            number: SomeFloat {
                                number: state.number.number / number,
                            },
                        },
                    }
                })
            }));

        let store: PrimeNode = ordo::create_combined_store!(
            translation,
            (
                ordo::config!("vecState", vec_state, vec_reducer),
                ordo::config!("structState", struct_state, struct_reducer)
            )
        );

        CombinedStoreAsyncExample { _ordo: store }
    }

    #[wasm_bindgen(js_name = testDispatch)]
    pub fn test_dispatch(&self) {
        let ordo = self._ordo.clone();
        spawn_local(async move {
            let _ = JsFuture::from(sleep(1500.0)).await;
            ordo.dispatch(VecAction::PUSH(10));
            let _ = JsFuture::from(sleep(500.0)).await;
            ordo.dispatch(VecAction::POP);
            let _ = JsFuture::from(sleep(500.0)).await;
            ordo.dispatch(FloatAction::MULTIPLY(10.0));
            let _ = JsFuture::from(sleep(500.0)).await;
            ordo.dispatch(FloatAction::DIVIDE(10.0));
        });
    }
}
