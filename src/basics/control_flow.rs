// Control flow in Rust
// Demonstrates if/else, loops, match expressions, and pattern matching

pub fn run() {
    println!("--- Rust Control Flow Demo ---");

    // 1. If/else statements
    let number = 7;
    if number < 5 {
        println!("{} is less than 5", number);
    } else if number < 10 {
        println!("{} is between 5 and 10", number);
    } else {
        println!("{} is 10 or greater", number);
    }

    // 2. If as expression
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("Result of if expression: {}", result);

    // 3. Loop
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 3 {
            break;
        }
    }
    println!("Counter after loop: {}", counter);

    // 4. While loop
    let mut countdown = 3;
    while countdown > 0 {
        println!("Countdown: {}", countdown);
        countdown -= 1;
    }
    println!("Liftoff!");

    // 5. For loop
    let array = [10, 20, 30, 40, 50];
    for element in array.iter() {
        println!("Array element: {}", element);
    }

    // 6. Range in for loop
    for number in 1..4 {
        println!("Range number: {}", number);
    }

    // 7. Match expression
    let value = 3;
    match value {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }

    // 8. Match with ranges
    let age = 25;
    match age {
        0..=12 => println!("Child"),
        13..=19 => println!("Teenager"),
        20..=64 => println!("Adult"),
        65..=120 => println!("Senior"),
        _ => println!("Invalid age"),
    }

    println!("--- End of Control Flow Demo ---\n");
}
