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

// Matching
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        CoinL::Quarter => 25,
    }