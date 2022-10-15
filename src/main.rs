extern crate konva;

#[cfg(test)]
mod test {

    #[test]
    fn hello() {
        let rect = konva::rect::Rect::new(10.0, 10.0, 10.0, 10.0, "red".to_string());
        assert_eq!(rect.color, "red".to_string());
    }
}

fn main() {
    ()
}
