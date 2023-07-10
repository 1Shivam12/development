fn main() {
    // Immutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 *60; // Use all uppercase
    println!("The value of 3 hours in seconds is : {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let m = 5;

    let m = m + 1;

    {
        let m = m * 2;
        println!{"The value of m in the inner scope is : {m}"}
    }

    println!{"The value of m is : {m}"};

    // Shadowing Example 2
    // Can't mutate a variables type - Won't work
    // let mut spaces = "    ";
    // spaces = spaces.len();
    // println!("Length of spaces is {spaces}");

    // Can mutate a shadowed variable type - Will work
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Length of spaces is {spaces}");
}