fn main() {
    let number = 7;
    if number == 5 {
        println!("True")
    } else if number < 10  {
        println!("Else If")
    } else {
        println!("False")
    }

    // Using if in let statement
    let condition = true;
    let number = if condition {5} else {6.4}; // Error as both results MUST be same type

    println!("Value of number is {number}")
}
