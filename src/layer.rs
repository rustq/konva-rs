use crate::container;
use crate::group;
use crate::rect;
use crate::canvas;
use crate::glue;
use wasm_bindgen::prelude::*;
use crate::wasm_bindgen::JsCast;
use std::rc::Rc;

pub struct Layer {
    // pub _canvas: canvas::Canvas,
    pub _children: Vec<rect::Rect>,
    pub _glue: glue::browser::BrowserGlue,
}

impl Layer {
    pub fn new() -> Self {
        let _canvas = canvas::Canvas::new();
        Layer { _glue: glue::browser::BrowserGlue{ _canvas: _canvas.native_element }, _children: Vec::new() }
    }


    pub fn add(&mut self, _shape: rect::Rect) {
		self._children.push(_shape);
    }

    pub fn draw(&self) -> Result<(), JsValue> {
        let context = self._glue._canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
        let context: Rc<web_sys::CanvasRenderingContext2d> = Rc::new(context);
        let mut transactions: Vec<Box<glue::browser::Transaction2D>> = Vec::new();
        for i in 0..self._children.len() {
            let rect = &self._children[i];
            rect.draw(&mut transactions);
        }
        self._glue.output_transaction_2ds(&mut transactions);
        Ok(())
    }
}

impl container::Container<group::Group> for Layer {
    
}