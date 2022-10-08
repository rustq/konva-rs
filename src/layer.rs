use crate::container;
use crate::group;
use crate::rect;
use crate::canvas;
use wasm_bindgen::prelude::*;
use crate::wasm_bindgen::JsCast;
use std::rc::Rc;

pub struct Layer {
    pub _canvas: canvas::Canvas,
    pub _children: Vec<rect::Rect>
}

impl Layer {
    pub fn new() -> Self {
        Layer { _canvas: canvas::Canvas::new(), _children: Vec::new() }
    }


    pub fn add(&mut self, _shape: rect::Rect) {
		self._children.push(_shape);
    }

    pub fn draw(&self) -> Result<(), JsValue> {
        let context = self._canvas.native_element
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
        let context: Rc<web_sys::CanvasRenderingContext2d> = Rc::new(context);
        for i in 0..self._children.len() {
            let rect = &self._children[i];
            rect.draw(&context)
        }
        Ok(())
    }
}

impl container::Container<group::Group> for Layer {
    
}