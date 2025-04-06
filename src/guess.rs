use rand::{rng, Rng};

use crate::util;

pub fn guess() {
    let mut rng = rng(); // instead of thread_rng()
    let secret = rng.random_range(1..=10); // instead of gen_range()
    println!("Guess a number between 1 and 10:");

    let guess = util::take_input();

    if guess == secret {
        println!("Correct!");
    } else {
        println!("Wrong! Secret was: {}", secret);
    }

}