extern crate konva;

#[derive(Debug)]
struct Hello(i32, i32);

impl konva::container::Container for Hello {

}

fn main() {
    println!("konva!: {:?}", Hello(1,2));

}