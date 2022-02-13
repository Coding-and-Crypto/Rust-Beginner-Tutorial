

// References
fn references() {

    let mut original_value: i32 = 20;

    // Borrow the original value to do some stuff
    let x = &original_value; // & is a reference

    // We can't do this operation because a reference to "a" is required later in the program
    // Rust's **borrower compiler** guarantees your reference won't change
    // If we conducted this operation, we would re-allocate this variable's location on the Heap
    //   and x would point to something else or nothing
    original_value = original_value*3;

    println!("{}", x)
}

