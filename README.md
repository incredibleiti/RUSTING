# Project Overview

This project is a simple Rust application that demonstrates the addition of two integers and includes a guessing game.
Over the time this repo will keep on evolving, and adding more samples into it.
Goal is to make a descriptive demo tutorial which one can follow if coming from C++ or OOPs background

## Building the Project

To build the project, use the following command:

```bash
cargo build
```

This command compiles the project and generates an executable in the `target/debug` directory.

## Usage

To run the project, use the following command:

```bash
cargo run
```

The application will first perform the addition of two integers and then prompt the user to play a guessing game.

## Functionality

The application consists of a main function located in `src/main.rs`, which imports the `add` module from `src/add.rs` and the `guess` module from `src/guess.rs`. 

### The `add` Function

The `add` function is defined as follows:

```rust
pub fn add(value1: i32, value2: i32) -> i32 {
    value1 + value2
}
```

This function takes two `i32` values as parameters and returns their sum.

In the `main` function, the `add` function is called to compute the sum of two integers, `x` and `y`, and the result is printed to the console.

```
Now the value is: {sum}
```

### The `guess` Function

The `guess` function is defined as follows:

```rust
pub fn guess() {
    let secret = rand::thread_rng().gen_range(1..=10);
    // ...
}
```

This function generates a random number between 1 and 10 and prompts the user to guess the number. It provides feedback on whether the guess was correct or not.

## UML Diagram

```plaintext
+-----------------+
|     Main        |
|-----------------|
| + main()        |
+-----------------+
         |
         |
         v
+-----------------+
|      Add        |
|-----------------|
| + add(value1, value2) |
+-----------------+
         |
         |
         v
+-----------------+
|     Guess       |
|-----------------|
| + guess()       |
+-----------------+
