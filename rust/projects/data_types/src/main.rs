use std::num::Wrapping;

fn main() {
    // Type Annotation for Ambiguous Variables
    // without u32 here it throws an error as parse can't determine what to convert to
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{guess}");

    // this works fine as i32 (aka signed), u32 causes a panic as unsigned can't be negative
    let num: i32 = "-42".parse().expect("Not a number");
    println!("{num}");

    // Wrapping in rust

    let zero = Wrapping(0u32); // unsigned 0
    let one = Wrapping(1u32); // unsigned 1

    assert_eq!(u32::MAX, (zero-one).0); // 0 - 1 results in negative, which is not allowed. Wrapping wraps it around to u32 MAX value

    // Floating types
    let x: f64 = 2.0;
    let y: f32 = 3.0;

    // Numeric Operations
    let sum = 5 + 10;

    let diff = 95.5 - 4.3;

    let product = 4*30;

    let quotient = 56.7/32.2;

    let truncated = -5/3;

    let remainder = 43%5;

    // Boolean Types
    let t = true;

    let f: bool = false;

    // Character Type - Use single quotes unlike string literals that use doubles
    let c = 'z';

    let z: char = 'ℤ';

    let cat = '😻';

    // Compound Types - Tuples and Arrays
    // Tuples
    let tup: (i32, f64, char) = (500, 6.4, 'z');
    println!("{:?}", tup);
    // Accessing tuple values by destructing
    let (x,y,z) = tup;
    println!("The value of y is: {z}");

    // Directly Accessing Tuple Values
    let first_char = tup.0;
    println!("The first char is: {first_char}");

    // Special empty tuple called unit, is the None type
    let unit = ();

    // Array
    let a = [1,2,3,4,5];
    println!("{:?}",a);
    let first_arr_char = a[0];
    println!("{first_arr_char}");
    let b = [5; 10]; // create array of 5's with 10 instances

    // Rust has memory safety unlike other languages (e.g. C)
    // Errors out like in python when index > length of array
    let memory_error = b[10];
    let memory_error_2 = tup.3;
    println!("{memory_error}")
}
