// Can overwrite a variable with "let"
//
fn variable_mutability_example() {
    let x = 5;
    let x = x + 3;
    let x = x * 2;
    println!("{}", x);
}

// Can't mutate a variable's type
//
fn variable_immutable_type_example() {
    let some_string = "joe";
    let some_string = some_string.len();
    let some_other_string = "joe";
    some_other_string = some_other_string.len();
}

// Functions can just return numbers without computation
//
fn five() -> u32 {
    5
}

// Basic example of a conditional
//
fn conditional() {
    let number = 3;

    if number < 5 {
        println!("True")
    } else if number == 5 {
        println!("OK")
    } else {
        println!("False")
    }

    let result = loop {
        counter += 1;
        if counter == 10 {
            break {
                counter * 2
            };
        }
    };
    println!("{}", result)
}


fn main() {
    variable_mutability_example();
    variable_immutable_type_example();
    println!(five());
    conditional();


}
