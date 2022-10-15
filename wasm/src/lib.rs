extern crate konva;
extern crate wasm_bindgen;

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use konva::*;
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
    let mut s1 = stage::Stage::new();
    let mut layer1 = layer::Layer::new();
    let mut layer2 = layer::Layer::new();
    let shape1 = rect::Rect::new(10.0, 10.0, 30.0, 30.0, "red".to_string());
    let shape2 = rect::Rect::new(50.0, 20.0, 30.0, 30.0, "green".to_string());
    layer1.add(shape1);
    layer1.add(shape2);
    let shape3 = rect::Rect::new(80.0, 60.0, 30.0, 100.0, "blue".to_string());
    let shape4 = rect::Rect::new(70.0, 70.0, 100.0, 10.0, "pink".to_string());
    layer2.add(shape3);
    layer2.add(shape4);
    s1.add(layer1);
    s1.add(layer2);
    s1.batch_draw();
    Ok({})
}