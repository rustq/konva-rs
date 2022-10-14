use std::rc::Rc;
use wasm_bindgen::prelude::*;
use crate::wasm_bindgen::JsCast;

#[derive(PartialEq)]
pub enum Transaction2DMethod {
    fill_rect,
    fill_stroke,
}


pub struct Transaction2DParam {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64
}
pub struct Transaction2D {
    pub method: Transaction2DMethod,
    pub param: Transaction2DParam
}

pub struct ListenerEvent {

}

pub struct Trigger {

}

pub struct BrowserGlue {
    pub _canvas: web_sys::HtmlCanvasElement
}



impl BrowserGlue {
    pub fn output_transaction_2ds(&self, transactions: &mut Vec<Box<Transaction2D>>) -> Result<(), JsValue> {
        let context = self._canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
        let context: Rc<web_sys::CanvasRenderingContext2d> = Rc::new(context);
        context.set_fill_style(&JsValue::from_str("red"));
        for i in 0..transactions.len() {
            let transition = &transactions[i];
            if transition.method == Transaction2DMethod::fill_rect {
                context.fill_rect(transition.param.x, transition.param.y, transition.param.width, transition.param.height);
            }
        }
        Ok(())

    }

    pub fn output_events(events: Vec<ListenerEvent>) {

    }

    pub fn input_trigger(trigger: Trigger) {

    }
}