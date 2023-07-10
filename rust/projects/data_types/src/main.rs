fn main() {
    // Type Annotation for Ambiguous Variables
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{guess}")
}
