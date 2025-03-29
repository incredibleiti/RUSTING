mod add;

fn main() {
    let x = 5; 
    let y = 9;
    let sum = add::add(x, y);
    
    println!("Now the value is: {sum}");
}

