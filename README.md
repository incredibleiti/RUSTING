# Project Overview

This project is a simple Rust application that demonstrates the addition of two integers using a modular approach.

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

## Functionality

The application consists of a main function located in `src/main.rs`, which imports the `add` module from `src/add.rs`. 

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
