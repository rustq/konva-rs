use crate::canvas;
use crate::container;
use crate::context;
use crate::glue;
use crate::group;
use crate::rect;
use wasm_bindgen::prelude::*;

pub struct Layer {
    pub _children: Vec<rect::Rect>,
    pub _glue: glue::browser::BrowserGlue,
}

impl Layer {
    pub fn new() -> Self {
        let canvas = canvas::Canvas::new();
        Layer {
            _glue: glue::browser::BrowserGlue {
                html_canvas: canvas.native_element,
            },
            _children: Vec::new(),
        }
    }

    pub fn add(&mut self, _shape: rect::Rect) {
        self._children.push(_shape);
    }

    pub fn draw(&self) -> Result<(), JsValue> {
        let ctx = &mut context::Context::new();
        for i in 0..self._children.len() {
            let rect = &self._children[i];
            rect.draw(ctx);
        }
        self._glue.output_transaction_2ds(ctx);
        Ok(())
    }
}

impl container::Container<group::Group> for Layer {}
