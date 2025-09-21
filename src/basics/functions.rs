// Functions in Rust
// Demonstrates function declaration, parameters, return values, and expressions vs statements

pub fn run() {
    println!("--- Rust Functions Demo ---");

    // 1. Basic function call
    greet("Alice");

    // 2. Function with parameters and return value
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);

    // 3. Function with explicit return
    let product = multiply(4, 6);
    println!("4 * 6 = {}", product);

    // 4. Function with implicit return (expression)
    let difference = subtract(10, 3);
    println!("10 - 3 = {}", difference);

    // 5. Function with multiple statements
    let result = complex_calculation(5);
    println!("Complex calculation result: {}", result);

    println!("--- End of Functions Demo ---\n");
}

// Simple function with no return value
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function with parameters and explicit return
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// Function with explicit return statement
fn multiply(a: i32, b: i32) -> i32 {
    let result = a * b;
    return result;
}

// Function with implicit return (expression - no semicolon)
fn subtract(a: i32, b: i32) -> i32 {
    a - b  // This is an expression, not a statement
}

// Function with multiple statements and expressions
fn complex_calculation(x: i32) -> i32 {
    let doubled = x * 2;
    let squared = doubled * doubled;
    squared + 10  // Implicit return
}
