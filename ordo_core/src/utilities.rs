use js_sys::Uint8Array;
use serde_json::Value;

pub(crate) fn value_to_uint8array(val: &Value) -> Uint8Array {
    /*
    unsafe {
        // Does not work: TypeError: cannot transfer WebAssembly/asm.js ArrayBuffer
        // Get Transferable
        // See: https://github.com/rustwasm/wasm-bindgen/issues/1516
        /*
        let ser = Uint8Array::view(&serialized);
        let ser = ser.buffer();
        let res = self.ctx.post_message_with_transfer(&ser, &Array::of1(&ser));
        match res {
            Ok(_) => {
                log("OKKK");
            }
            Err(err) => {
                log(&format!("NOT OKKK: {:?}", err));
            }
        }*/
        let serialized = serde_json::to_vec(&val).unwrap();
        return Uint8Array::view(&serialized);
    }*/
    let mut serialized = serde_json::to_vec(val).unwrap();
    let length = serialized.len() as u32;
    let arr = Uint8Array::new_with_length(length);
    for i in 0..length {
        arr.fill(serialized.remove(0), i, length);
    }
    arr
}
