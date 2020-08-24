use crate::action::Action;
use serde::{Deserialize, Serialize};

pub trait Reducer<State: Clone + Serialize + Deserialize<'static>, ActionEnum: Action> {}

impl<State: Clone + Serialize + Deserialize<'static>, ActionEnum: Action> Reducer<State, ActionEnum>
    for dyn Fn(&State, ActionEnum)
{
}
