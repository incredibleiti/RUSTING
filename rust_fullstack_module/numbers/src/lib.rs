pub fn say_hello() {
    println!("Hi this a call from library");
}

pub fn print_number_array() {
    let numbers = [1, 2, 3, 4, 5];
    for n in numbers.iter() {
        println!("{}", n);
    }
}

pub fn print_number_vector() {
    let numbers = vec![5, 6, 7, 8, 9];
    for n in numbers {
        println!("{}", n);
    }
}

pub fn print_array_arguement(numbers:[u8;5]) {
     for n in numbers.iter() {
        println!("{}", n);
    }
}
