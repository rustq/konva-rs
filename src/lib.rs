pub mod stage;
pub mod layer;
pub mod group;
pub mod shape;
pub mod container;
pub mod node;

extern crate wasm_bindgen;

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello!!!!");
}


#[wasm_bindgen]
pub fn play() -> Result<(), JsValue> {
    let s1 = stage::Stage::new();
    stage::Stage::build_dom()
}