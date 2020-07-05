use serde_json::value::{Value, Map};
use serde::ser::Serialize;

pub trait Store {
    fn get_state(&self) -> Value;
}


struct SingleStore<'a, State: Copy + Serialize, Param> {
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



struct CombinedStore<'a, State: Copy + Serialize, Param>  {
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









