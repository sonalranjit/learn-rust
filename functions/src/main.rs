fn main() {

    let x = 5;

    // Expressions, like Groovy's closure
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Function with a return value
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // if you place a semi colon at the end it will change from an expression
    // to a statement
    x + 1
}
