use crate::util;

pub fn add() {  
   println!("Give value for x: ");
   let x:i32 = util::take_input();

   println!("Give value for y=");
   let y:i32 = util::take_input();

   let sum = x+y;
   println!("The add results in: {}", sum);
}

