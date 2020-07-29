use js_sys::Uint8Array;
use serde_json::Value;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn uint8array_to_value(val: &Uint8Array) -> Result<Value, ()> {
    let mut deserialized: Vec<u8> = vec![0; val.length() as usize];
    val.copy_to(&mut deserialized[..]);
    match serde_json::from_slice::<Value>(&*deserialized) {
        Ok(value) => Ok(value),
        Err(_) => Err(()),
    }
}
