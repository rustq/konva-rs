use crate::container;
use crate::layer;
extern crate wasm_bindgen;

extern crate web_sys;

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

use wasm_bindgen::prelude::*;



#[derive(Debug)]
pub struct Stage {
    // container: &String
}

// #[wasm_bindgen]
impl Stage {


    pub fn new() -> Self {
        Self{ }
    }

    pub fn build_dom() -> Result<(), JsValue> {
		let window = web_sys::window().expect("global window does not exists");
		let document = window.document().expect("expecting a document on window");
		let container = document.get_element_by_id("container")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
            container.set_inner_html("QQQQ");
        let div = document.create_element("div")?;
        div.set_attribute("style", "position: relative; user-select: none;")?;
        div.set_attribute("class", "konva-rs-content")?;
        container.append_child(&div)?;

        Ok(())

    }
}

impl container::Container<layer::Layer> for Stage {
    
}