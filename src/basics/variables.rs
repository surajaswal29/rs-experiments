// Variables and data types in Rust
// Demonstrates variable declaration, mutability, shadowing, and data types

pub fn run() {
    println!("--- Rust Variables Demo ---");

    // 1. Immutable variable (default)
    let x = 5;
    println!("Immutable x: {}", x);
    // x = 6; // ❌ This will cause a compile-time error

    // 2. Mutable variable
    let mut y = 10;
    println!("Mutable y before change: {}", y);
    y = 20;
    println!("Mutable y after change: {}", y);

    // 3. Shadowing
    let z = 30;
    println!("Original z: {}", z);
    let z = z + 5; // shadowing allows reusing same variable name
    println!("Shadowed z: {}", z);

    // 4. Constants
    const PI: f64 = 3.14159;
    println!("Constant PI: {}", PI);

    // 5. Data types
    let a: i32 = 100;          // integer
    let b: f64 = 10.5;         // float
    let c: bool = true;        // boolean
    let d: char = 'R';         // character
    let e: &str = "Hello Rust"; // string slice
    let f: String = String::from("Owned String"); // heap-allocated string

    println!("Integer a: {}", a);
    println!("Float b: {}", b);
    println!("Boolean c: {}", c);
    println!("Character d: {}", d);
    println!("String slice e: {}", e);
    println!("String f: {}", f);

    // 6. Multiple variables in one line
    let (g, h) = (1, "two");
    println!("Tuple destructuring: g = {}, h = {}", g, h);

    println!("--- End of Variables Demo ---\n");
}
