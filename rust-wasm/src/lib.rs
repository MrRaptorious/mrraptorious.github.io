use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = add_num)] 
pub fn add_num(left: i32, right: i32) -> i32 {
    left + right
}