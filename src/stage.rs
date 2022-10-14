use crate::container;
use crate::layer;
extern crate wasm_bindgen;

extern crate web_sys;

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

use wasm_bindgen::prelude::*;



// #[derive(Debug)]
pub struct Stage {
    content: web_sys::HtmlElement,
    pub _children: Vec<layer::Layer>
}

// #[wasm_bindgen]
impl Stage {


    pub fn new() -> Self {
        let window = web_sys::window().expect("global window does not exists");
		let document = window.document().expect("expecting a document on window");
		let container = document.get_element_by_id("container")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
            container.set_inner_html("QQQQ");
        let div = document.create_element("div").unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        div.set_attribute("style", "position: relative; user-select: none;");
        div.set_attribute("class", "konva-rs-content");
        container.append_child(&div);
        Stage{ content: div, _children: Vec::new() }
    }

    pub fn add(&mut self, _layer: layer::Layer) {
		let content = &self.content;
        content.append_child(&_layer._glue._canvas);
        self._children.push(_layer);
    }


    pub fn batch_draw(&self) {
        for i in 0..self._children.len() {
            let layer = &self._children[i];
            layer.draw();
        }
    }
}

impl container::Container<layer::Layer> for Stage {
    
}