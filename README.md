# Multiple Mutable Borrows in Rust

This repository demonstrates a common error in Rust: attempting to create multiple mutable borrows of the same variable.  Rust's borrow checker prevents this to avoid data races and ensure memory safety.  The `bug.rs` file shows the erroneous code, while `bugSolution.rs` demonstrates a solution.

## How to run

1. Clone the repository.
2. Navigate to the directory.
3. Compile and run `bug.rs` (it will fail).
4. Compile and run `bugSolution.rs` (it will succeed).