
fn main() {
    //crate::module::function
    // numbers::say_hello(); //calling sequence test
    // numbers::print_number_array();
    // numbers::print_number_vector();

    // Passing twice the vector to understand copy trait implementation
    let numbers_vec = vec![1,2,3,4,5,6,7,8,9,10];
    numbers::print_vector_arguement(numbers_vec);
    numbers::print_vector_arguement(numbers_vec);
}
