extern crate konva;
extern crate wasm_bindgen;

use konva::*;
use konva::stage::Stage;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello!!!!");
}

fn exec<F: FnOnce()>(f: F)  { // Fn 也可以传到 FnOnce 类型
    f() // 调用的是 Fn，所有权不会转移
}

fn exec1<F: FnMut()>(mut f: F)  { // Fn 也可以传到 FnMut 类型
    f()
}

fn exec2<F: Fn()>(f: F)  {
    f()
}


#[wasm_bindgen]
pub fn play() -> Result<(), JsValue> {
    let mut layer1 = layer::Layer::new();
    let mut layer2 = layer::Layer::new();
    let shape1 = rect::Rect::new(10.0, 10.0, 30.0, 30.0, "red".to_string());
    let shape2 = rect::Rect::new(50.0, 20.0, 30.0, 30.0, "green".to_string());
    layer1.add(shape1);
    layer1.add(shape2);
    let shape3 = rect::Rect::new(80.0, 60.0, 30.0, 100.0, "blue".to_string());
    let shape4 = rect::Rect::new(70.0, 70.0, 100.0, 10.0, "pink".to_string());
    layer2.add(shape3);
    layer2.add(shape4);

    // let mut on_shape5_click = || {
    //     let mut s2 = stage::Stage::new();
    //     let mut layer3 = layer::Layer::new();
    //     let shape6 = rect::Rect::new(40.0, 80.0, 10.0, 10.0, "cyan".to_string());
    //     layer3.add(shape6);
    //     s2.add(layer3);
    //     s2.batch_draw();
    // };
    // let on_shape5_click = Closure::wrap(Box::new(|e: web_sys::Event| {
    //     let mut s2 = stage::Stage::new();
    //     let mut layer3 = layer::Layer::new();
    //     let shape6 = rect::Rect::new(40.0, 80.0, 10.0, 10.0, "cyan".to_string());
    //     layer3.add(shape6);
    //     s2.add(layer3);
    //     s2.batch_draw();
    // }) as Box<dyn FnMut(web_sys::Event)>);
    // let mut shape5 = rect::Rect::new(40.0, 80.0, 10.0, 10.0, "yellow".to_string());
    // shape5.on_click(on_shape5_click);
    // s1.listen();
    let stage = Rc::new(RefCell::new(stage::Stage::new()));

    let mut shape5 = rect::Rect::new(40.0, 80.0, 10.0, 10.0, "cyan".to_string());
    shape5.compose(5,
        |n: i32| { n + 42 },
        Box::new(|n: i32| { n * 2 })); // evaluates to 94
    shape5.biu3(3);
    shape5.biu3(3);
    shape5.biu3(3);

    let s1 = Rc::clone(&stage);
    shape5.on_click(Box::new(move |x: i32| {
        let mut layer3 = layer::Layer::new();
        let shape6 = rect::Rect::new(40.0, 80.0, 10.0, 10.0, "pink".to_string());
        layer3.add(shape6);
        s1.borrow().batch_draw();
        3
    }));


    layer2.add(shape5);

    let s2 = Rc::clone(&stage);
    s2.borrow_mut().add(layer1);
    s2.borrow_mut().add(layer2);
    s2.borrow_mut().batch_draw();

    Stage::listen(Rc::clone(&stage));
    Ok({})
}
