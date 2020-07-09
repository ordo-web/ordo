use crate::action::Action;
use crate::store::Store;
use serde_json::value::Value;

pub struct PrimeNode {
    store: Box<dyn Store + 'static>,
    subscriptions: Vec<Box<dyn Fn(&Value)>>,
}

#[doc(hidden)]
pub fn __build_prime_node(store: impl Store + 'static) -> PrimeNode {
    PrimeNode {
        store: Box::new(store),
        subscriptions: Vec::new(),
    }
}

impl PrimeNode {
    pub fn get_state(&self) -> Value {
        self.store.get_state()
    }

    pub fn dispatch(&mut self, action: impl Action + 'static) {
        let action = Box::new(action);
        if self.store.dispatch(action) && self.subscriptions.len() > 0 {
            let state: Value = self.get_state();
            for subscription in self.subscriptions.iter() {
                subscription(&state);
            }
        }
    }

    pub fn subscribe(&mut self, subscription: impl Fn(&Value) + 'static) {
        let subscription = Box::new(subscription);
        self.subscriptions.push(subscription);
    }
}
