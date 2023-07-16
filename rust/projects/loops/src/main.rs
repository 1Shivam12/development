fn main() {
    // loop
    // loop {
    //     println!{"runs forever"};
    // }

    // Returning values from loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println! {"Thr result is {result}"};

    // Loop Labels for Multiple Loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println! {"End count = {count}"};

    // Conditional Loops
    let mut cnt = 0;

    while cnt <= 2 {
        println!("The count is {cnt}");
        cnt += 1;
    }

    println!("Loop over");

    // Loop through a collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is {}", a[index]);

        index += 1;
    }
    // The while loop to go through an iterable is slow as the compiler has to execute run-time
    // code to perform conditional check of whether index is within bounds of array at every iteration

    // Faster is the for loop
    for element in a {
        println!("the value is {element}");
    }

    // For loop is the most common and safest
    for number in (1..5).step_by(2).rev() {
        println!("{number}");
    }
    println!("We are done!!");
}