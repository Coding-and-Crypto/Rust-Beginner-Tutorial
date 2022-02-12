
// Can't overwrite an immutable var
fn immutable() {
    let x = 5;
    // x = 6;
}

// Must overwrite with "let"
fn overwrite() {
    let x = 5;
    let x = 6;
}

// Can't mutate a variable's type
fn mutate_type() {
    let some_string = "donut";
    // some_string = some_string.len();
}

// Must mutate with "let"
fn overwrite_type() {
    let some_string = "donut";
    let some_string = some_string.len();
}

// Declaring data types
fn declare() {
    let x: u32 = 20;
}

// Integers
fn integer() {
    let signed_int: i64 = 34;
    let unsigned_int: u16 = 11; 
}

// Strings
fn strings() {
    let some_string = "donut"; // Slice (str)
    let some_other_string: String = String::from("donut"); // String Object (String)
}

// Slices
fn slices() {
    // let some_string: str = "donut";
    let some_other_string: &'static str = "donut";
}

// Constants
const SOME_CONSTANT: u32 = 20;

// Function returns
fn give_me_slice() -> &'static str {
    return "pizza"
}
fn give_me_string() -> String {
    String::from("pizza")
}
fn give_me_int() -> u32 {
    4
}

// Conditionals
fn conditional() {
    let number = 3;

    if number < 5 {
        println!("True")
    } else if number == 5 {
        println!("OK")
    } else {
        println!("False")
    }
}

// Loops
fn while_loop() {
    loop {
        println!("Infinite loop!!!");
    };
}
fn for_loop() {
    for n in 1..40 {
        println!("Hello #{}", n);
    }
}
fn loop_as_variable() {
    let mut counter = 0;
    let g = loop {
        if counter == 3 {
            break {
                "donut"
            };
        }
    };
}


fn main() {
    println!("The basics of Rust");
}
