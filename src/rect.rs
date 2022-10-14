use crate::glue::browser::Transaction2D;
use crate::glue::browser::Transaction2DParam;
use crate::glue::browser::Transaction2DMethod;
use crate::node;
use crate::shape;
use std::rc::Rc;

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

    pub fn draw(&self, mut transactions: &mut Vec<Box<Transaction2D>>) -> () {
        transactions.push(Box::new(Transaction2D{ method: Transaction2DMethod::fill_rect, param: Transaction2DParam{ x: self.x, y: self.y, width: self.width, height: self.height } }));
    }
}