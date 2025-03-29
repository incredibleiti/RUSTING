fn add(value1:i32, value2:i32)->i32{
   return value1 + value2;
}

fn main() {
    let x = 5; 
    let y = 9;
    let sum = add(x, y);
    
    println!("Now the value is: {sum}");
}

