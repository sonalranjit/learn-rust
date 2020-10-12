pub trait Iterator {
    type Item;

    fn next(&mut self) -> Options<Self::Item>;
}
