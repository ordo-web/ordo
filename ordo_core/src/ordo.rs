use crate::action::Action;
use crate::prime::{build_prime_node, PrimeNode};
use crate::store::{build_single_store, Store};
use serde::ser::Serialize;

pub struct Ordo {}

impl Ordo {
    pub fn create_store<
        State: 'static + Copy + Serialize,
        ActionEnum: 'static + Action,
        Param: 'static,
    >(
        state: State,
        //actions: Vec<String>,
        reducer: fn(&State, ActionEnum, Option<&Param>) -> State,
        param: std::option::Option<&'static Param>,
    ) -> PrimeNode {
        let store = build_single_store(state, reducer, param);
        build_prime_node(store)
    }
}
