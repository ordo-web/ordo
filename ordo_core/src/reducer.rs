use wasm_bindgen::prelude::*;

pub trait Reducer {}

pub struct WasmReducer<'a, State, Param> {
    state: State,
    reducer: fn(&'a State, String, Option<&'a Param>) -> State,
    param: Option<&'a Param>,
}

impl<'a, State, Param> Reducer for WasmReducer<'a, State, Param> {}

#[wasm_bindgen]
pub struct JsReducer {}

impl Reducer for JsReducer {}

//pub type Reducer<State, Param> = fn(&State, String, &Option<Param>) -> State;
