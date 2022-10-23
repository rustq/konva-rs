use crate::context;
use crate::stage;
use crate::layer;
use crate::rect;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct ListenerEvent {}

pub struct Trigger {}

pub struct BrowserGlue {
    pub html_canvas: web_sys::HtmlCanvasElement,
}

impl BrowserGlue {
    pub fn create_div_inside_container(container_id: String) -> web_sys::HtmlElement {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let container = document
            .get_element_by_id(container_id.as_str())
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        container.set_inner_html("Hello Stage");
        let div = document
            .create_element("div")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        div.set_attribute("style", "position: relative; user-select: none;");
        div.set_attribute("class", "konva-rs-content");
        container.append_child(&div);
        div
    }

    pub fn create_canvas() -> web_sys::HtmlCanvasElement {
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        canvas
    }

    // pub fn listen(event_name: String, listener: &'static mut dyn FnMut(i32) -> i32) -> Result<(), JsValue> {

    //     let cb = Closure::wrap(Box::new(|e: web_sys::Event| {
    //         exec(Box::leak(Box::new(listener)));
    //     }) as Box<dyn FnMut(web_sys::Event)>);
    //     // exec(listener);
        
    //     let window = web_sys::window().expect("global window does not exists");
    //     let document = window.document().expect("expecting a document on window");
    //     document.add_event_listener_with_callback(event_name.as_str(), &cb.as_ref().unchecked_ref())?;
        
    //     cb.forget();
    //     Ok(())
    // }

    pub fn listen(event_name: String, st: &'static mut stage::Stage) -> Result<(), JsValue> {
        let sst = Box::leak(Box::new(RefCell::new(st)));
        let cb = Closure::wrap(Box::new(|e: web_sys::Event| {
            sst.borrow_mut().batch_fire();
            let mut layer3 = layer::Layer::new();
            let shape6 = rect::Rect::new(50.0, 90.0, 35.0, 15.0, "yellow".to_string());
            layer3.add(shape6);
            sst.borrow_mut().add(layer3);
            sst.borrow_mut().batch_draw();
        }) as Box<dyn FnMut(web_sys::Event) + 'static>);
        // sst.borrow_mut().batch_fire();
        let window = web_sys::window().expect("global window does not exists");
        let document = window.document().expect("expecting a document on window");
        document.add_event_listener_with_callback(event_name.as_str(), cb.as_ref().unchecked_ref())?;
        
        cb.forget();
        Ok(())
    }

    pub fn append_canvas_to_content_div(canvas_element: &web_sys::HtmlCanvasElement, div_element: &web_sys::HtmlElement) {
        canvas_element.set_attribute("style", "position: absolute;");
        div_element.append_child(&canvas_element);
    }

    pub fn output_transaction_2ds(&self, ctx: &mut context::Context) -> Result<(), JsValue> {
        let context2d = self
            .html_canvas
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
                    let param = &ctx.get_fill_rect_param_list()[fill_rect_list_pointer];
                    fill_rect_list_pointer += 1;
                    context2d.fill_rect(param.x, param.y, param.width, param.height)
                }
                context::Transaction2DMethod::set_fill_style => {
                    let param = &ctx.get_set_fill_style_param_list()[set_fill_style_list_pointer];
                    set_fill_style_list_pointer += 1;
                    context2d.set_fill_style(&JsValue::from_str(&param.color))
                }
                _ => (),
            }
        }
        Ok(())
    }

    pub fn output_events(events: Vec<ListenerEvent>) {}

    pub fn input_trigger(trigger: Trigger) {}
}

fn exec<F: FnOnce(i32) -> i32>(f: F)  { // Fn 也可以传到 FnOnce 类型
    f(3); // 调用的是 Fn，所有权不会转移
}

fn exec1<F: FnOnce(i32) -> i32>(mut f: F)  { // Fn 也可以传到 FnMut 类型
    f(3);
}

fn exec2<F: FnMut(i32) -> i32>(mut f: F)  {
    (f)(3);
}