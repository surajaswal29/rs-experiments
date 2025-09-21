mod basics;
// mod ownership;  // Rust's unique memory management system
// mod error_handling; // Result<T, E>, Option<T>, and proper error handling patterns
// mod collections; // Vec, HashMap, HashSet, etc.
// mod traits; // Trait definition and implementation
// mod generics; // Generic programming concepts

use basics::variables;
use basics::functions;
use basics::control_flow;

fn main() {
    println!("🦀 Rust Learning Experiments 🦀");
    println!("================================");
    
    // Run basic concepts
    println!("\n📚 BASIC CONCEPTS");
    println!("-----------------");
    variables::run();
    functions::run();
    control_flow::run();
    
    // Uncomment to explore other topics
    // ownership::run();
    // error_handling::run();
    // collections::run();
    // traits::run();
    // generics::run();
}
