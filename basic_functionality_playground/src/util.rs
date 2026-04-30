use std::io;

pub fn take_input()->i32{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    return input.trim().parse().expect("Please enter a valid number");
 }