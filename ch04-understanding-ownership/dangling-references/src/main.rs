fn main() {
    let reference_to_nothing = no_dangle();
}

fn dangle() -> &String { // dangle returns a refernce to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. It's memory goes away

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
