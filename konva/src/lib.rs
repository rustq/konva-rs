pub mod canvas;
pub mod container;
pub mod context;
pub mod glue;
pub mod group;
pub mod layer;
pub mod node;
pub mod rect;
pub mod shape;
pub mod stage;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
