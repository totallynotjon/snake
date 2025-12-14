mod apple;
pub mod events;
mod snake;

use crate::game::events::apple::AppleDelta;
use crate::prelude::*;
use apple::Apple;
use snake::Snake;

#[wasm_bindgen]
pub struct SnakeGame {
    width: u16,
    height: u16,

    snake: Snake,
    apple: Apple,
    score: u128,
    apple_add_cb: js_sys::Function,
}

#[wasm_bindgen]
impl SnakeGame {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u16, height: u16, apple_add_cb: js_sys::Function) -> Self {
        Self {
            width: width,
            height: height,
            score: 0,
            snake: Snake::new(width, height, width/2, height/2),
            apple: Apple::new(0, 0),

            apple_add_cb: apple_add_cb,
        }
    }

    fn emit_apple_delta(&self, delta: AppleDelta) {
        let js_state = serde_wasm_bindgen::to_value(&delta).unwrap();
        self.apple_add_cb.call1(&JsValue::NULL, &js_state).ok();
    }
}
