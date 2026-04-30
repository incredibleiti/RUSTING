use std::io;

mod add;
mod guess;
mod multiply;
mod util;

fn main() {
    println!("Which program you want to run?");
    println!("1. Add two numbers?");
    println!("2. Play the guessing number game?");
    println!("3. Multiply the numbers?");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    
    let number: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };
    
    match number {
        1 => { add::add(); },
        2 => { guess::guess(); },
        3 => { multiply::multiple(); },
        _ => println!("Ain't special"),
    }
}