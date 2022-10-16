use crate::context;
use crate::node;
use crate::shape;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub color: String,
    click_listener: Box<(dyn FnMut(i32) -> i32)>,
    gg: Box<dyn Fn(i32) -> i32>
}

impl node::Node for Rect {}

impl shape::Shape for Rect {}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64, color: String) -> Self {
        Rect {
            x,
            y,
            width,
            height,
            color,
            click_listener: Box::new(|x: i32| { x + 9 }),
            gg: Box::new(|x: i32| { x + 9 })
        }
    }

    pub fn draw(&self, ctx: &mut context::Context) -> () {
        ctx.set_fill_style(self.color.clone());
        ctx.fill_rect(self.x, self.y, self.width, self.height)
    }

    pub fn compose<F>(&mut self, x: i32, f: F, g: Box<dyn Fn(i32) -> i32>) -> i32
        where F: Fn(i32) -> i32 {
        self.gg = g;
        // g(f(x))
        (self.gg)(f(x))
    }

    pub fn biu3(&self, x: i32) -> i32 {
        (self.gg)(x)
    }

    pub fn on_click(&mut self, listener: Box<(dyn FnMut(i32) -> i32)>) {
        self.click_listener = listener;
    }

    pub fn fire(&mut self) {
        (self.click_listener)(8);
    }
}
