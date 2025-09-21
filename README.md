# Rust Learning Experiments 🦀

A modular Rust learning repository with organized examples covering fundamental
to advanced concepts.

## Project Structure

```
src/
├── main.rs                 # Main entry point with module organization
├── basics/                 # Fundamental Rust concepts
│   ├── mod.rs
│   ├── variables.rs       # Variables, data types, mutability
│   ├── functions.rs       # Function declaration and usage
│   └── control_flow.rs    # If/else, loops, match expressions
├── ownership/             # Memory management concepts
│   └── mod.rs
├── error_handling/        # Result<T, E>, Option<T>, error handling
│   └── mod.rs
├── collections/           # Vec, HashMap, HashSet, etc.
│   └── mod.rs
├── traits/               # Trait definition and implementation
│   └── mod.rs
└── generics/             # Generic programming concepts
    └── mod.rs
```

## Getting Started

1. **Clone and run:**

   ```bash
   git clone <your-repo-url>
   cd rs-experiments
   cargo run
   ```

2. **Explore modules:**
   - Uncomment module calls in `main.rs` to explore different topics
   - Each module contains focused examples with clear explanations

## Learning Path

### Beginner (Basics)

- **Variables**: Data types, mutability, shadowing, constants
- **Functions**: Declaration, parameters, return values, expressions vs
  statements
- **Control Flow**: If/else, loops, match expressions, pattern matching

### Intermediate

- **Ownership**: Memory management, borrowing, lifetimes
- **Error Handling**: Result and Option types, proper error handling
- **Collections**: Common data structures and their usage

### Advanced

- **Traits**: Defining and implementing traits, trait bounds
- **Generics**: Generic functions, structs, enums, trait bounds

## Usage

Each module is self-contained with a `run()` function that demonstrates concepts
through examples. To explore a specific topic:

1. Uncomment the corresponding module call in `main.rs`
2. Run `cargo run` to see the examples
3. Study the code and experiment with modifications

## Contributing

Feel free to add more examples or improve existing ones. Follow the modular
structure and include clear comments explaining concepts.

## License

MIT License - see LICENSE file for details.
