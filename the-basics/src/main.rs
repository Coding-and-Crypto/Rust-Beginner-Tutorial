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

// A Constant
//
const SOME_CONSTANT: u28 = 20;

// Enum
//
enum Direction {
    Up,
    Down
}
fn enum_example() {
    let player_direction: Direction = Direction::Up;
    match player_direction {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
    }
}

// Struct
//
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn initialize_struct() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}

// When params have the same name you can just pass them!
//
fn initialize_struct_simple(email: String, username: String) -> User {
    User {
        email,
        // email: email,
        username,
        // username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn update_struct() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // Inherit the rest of the values from user1
    let user2 = User {
        email: String::from("someoneelse@example.com"),
        username: String::from("someotherusername123"),
        ..user1
    };
}

// Methods
//
struct Person {
    first_name: String,
    last_name: String,
}
impl Person {
    fn details(&self) -> String {
        String::from("{}, {}", &self.last_name, &self.first_name)
    }
}

// Enums also have methods
//
enum Message {
    Quit,
    Move {x: u64, y: u64},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call() -> String {
        String::from("Joe")
    }
}

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
}


fn main() {
    variable_mutability_example();
    variable_immutable_type_example();
    println!(five());
    conditional();


}
