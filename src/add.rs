use std::io;

pub fn add() {  
   println!("Give value for x: ");
   let x:i32 = take_input();

   println!("Give value for y=");
   let y:i32 = take_input();

   let sum = x+y;
   println!("The add results in: {}", sum);
}

fn take_input()->i32{
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Failed to read input");
   return input.trim().parse().expect("Please enter a valid number");
}