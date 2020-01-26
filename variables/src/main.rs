fn main() {
    let x = 5;
    
    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    // Floating-Point types
    let float_type = 2.0; //f64
    println!("Floating type: {}", float_type);
    
    //f64 is by default 
    let y: f32 = 3.0; //f32
    println!("Floating 32: {}", y);

    // addition
    let sum = 5 + 10;
    println!("Sum of 5 + 10 is: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("Difference of 95.5 - 4.3 is: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("Multiplication of 4 and 30 is: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("Division of 56.7 / 32.2 is: {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("Remainder of 43 % 4 is: {}", remainder);
    
    // boolean type
    let t = true;
    println!("t is: {}", t);

    let f: bool = false; // explicit type declaration
    println!("f is: {}", f);

    // Compound types
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
    println!("The value of z is: {}", z);

    // Array type
    let a = [1, 2, 3, 4, 5];
    println!("Array a: {}", a[0]);

    // set array's type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array a at 2: {}", a[1]);

    // initialize an array with values
    let a = [3; 4];
    println!("Array a at 3: {}", a[2]);
}
