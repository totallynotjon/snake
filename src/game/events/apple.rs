use crate::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct AppleDelta {
    pub x: u32, //coordinates of the apple create/remove
    pub y: u32,
    pub exist: bool, //if exist create, if no exist remove
}
