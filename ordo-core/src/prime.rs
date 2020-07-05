use crate::store::Store;
use serde_json::value::Value;

pub struct PrimeNode {
    store: Box<dyn Store + 'static>,
}

pub fn build_prime_node(store: impl Store + 'static) -> PrimeNode {
    PrimeNode {
        store: Box::new(store),
    }
}

impl PrimeNode {
    pub fn get_state(&self) -> Value {
        self.store.get_state()
    }
}
