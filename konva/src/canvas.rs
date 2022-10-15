
extern crate wasm_bindgen;

extern crate web_sys;

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

use wasm_bindgen::prelude::*;

pub struct Canvas {
    pub native_element: web_sys::HtmlCanvasElement
}


impl Canvas {

    pub fn new() -> Self {
		let window = web_sys::window().expect("global window does not exists");
		let document = window.document().expect("expecting a document on window");
        let canvas = document.create_element("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
        Canvas{
            native_element: canvas
        }
    }
}