mod utils;

use crate::utils::set_panic_hook;
use ordo;
use ordo::action::*;
use ordo::babel;
use ordo::console_error;
use ordo::error;
use ordo_derive::{action, state, Action};
use serde::Deserialize;
use serde::Serialize;

use ordo::prime::PrimeNode;
use serde_json::Value;
use wasm_bindgen::__rt::core::any::Any;
use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::collections::HashMap;
use wasm_bindgen::__rt::std::rc::Rc;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn logging() {
    ordo::hi();
    log("Hello, bin!");
}

#[state]
pub struct Testo {
    counter: u8,
}

#[action]
enum SomeTest {
    INCREMENT,
    DECREMENT,
}

#[action]
pub enum MyAction {
    INCREMENT(String),
    DECREMENT,
}

fn baum(state: &Testo, action: MyAction, param: &Option<Rc<u64>>) -> Testo {
    log(&format!("STATE: {}", &state.counter));
    match action {
        MyAction::INCREMENT(some) => {
            log(&format!("INCREMENT: {}", &some));
        }
        MyAction::DECREMENT => log("DECREMENT"),
    }

    let param = param.as_ref().unwrap();
    log(&format!("PARAM: {}", &param));
    //Testo { ..*state }
    Testo {
        counter: state.counter + 1,
    }
}

#[action]
pub enum MyAction2 {
    INCREMENT(String),
    DECREMENT,
}

fn baum2(state: &Testo, action: MyAction2, param: &Option<Rc<u64>>) -> Testo {
    log(&format!("STATE2: {}", &state.counter));
    match action {
        MyAction2::INCREMENT(some) => {
            log(&format!("INCREMENT2: {}", &some));
        }
        MyAction2::DECREMENT => log("DECREMENT2"),
    }

    let param = param.as_ref().unwrap();
    log(&format!("PARAM:2 {}", &param));
    Testo {
        counter: state.counter + 1,
    }
}

#[wasm_bindgen]
pub struct MyApp {
    ordo: PrimeNode,
}

#[wasm_bindgen]
impl MyApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MyApp {
        set_panic_hook();

        let babel = babel!(MyAction2, parse_MyAction2, MyAction, parse_MyAction);

        let param = Rc::new(10);

        let testo2 = Testo { counter: 10 };
        let testo3 = Testo { counter: 100 };
        let node = ordo::create_combined_store!(
            babel,
            (
                ordo::reducer!("test2", testo2, baum, Some(param.clone())),
                ordo::reducer!("test3", testo3, baum2, Some(param.clone()))
            )
        );
        /*
        node.dispatch(MyAction::INCREMENT(String::from("INC")));
        node.dispatch(MyAction2::INCREMENT(String::from("INC2")));

        let moved_value = 2;
        node.subscribe(move |state: &Value| {
            log(&format!("Subscription Invocation | State: {:?}", &state));
            log(&format!("Some moved value: {}", &moved_value));
        });

        node.dispatch(MyAction2::INCREMENT(String::from("INC2")));
        */
        //let val = node.get_state();
        //log(&format!("VAL: {:?}", &val));

        MyApp { ordo: node }
    }
}

#[wasm_bindgen]
pub fn test() {
    let val = serde_json::to_value(MyAction2::INCREMENT(String::from("Baum"))).unwrap();
    let val2 = serde_json::to_value(MyAction2::DECREMENT).unwrap();
    log(&format!("VAL: {:?}", &val));
    log(&format!("VAL2: {:?}", &val2));

    let babel = babel!(
        MyAction2,
        parse_MyAction2,
        MyAction,
        parse_MyAction,
        SomeTest,
        parse_SomeTest
    );

    //let wut = serde_json::from_value(val).unwrap();
    //let val: Box<dyn Any> = Box::new(wut);
    //let kek: MyAction2 = __funcMyAction2(String::from("INCREMENT"), Some(Box::new(String::from("kek"))));
    //log(&format!("VAL: {:?}", &kek));
    //let val = check(val);
    //let kek: MyAction2 = __parseMyAction2(String::from("INCREMENT"), Some(val));

    let param = Rc::new(10);

    let testo = Testo { counter: 0 };
    let node = ordo::create_store(testo, baum, Some(param.clone()), babel);
    let val = node.get_state();
    log(&format!("VAL: {:?}", &val));

    // This dispatch works because the correct enum is used
    node.dispatch(MyAction::INCREMENT(String::from("INC")));
    node.dispatch(MyAction::INCREMENT(String::from("INC")));
    // This dispatch does not work because the incorrect enum is used
    node.dispatch(SomeTest::INCREMENT);

    let babel = babel!(
        MyAction2,
        parse_MyAction2,
        MyAction,
        parse_MyAction,
        SomeTest,
        parse_SomeTest
    );

    let testo2 = Testo { counter: 10 };
    let testo3 = Testo { counter: 100 };
    let node2 = ordo::create_combined_store!(
        babel,
        (
            ordo::reducer!("test2", testo2, baum, Some(param.clone())),
            ordo::reducer!("test3", testo3, baum2, Some(param.clone()))
        )
    );
    node2.dispatch(MyAction::INCREMENT(String::from("INC")));
    node2.dispatch(MyAction2::INCREMENT(String::from("INC2")));

    let moved_value = 2;
    node2.subscribe(move |state: &Value| {
        log(&format!("Subscription Invocation | State: {:?}", &state));
        log(&format!("Some moved value: {}", &moved_value));
    });

    node2.dispatch(MyAction2::INCREMENT(String::from("INC2")));

    let val = node2.get_state();
    log(&format!("VAL: {:?}", &val));
}

pub fn kekkkk(name: String, payload: Option<Box<dyn Any>>) -> MyAction2 {
    let flag = payload.is_some();
    let payload = payload.unwrap();
    if name.as_str() == "INCREMENT" && flag && payload.is::<String>() {
        MyAction2::INCREMENT(*payload.downcast::<String>().unwrap())
    } else {
        MyAction2::DECREMENT
    }
}

fn check(val: Value) -> Box<dyn Any> {
    if val.is_string() {
        Box::new(String::from(val.as_str().unwrap()))
    } else if val.is_boolean() {
        Box::new(val.as_bool().unwrap())
    } else {
        panic!("Unsupported type!")
    }
}
