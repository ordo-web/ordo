use crate::action::Action;
use crate::reducer::Reducer;
use async_trait::async_trait;
use serde::ser::Serialize;
use serde::Deserialize;
use serde_json::value::{Map, Value};
use wasm_bindgen::__rt::core::any::Any;

// TODO Remove param and convert reducer fn to reducer Fn (Closure).
// This way values can be easily moved into (like via RC).

#[async_trait(?Send)]
pub trait Store {
    // Returns the current state.
    fn get_state(&self) -> Value;

    // Dispatches an Action which should result in called Reducers.
    async fn dispatch(&mut self, action: Box<dyn Any>) -> bool;
}

#[doc(hidden)]
pub fn __build_single_store<
    State: Clone + Serialize + Deserialize<'static>,
    ActionEnum: Action,
    Param,
>(
    state: State,
    //reducer: fn(&State, ActionEnum, &Option<Param>) -> State,
    reducer: Reducer<State, ActionEnum>,
    param: Option<Param>,
) -> SingleStore<State, ActionEnum, Param> {
    SingleStore {
        state,
        reducer,
        param,
    }
}

#[doc(hidden)]
pub fn __build_combined_store(stores: Vec<Box<dyn StoreUtility>>) -> CombinedStore {
    CombinedStore { stores }
}

pub struct SingleStore<State: Clone + Serialize + Deserialize<'static>, ActionEnum: Action, Param> {
    state: State,
    //reducer: fn(&State, ActionEnum, &Option<Param>) -> State,
    reducer: Reducer<State, ActionEnum>,
    param: Option<Param>,
}

impl<
        State: 'static + Clone + Serialize + Deserialize<'static>,
        ActionEnum: Action + Clone + 'static,
        Param,
    > SingleStore<State, ActionEnum, Param>
{
    async fn dispatch_internal(&mut self, action: &Box<dyn Any>) -> bool {
        if let Some(action) = action.downcast_ref::<ActionEnum>() {
            let new_state: State = self.reducer.call(self.state.clone(), action.clone()).await;
            //let new_state: State = (&self.reducer)(&self.state, action.clone(), &self.param);
            self.state = new_state;
            return true;
        }
        false
    }
}

#[async_trait(?Send)]
impl<
        State: 'static + Clone + Serialize + Deserialize<'static>,
        ActionEnum: Action + Clone + 'static,
        Param,
    > Store for SingleStore<State, ActionEnum, Param>
{
    fn get_state(&self) -> Value {
        serde_json::to_value(self.state.clone()).unwrap()
    }

    async fn dispatch(&mut self, action: Box<dyn Any>) -> bool {
        self.dispatch_internal(&action).await
    }
}

pub struct CombinedStore {
    stores: Vec<Box<dyn StoreUtility>>,
}

#[async_trait(?Send)]
impl Store for CombinedStore {
    fn get_state(&self) -> Value {
        let mut complete_state = Map::new();
        for store in self.stores.iter() {
            let result = store.serialize();
            complete_state.insert(result.0, result.1);
        }
        Value::from(complete_state)
    }

    async fn dispatch(&mut self, action: Box<dyn Any>) -> bool {
        let mut flag = false;
        for store in self.stores.iter_mut() {
            if store.dispatch_internal(&action).await && !flag {
                flag = true;
            }
        }
        flag
    }
}

// StoreUtility is used for the type of the combined stores.
// Using generics would not work for CombinedStore because every SingleStore can use a different
// concrete type.
// See: https://stackoverflow.com/a/40065342/12347616

#[doc(hidden)]
#[async_trait(?Send)]
pub trait StoreUtility {
    fn serialize(&self) -> (String, Value);

    async fn dispatch_internal(&mut self, action: &Box<dyn Any>) -> bool;
}

#[async_trait(?Send)]
impl<
        State: 'static + Clone + Serialize + Deserialize<'static>,
        ActionEnum: Action + Clone + 'static,
        Param: 'static,
    > StoreUtility for (String, SingleStore<State, ActionEnum, Param>)
{
    fn serialize(&self) -> (String, Value) {
        let state = serde_json::to_value(self.1.state.clone()).unwrap();
        (self.0.clone(), state)
    }

    async fn dispatch_internal(&mut self, action: &Box<dyn Any>) -> bool {
        self.1.dispatch_internal(&action).await
    }
}
