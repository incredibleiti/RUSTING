
fn main() {
    /* ------crate::module::function------- */
    // numbers::say_hello(); //calling sequence test
    // numbers::print_number_array();
    // numbers::print_number_vector();

    /* ------- Passing twice the vector to understand copy trait implementation -----*/
    // let num_vec = vec![6,7,8,9,10];
    // numbers::print_vector_arguement(num_vec);
    // numbers::print_vector_arguement(num_vec);
    
    let num_array = [1,2,3,4,5];
    let num_vec = vec![6,7,8,9,10];
    
    // numbers::print_vector_arguement(num_vec);
    numbers::print_slice_arguement(num_array);
    numbers::print_slice_arguement(num_vec);
}
