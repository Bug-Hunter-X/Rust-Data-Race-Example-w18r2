# Rust Data Race Example

This repository demonstrates a simple example of a data race in Rust.  Data races occur when multiple mutable references to the same data exist simultaneously, leading to unpredictable and often incorrect results.

The `bug.rs` file contains the buggy code. The `bugSolution.rs` file provides a corrected version illustrating how to avoid data races.

## How to Run

1. Clone this repository.
2. Navigate to the repository directory.
3. Compile and run `bug.rs` using `rustc bug.rs && ./bug`. Observe the unpredictable result.
4. Compile and run `bugSolution.rs` using `rustc bugSolution.rs && ./bugSolution` . Observe the corrected behaviour.

## Understanding the Bug

The bug arises from attempting to simultaneously modify a variable (`x`) using multiple mutable references (`y` and `z`).  Rust's borrow checker normally prevents this; however, in this case, it's bypassed due to incorrect code structure. This leads to a data race where the final value of `x` is undefined and changes based on compiler optimization and runtime environment.

## Solution

The solution involves restructuring the code to avoid multiple mutable references pointing to the same data simultaneously.  Often, using `RefCell` or other mechanisms to handle mutable state within threads is required to solve real-world concurrency problems. The solution will focus on not creating multiple mutable references at the same time.