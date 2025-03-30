use std::io;
use rand::Rng;

pub fn guess() {
    let secret = rand::thread_rng().gen_range(1..=10);
    println!("Guess a number between 1 and 10:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let guess: i32 = input.trim().parse().expect("Enter a valid number");

    if guess == secret {
        println!("Correct!");
    } else {
        println!("Wrong! Secret was: {}", secret);
    }

}