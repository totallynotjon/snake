use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SnakeGame {
    n : i16,
    m : i16,
    apple_add_cb: js_sys::Function,
}

#[wasm_bindgen]
impl SnakeGame {
    #[wasm_bindgen(constructor)]
    pub fn new(n : i16, m : i16, apple_add_cb: js_sys::Function) -> Self {
        Self {
            n : n,
            m : m,
            apple_add_cb: apple_add_cb,
        }
    }

    fn emit_apple_delta(&self, delta : AppleDelta) {
        let js_state = serde_wasm_bindgen::to_value(&delta).unwrap();
        self.apple_add_cb.call1(&JsValue::NULL, &js_state).ok();
    }
}



#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct AppleDelta {
    pub x: u32, //coordinates of the apple create/remove
    pub y: u32,
    pub exist : bool //if exist create, if no exist remove
}
