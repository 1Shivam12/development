fn main() {
    // Type Annotation for Ambiguous Variables
    // without u32 here it throws an error as parse can't determine what to convert to
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{guess}");

    // this works fine as i32 (aka signed), u32 causes a panic as unsigned can't be negative
    let num: i32 = "-42".parse().expect("Not a number");
    println!("{num}")

}
