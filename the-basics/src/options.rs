// Rust does not have null
// Instead we use Options
enum Option<T> {
    Some(T),
    None,
}

fn testing_options() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = Option::None;
}