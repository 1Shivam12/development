fn main() {
    println!("Hello, world!");

    another_function(5, '\u{263A}');
    statements_and_expressions();
    let out = plus_5(10);
    println!("{out}")
}

fn another_function(x: i32, unit_label: char) {
    println!("The value of x is: {x}{unit_label}");
}

fn statements_and_expressions() {
    let y = {
        let x = 3; // Statement
        x + 1 // Expressions, DONT have ending semi-colons
    }; // Adding a scope block makes this a expression

    println!("The value of y is: {y}")
}

fn plus_5(x: i32) -> i32 {
    x + 5
}
