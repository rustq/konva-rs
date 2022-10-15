use crate::glue;
extern crate web_sys;

pub struct Canvas {
    pub native_element: web_sys::HtmlCanvasElement,
}

impl Canvas {
    pub fn new() -> Self {
        let native_canvas_element = glue::browser::BrowserGlue::create_canvas();
        Canvas {
            native_element: native_canvas_element,
        }
    }
}
