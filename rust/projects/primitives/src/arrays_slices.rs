/*
An array is a collection of objects of the same type T, stored in contiguous memory. Arrays are created
 using brackets [], and their length, which is known at compile time, is part of their type signature 
 [T; length].

Slices are similar to arrays, but their length is not known at compile time. Instead, a slice is a 
two-word object; the first word is a pointer to the data, the second word is the length of the slice. 
The word size is the same as usize, determined by the processor architecture, e.g. 64 bits on an x86-64. 
Slices can be used to borrow a section of an array and have the type signature &[T].
*/
use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}


pub fn main () {
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