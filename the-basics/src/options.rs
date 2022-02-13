
// Rust does not have null
// Instead we use Options
// enum Option<T> {
//     Some(T),
//     None,
// }

fn option_example() {
    let mut vector = vec![1, 2, 3];
    for i in (0..5) {
        let x = match vector.pop() {
            Some(val) => val,
            None => 0,
        };
        println!("{}", x);
    }
}

// Match
fn match_this(val: i32) -> String {
    match val {
        1 => "One".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        _ => "Not listed".to_string(),
    }
}

fn match_example() {
    println!("{}", match_this(1));
    println!("{}", match_this(2));
    println!("{}", match_this(3));
    println!("{}", match_this(4));
}
