use std::fmt::{Display, Formatter, Result};
use std::mem;

/*
A tuple is a collection of values of different types. Tuples are constructed using parentheses (), 
and each tuple itself is a value with type signature (T1, T2, ...), where T1, T2 are the types of 
its members. Functions can use tuples to return multiple values, as tuples can hold any number of values.
*/

/*
An array is a collection of objects of the same type T, stored in contiguous memory. Arrays are created
 using brackets [], and their length, which is known at compile time, is part of their type signature 
 [T; length].

Slices are similar to arrays, but their length is not known at compile time. Instead, a slice is a 
two-word object; the first word is a pointer to the data, the second word is the length of the slice. 
The word size is the same as usize, determined by the processor architecture, e.g. 64 bits on an x86-64. 
Slices can be used to borrow a section of an array and have the type signature &[T].
*/

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 'let' can be used to bind the members of a tuple to variables
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

fn transpose(input_matrix: Matrix) -> Matrix {

    let transposed = Matrix(
        input_matrix.0,
        input_matrix.2,
        input_matrix.1,
        input_matrix.3
    );

    transposed
}

// #[derive(Debug)] // This is a TRAIT for the matrix
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}


fn main () {

    // Literals and Operators
    // Integer Addition
    println!("1 + 2 = {}", 1u32 + 2);
    
    // Integer Subtraction
    println!("1 - 2 = {}", 1i32 - 2); // i32 is a signed integer, so can be negative

    // Scientific Notification
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short Circuit Boolean Logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("Not true is {}", !true);

    // Bitwise Operations
    println!("0011 and 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    // Tuples

    // Tuple with a bunch of types
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    println!("Long Tuple first value: {}", long_tuple.0);
    println!("Long Tuple second value: {}", long_tuple.1);

    // Tuple of Tuples
    let tuple_of_tuples = ((1u8, 2u8, 3u8), (-1i8, -2i16, -3i32));

    println!("Tuple of Tuples is {:?}", tuple_of_tuples);

    // But long Tuples (more than 12 elements) cannot be printed.
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Destructure Tuples
    let tuple = (1, "hello", 4.5, true);

    let (a,b,c,d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a,b,c,d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transposed:\n{}", transpose(matrix));

    // Arrays and Slices
    // Fixed size array
    let xs: [i32; 5] = [1,2,3,4,5];

    // All elements are initialized to the same value in this example - Another Fixed Size Array
    let ys: [i32; 500] = [0; 500];

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array, in the form [starting_index..ending_index]
    // starting_index is first position of the slice, ending_index is one more than the last position of the slice
    println!("Borrow a slice of the array.");
    analyze_slice(&ys[1 .. 4]);

    // Example of empty slice
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    // Arrays can be safely accessed using `.get` which returns an `Option`, which can be matched as shown
    // or can be matched using .expect()
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow Down! {} is too far", i),
        }
    }

    // Out of bound on array causes compile time error
    // Out of bound on slice causes run-time error

}

