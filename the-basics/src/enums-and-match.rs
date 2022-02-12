

// Match
fn match_example() {
    // TODO: Match example
}

// Enum
enum Direction {
    Up,
    Down
}

// Match with Enum
fn enum_example() {
    let player_direction: Direction = Direction::Up;
    match player_direction {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
    }
}