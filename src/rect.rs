use crate::node;
use crate::shape;

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64
}

impl node::Node for Rect {
    
}

impl shape::Shape for Rect {
    
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Rect { x, y, width, height }
    }
}