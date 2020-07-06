#[macro_export]
macro_rules! create_action {
    ( $name:ident ) => {{
        pub enum $name {
            TYPE,
            PAYLOAD(Value),
        }
    }};
}
