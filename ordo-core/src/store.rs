use serde::ser::Serialize;
use serde_json::value::{Map, Value};

pub trait Store {
    fn get_state(&self) -> Value;
}

pub fn build_single_store<State: Copy + Serialize, Param>(
    state: State,
    actions: Vec<String>,
    reducer: fn(&State, String, Option<&Param>) -> State,
    param: Option<&Param>,
) -> SingleStore<State, Param> {
    SingleStore {
        state,
        actions,
        reducer,
        param,
    }
}

pub struct SingleStore<'a, State: Copy + Serialize, Param> {
    state: State,
    actions: Vec<String>,
    reducer: fn(&'a State, String, Option<&'a Param>) -> State,
    param: Option<&'a Param>,
}

impl<'a, State: Copy + Serialize, Param> Store for SingleStore<'a, State, Param> {
    fn get_state(&self) -> Value {
        serde_json::to_value(self.state.clone()).unwrap()
    }
}

pub struct CombinedStore<'a, State: Copy + Serialize, Param> {
    stores: Vec<(String, SingleStore<'a, State, Param>)>,
}

impl<'a, State: Copy + Serialize, Param> Store for CombinedStore<'a, State, Param> {
    fn get_state(&self) -> Value {
        let mut complete_state = Map::new();
        for store in self.stores.iter() {
            let this_state = serde_json::to_value(store.1.state.clone()).unwrap();
            complete_state.insert(store.0.clone(), this_state);
        }
        Value::from(complete_state)
    }
}
