use crate::container;
use crate::group;
use crate::canvas;

pub struct Layer {
    pub _canvas: canvas::Canvas
}

impl Layer {
    pub fn new() -> Self {
        Layer { _canvas: canvas::Canvas::new() }
    }
}

impl container::Container<group::Group> for Layer {
    
}