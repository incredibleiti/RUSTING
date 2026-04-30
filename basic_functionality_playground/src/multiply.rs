use crate::util;

pub fn multiple () {
    println!("Enter the number 1");
    let x = util::take_input();

    println!("Enter number 2");
    let y = util::take_input();

    let mult_result = x * y;

    println!("The result of multiplication is: {}", mult_result);
}