fn main() {
    println!("{} days", 31); //just a default argument

    /* string formatting */
    let structure = "this is";
    let object = "car";
    let end = "of red color";
    println!("{} {} {}", structure, object, end);

    /* can also initialize the strings in a println */
    println!("{a} {b} {c}", a = "this", b="is", c = "a test." );
}
