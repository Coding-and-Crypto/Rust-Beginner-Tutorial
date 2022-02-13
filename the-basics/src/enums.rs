

// Enum
enum Direction {
    Forward,
    Backward,
}

fn enum_example() {
    let player_direction: Direction = Direction::Forward;
}


// Enums also have methods

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

fn example() {
    let msg = Message.Move {x: 20, y: 40};
    println!(msg.call())
}