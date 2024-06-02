/*
A tuple is a collection of values of different types. Tuples are constructed using parentheses (), 
and each tuple itself is a value with type signature (T1, T2, ...), where T1, T2 are the types of 
its members. Functions can use tuples to return multiple values, as tuples can hold any number of values.
*/
use std::fmt::{Display, Formatter, Result};

fn transpose(input_matrix: Matrix) -> Matrix {

    let transposed = Matrix(
        input_matrix.0,
        input_matrix.2,
        input_matrix.1,
        input_matrix.3
    );

    transposed
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 'let' can be used to bind the members of a tuple to variables
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

// #[derive(Debug)] // This is a TRAIT for the matrix
struct Matrix(f32, f32, f32, f32);

pub fn main () {
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
}