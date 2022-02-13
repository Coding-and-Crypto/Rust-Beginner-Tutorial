

// Enums
enum StopLight {
    Green,
    Yellow,
    Red,
}

// Matching
fn read_light(light: StopLight) -> String {
    match light {
        StopLight::Green => "Go!".to_string(),
        StopLight::Yellow => "Slow down!".to_string(),
        StopLight::Red => "Stop".to_string(),
    }
}

fn match_example() {
    println!("{}", read_light(StopLight::Green));
    println!("{}", read_light(StopLight::Yellow));
    println!("{}", read_light(StopLight::Red));
}


// Advanced Enums
enum GolfEvent {
    TeeUp, // Unit - The golfer is teeing up the ball
    Drive(String), // Function - The golfer drives the ball n yards
    ClubSelect {club: String, max_yard: String}, // Struct - The golfer selects a new club
}

fn golf_event(event: GolfEvent) -> String {
    match event {
        GolfEvent::TeeUp => "Golfer is teeing up the ball".to_string(),
        GolfEvent::Drive(yards) => "Golfer just hit the ball ".to_string() + &yards + &" yards!".to_string(),
        GolfEvent::ClubSelect{club, max_yard} => "Golfer has equipped ".to_string() + &club + &". Max yardage is ".to_string() + &max_yard,
    }
}

fn match_golf_example() {
    println!("{}", golf_event(GolfEvent::TeeUp));
    println!("{}", golf_event(GolfEvent::Drive("250".to_string())));
    println!("{}", golf_event(GolfEvent::ClubSelect{club: "6 Iron".to_string(), max_yard: "175".to_string()}));
}


// Enums have methods, too!
enum Direction {
    Forward,
    Backward,
}

impl Direction {
    // Method
    fn walk(&self) {
        println!("Walking...")
    }
    // Associated function
    fn run() {
        println!("Running!")
    }
}

fn methods_example() {
    let direction = Direction::Forward;
    println!("{:?}", direction.walk());
    println!("{:?}", Direction::run());
}
