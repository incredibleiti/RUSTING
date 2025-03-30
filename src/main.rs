use std::io;

mod add;
mod guess;

fn main() {
    println!("Which program you want to run?");
    println!("1. Add two numbers?");
    println!("2. Play the guessing number game?");

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
        _ => println!("Ain't special"),
    }
}