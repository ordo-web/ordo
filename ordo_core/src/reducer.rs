use crate::action::Action;
use serde::{Deserialize, Serialize};
use wasm_bindgen::__rt::core::future::Future;
use wasm_bindgen::__rt::core::pin::Pin;

pub trait Reducer<State: Clone + Serialize + Deserialize<'static>, ActionEnum: Action> {
    fn call(&self, state: State, action: ActionEnum) -> Pin<Box<dyn Future<Output = State> + '_>>;
}

impl<State: Clone + Serialize + Deserialize<'static>, ActionEnum: Action> Reducer<State, ActionEnum>
    for dyn Fn(State, ActionEnum) -> State
{
    fn call(&self, state: State, action: ActionEnum) -> Pin<Box<dyn Future<Output = State> + '_>> {
        Box::pin(async move { (self)(state, action) })
    }
}
