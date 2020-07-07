use crate::action::Action;
use crate::log;
use serde::ser::Serialize;
use serde::Deserialize;
use serde_json::value::{Map, Value};
use wasm_bindgen::__rt::core::any::{Any, TypeId};

pub trait Store {
    fn get_state(&self) -> Value;

    fn dispatch(&mut self, action: Box<dyn Any>);
}

// TODO Param => Option<Rc<Param>>

pub fn build_single_store<
    State: Clone + Serialize + Deserialize<'static>,
    ActionEnum: Action + Clone,
    Param,
>(
    state: State,
    //actions: Vec<Box<dyn Action>>,
    reducer: fn(&State, ActionEnum, &Option<Param>) -> State,
    param: Option<Param>,
) -> SingleStore<State, ActionEnum, Param> {
    SingleStore {
        state,
        //actions,
        reducer,
        param,
    }
}

pub struct SingleStore<
    State: Clone + Serialize + Deserialize<'static>,
    ActionEnum: Action + Clone,
    Param,
> {
    state: State,
    //actions: Vec<Box<dyn Action>>,
    reducer: fn(&State, ActionEnum, &Option<Param>) -> State,
    param: Option<Param>,
}

impl<
        State: Clone + Serialize + Deserialize<'static>,
        ActionEnum: Action + Clone + 'static,
        Param,
    > Store for SingleStore<State, ActionEnum, Param>
{
    fn get_state(&self) -> Value {
        serde_json::to_value(self.state.clone()).unwrap()
    }

    fn dispatch(&mut self, action: Box<dyn Any>) {
        if let Some(kek) = action.downcast_ref::<ActionEnum>() {
            let new_state: State = (&self.reducer)(&self.state, kek.clone(), &self.param);
            self.state = new_state;
        }
    }
}

pub struct CombinedStore<
    State: Clone + Serialize + Deserialize<'static>,
    ActionEnum: Action + Clone,
    Param,
> {
    stores: Vec<(String, SingleStore<State, ActionEnum, Param>)>,
}

impl<State: Clone + Serialize + Deserialize<'static>, ActionEnum: Action + Clone, Param> Store
    for CombinedStore<State, ActionEnum, Param>
{
    fn get_state(&self) -> Value {
        let mut complete_state = Map::new();
        for store in self.stores.iter() {
            let this_state = serde_json::to_value(store.1.state.clone()).unwrap();
            complete_state.insert(store.0.clone(), this_state);
        }
        Value::from(complete_state)
    }

    fn dispatch(&mut self, action: Box<dyn Any>) {
        unimplemented!()
    }
}
