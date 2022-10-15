pub mod stage;
pub mod layer;
pub mod group;
pub mod shape;
pub mod rect;
pub mod container;
pub mod node;
pub mod canvas;
pub mod context;
pub mod glue;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
