use serde_json::Value;

/**pub struct Action {
    pub name: String,
    pub payload: Value,
}

#[macro_export]
macro_rules! create_action {
    ( $name:expr, $payload:expr ) => {{
        let name = String::from($name);
        Action {
            name: name,
            payload: $payload,
        }
    }};
}*/

pub trait Action {}
