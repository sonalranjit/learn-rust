
#![allow(unused)]
fn main() {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }
}
