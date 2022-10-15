use crate::container;
use crate::glue;
use crate::layer;

pub struct Stage {
    content: web_sys::HtmlElement,
    pub _children: Vec<layer::Layer>,
}

impl Stage {
    pub fn new() -> Self {
        let div = glue::browser::BrowserGlue::create_div_inside_container("container".to_string());
        Stage {
            content: div,
            _children: Vec::new(),
        }
    }

    pub fn add(&mut self, _layer: layer::Layer) {
        glue::browser::BrowserGlue::append_canvas_to_content_div(&_layer._glue.html_canvas, &self.content);
        self._children.push(_layer);
    }

    pub fn batch_draw(&self) {
        for i in 0..self._children.len() {
            let layer = &self._children[i];
            layer.draw();
        }
    }
}

impl container::Container<layer::Layer> for Stage {}
