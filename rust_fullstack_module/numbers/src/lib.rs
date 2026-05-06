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

/* creating genenric function which can be used to print both vector and array wont work, why??? */
pub fn print_array_arguement(numbers:[u8;5]) {
    for n in numbers.iter() {
        println!("{}", n);
    }
}

/* Understanding array implement copy trait and vector does not using the example below */

pub fn print_vector_arguement(numbers: Vec<u8>) {
    for n in numbers {
        println!("{}", n);
    }
}
/* revising this function to take any array or vector of u8 */
pub fn print_slice_arguement(numbers: &[u8]) {
    for n in numbers {
        println!("{}", n);
    }
}
