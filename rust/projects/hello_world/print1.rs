fn main() {
    println!("Rust Tutorial 2.0");

    // Positional Arguments
    println!("{0} just went and bought {1}. {1} is owned by {0}", "Alice", "Dog");

    // Keyword Arguments
    println!("{person} {place} {thing}",
            person="Steven",
            place="SA",
            thing="ball");

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    //fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // Activity 1
    println!("The name is {1}. {0} {1}", "James", "Bond");

    let pi = 3.141592;

    // Activity 2
    println!("Pi is roughly {:.precision$}", pi, precision=3);
}
