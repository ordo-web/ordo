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

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct TodoEntry {
    id: u32,
    content: String,
}

#[wasm_bindgen]
impl TodoEntry {
    #[wasm_bindgen(constructor)]
    pub fn new(id: u32, content: String) -> TodoEntry {
        TodoEntry { id, content }
    }
}

#[action]
enum TodoAction {
    AddTodo(TodoEntry),
    ToggleTodo(u32),
}

#[derive(Serialize, Deserialize, Clone)]
struct TodoInfo {
    content: String,
    completed: bool,
}

#[state]
struct TodoState {
    all_ids: Vec<u32>,
    by_ids: HashMap<u32, TodoInfo>,
}

#[action]
enum FilterAction {
    SetFilter(String),
}

#[state]
struct FilterState {
    filter: String,
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
            TodoAction,
            parse_TodoAction,
            FilterAction,
            parse_FilterAction
        );

        let todo_state = TodoState {
            all_ids: Vec::new(),
            by_ids: HashMap::new(),
        };

        let filter_state = FilterState {
            filter: String::from("all"),
        };

        let todo_reducer =
            Reducer::new(Box::new(
                move |state: TodoState, action: TodoAction| match action {
                    TodoAction::AddTodo(info) => {
                        let mut all_ids = state.all_ids.clone();
                        all_ids.push(info.id);
                        let mut by_ids = state.by_ids.clone();
                        by_ids.insert(
                            info.id,
                            TodoInfo {
                                content: info.content,
                                completed: false,
                            },
                        );
                        TodoState { all_ids, by_ids }
                    }
                    TodoAction::ToggleTodo(id) => {
                        let mut by_ids = state.by_ids.clone();
                        let mut entry = by_ids.remove(&id).unwrap();
                        entry.completed = !entry.completed;
                        by_ids.insert(id, entry);
                        TodoState { by_ids, ..state }
                    }
                },
            ));

        let filter_reducer = Reducer::new(Box::new(
            move |state: FilterState, action: FilterAction| match action {
                FilterAction::SetFilter(filter) => FilterState { filter },
            },
        ));

        let store: PrimeNode = ordo::create_combined_store!(
            translation,
            (
                ordo::config!("todos", todo_state, todo_reducer),
                ordo::config!("visibilityFilter", filter_state, filter_reducer)
            )
        );

        TodoApp { _ordo: store }
    }
}
