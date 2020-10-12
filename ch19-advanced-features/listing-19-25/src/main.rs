fn main() {
    type Thunk = Box<dyn Fn() + Send 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type() {}

    fn returns_long_type() {}
}
