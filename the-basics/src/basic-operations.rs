

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
        println!("True!")
    } else if number == 5 {
        println!("Ok!")
    } else {
        println!("False!")
    }
}

// Loops
fn while_loop() {
    loop {
        println!("Infinite loop!");
    }
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
        counter += 1
    };
}
