
// Tuples
fn tuples() {
    let tup: (i32, i32, String) = (1, 2, String::from("Paul"));
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
    let mut tup: (i32, i32, String) = (1, 2, String::from("Paul"));
    tup.2 = String::from("John");
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
}

// Arays
fn arrays() {

    // Mutable
    let mut arr: [i32; 3] = [4, 5, 3];
    arr[0] = 1;
    arr[1] = 2;
    for x in &arr {
        println!("{}", x);
    }

    // Initialize with expression
    let exp_array: [i32; 5] = [0; 5];
    for x in &exp_array {
        println!("{}", x);
    }

    // Initialize variables with an array
    let [greg, mark] = ["Greg".to_string(), "Mark".to_string()];
    println!("{}", greg);
    println!("{}", mark);
}

// Vectors
fn vectors() {
    let vector: Vec<i32> = vec![1, 3, 7, -1];
    let mut vector: Vec<i32> = (0..10).collect();
    for x in &vector {
        println!("{}", x);
    }
    // Append
    vector.push(-2);
    println!("{}", vector[vector.len() - 1]);
    println!("{:?}", vector.pop());
}

// Iterating
fn iterating() {

    // Arrays
    let arr: [i32; 5] = [0; 5];
    //  By reference (.iter)
    for x in arr.iter() {
        println!("{}", x);
    }
    //  By reference (.iter) - enumerated
    for x in arr.iter().enumerate() {
        // gives you a tuple: (index, value)
        println!("{}", x.1);
    }
    //  By value (.into_iter)
    for item in arr.into_iter().enumerate() {
        // You can choose to do both index and value
        let (i, x): (usize, i32) = item;
        println!("Index: {}, Value: {}", i, x)
    }

    println!("-----------------");

    // Vectors - Iterate the same as Arrays
    let vector: Vec<i32> = (0..5).collect();
    for x in vector.iter() {
        println!("{}", x);
    }
    let mut mut_vector: Vec<i32> = (0..5).collect();
    for x in mut_vector.iter_mut() {
        *x += 3;
    }
    println!("{:?}", mut_vector);
}
