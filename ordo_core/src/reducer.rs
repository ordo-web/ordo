use crate::action::Action;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use wasm_bindgen::__rt::core::future::Future;
use wasm_bindgen::__rt::core::pin::Pin;

/**
pub(crate) trait ReducerTrait<State: Clone + Serialize + Deserialize<'static>, ActionEnum: Action> {
    fn call(&self, state: State, action: ActionEnum) -> Pin<Box<dyn Future<Output = State> + '_>>;
}

// Source: https://users.rust-lang.org/t/solved-is-it-possible-to-run-async-code-in-a-trait-method-with-stdfuture-async-await/24874/3
impl<State: Clone + Serialize + Deserialize<'static>, ActionEnum: Action>
    ReducerTrait<State, ActionEnum> for dyn Fn(State, ActionEnum) -> State
{
    fn call(&self, state: State, action: ActionEnum) -> Pin<Box<dyn Future<Output = State> + '_>> {
        Box::pin(async move { (self)(state, action) })
    }
}

// Source: https://users.rust-lang.org/t/solved-is-it-possible-to-run-async-code-in-a-trait-method-with-stdfuture-async-await/24874/3
impl<State: Clone + Serialize + Deserialize<'static>, ActionEnum: Action>
    ReducerTrait<State, ActionEnum>
    for dyn Fn(State, ActionEnum) -> Pin<Box<dyn Future<Output = State>>>
{
    fn call(&self, state: State, action: ActionEnum) -> Pin<Box<dyn Future<Output = State> + '_>> {
        Box::pin(async move { (self)(state, action).await })
    }
}

pub struct Reducer<State: Clone + Serialize + Deserialize<'static>, ActionEnum: Action> {
    func: Box<dyn ReducerTrait<State, ActionEnum>>,
}

impl<State: Clone + Serialize + Deserialize<'static>, ActionEnum: Action>
    Reducer<State, ActionEnum>
{
    pub fn new(func: impl ReducerTrait<State, ActionEnum> + 'static) -> Reducer<State, ActionEnum> {
        Reducer {
            func: Box::new(func),
        }
    }

    pub(crate) async fn call(&self, state: State, action: ActionEnum) -> State {
        self.func.call(state, action).await
    }
}*/

#[async_trait(?Send)]
pub trait ReducerFunc<State: Clone + Serialize + Deserialize<'static>, ActionEnum: Action> {
    async fn call(&self, state: State, action: ActionEnum) -> State;
}

#[async_trait(?Send)]
impl<State: Clone + Serialize + Deserialize<'static>, ActionEnum: Action>
    ReducerFunc<State, ActionEnum> for Box<dyn Fn(State, ActionEnum) -> State>
{
    async fn call(&self, state: State, action: ActionEnum) -> State {
        (self)(state, action)
    }
}

#[async_trait(?Send)]
impl<State: Clone + Serialize + Deserialize<'static>, ActionEnum: Action>
    ReducerFunc<State, ActionEnum>
    for Box<dyn Fn(State, ActionEnum) -> Box<dyn Future<Output = State>>>
{
    async fn call(&self, state: State, action: ActionEnum) -> State {
        // Pin help: https://stackoverflow.com/a/58357166/12347616
        Pin::from((self)(state, action)).await
    }
}

pub struct Reducer<State: Clone + Serialize + Deserialize<'static>, ActionEnum: Action> {
    func: Box<dyn ReducerFunc<State, ActionEnum>>,
}

impl<State: 'static + Clone + Serialize + Deserialize<'static>, ActionEnum: 'static + Action>
    Reducer<State, ActionEnum>
{
    pub fn new(func: Box<dyn Fn(State, ActionEnum) -> State>) -> Reducer<State, ActionEnum> {
        Reducer {
            func: Box::new(func),
        }
    }

    pub fn new_async(
        func: Box<dyn Fn(State, ActionEnum) -> Box<dyn Future<Output = State>>>,
    ) -> Reducer<State, ActionEnum> {
        Reducer {
            func: Box::new(func),
        }
    }

    pub(crate) async fn call(&self, state: State, action: ActionEnum) -> State {
        self.func.call(state, action).await
    }
}
