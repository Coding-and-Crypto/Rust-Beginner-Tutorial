fn variable_mutability_example() {
    let x = 5;
    let x = x + 3;
    let x = x * 2;
    println!("{}", x);
}

fn variable_immutable_type_example() {

    // Can't mutate a variable's type

    let some_string = "joe";
    let some_string = some_string.len();
    let some_other_string = "joe";
    some_other_string = some_other_string.len();
}

fn five() -> u32 {
    5
}

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
