fn main() {
    println!("Example of converting a string to a number
    ");
    // Example  of converting a string to a number.
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess: {} is type u32
    ", guess);

    println!("Examples of numeric types
    ");
    // Example of floating point numbers.
    let x = 2.0; // f64
    println!("x: {} is type f64", x);

    let y: f32 = 3.0; // f32
    println!("y: {} is type f32
    ", y);

    // Example of numeric operations
//     // addition
//     let sum = 5 + 10;
    
//     // subtraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // Results in -1

//     // remainder
//     let remainder = 43 % 5;
    println!("Examples of boolean logic
    ");
    // Example of boolean logic
    let t = true;
    println!("t: {} is type bool", t);

    let f: bool = false; // with explicit type annotation
    println!("f: {} is type bool
    ", f);

    // Example of char
    println!("Examples of chars
    ");
    let c = 'z';
    println!("c: {} is type char", c);

    let z: char = 'ℤ'; // with explicit type annotation
    println!("z: {} is type char with explicit type annotation", z);

    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat: {} is type char
    ", heart_eyed_cat);

    // Compound types
    // Tuples
    println!("Examples of compound types
    ");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x y and z below are from destructuring a tuple");    
    println!("x: {} is type i32 y: {} is type f64 z: {} is type u8", x, y, z);
    println!("the following numbers were accessed by from a tuple element using the .index method {} {} {}
    ", tup.0, tup.1, tup.2);

    // Arrays
    println!("let a = [1, 2, 3, 4, 5]; is an example of assigning an array to the variable a
    let a: [i32; 5] = [1, 2, 3, 4, 5]; is an example of the same assignment but explicitly declaring the 5 elements' types as i32
    let a = [3; 5]; is the same as let a = [3, 3, 3, 3, 3];
    array elements are accessed like this: a[0] where the bracketed number is the array index and a is the array");    
}