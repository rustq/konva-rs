use crate::context;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use crate::wasm_bindgen::JsCast;

pub struct ListenerEvent {

}

pub struct Trigger {

}

pub struct BrowserGlue {
    pub html_canvas: web_sys::HtmlCanvasElement
}



impl BrowserGlue {
    pub fn output_transaction_2ds(&self, ctx: &mut context::Context) -> Result<(), JsValue> {
        let context2d = self.html_canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
        let context2d: Rc<web_sys::CanvasRenderingContext2d> = Rc::new(context2d);
        let mut fill_rect_list_pointer: usize = 0;
        let mut set_fill_style_list_pointer: usize = 0;
        for i in 0..ctx.get_transaction_methods().len() {
            let transition = &ctx.get_transaction_methods()[i];
            match transition {
                context::Transaction2DMethod::fill_rect => {
                    let param = &ctx.fill_rect_parma_list[fill_rect_list_pointer];
                    fill_rect_list_pointer += 1;
                    context2d.fill_rect(param.x, param.y, param.width, param.height)
                },
                context::Transaction2DMethod::set_fill_style => {
                    let param = &ctx.set_fill_style_parma_list[set_fill_style_list_pointer];
                    set_fill_style_list_pointer += 1;
                    context2d.set_fill_style(&JsValue::from_str(&param.color))
                },
                _ => (),
            }
        }
        Ok(())

    }

    pub fn output_events(events: Vec<ListenerEvent>) {

    }

    pub fn input_trigger(trigger: Trigger) {

    }
}