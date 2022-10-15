use crate::context;
use crate::node;
use crate::shape;

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub color: String,
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
        }
    }

    pub fn draw(&self, ctx: &mut context::Context) -> () {
        ctx.set_fill_style(self.color.clone());
        ctx.fill_rect(self.x, self.y, self.width, self.height)
    }
}
