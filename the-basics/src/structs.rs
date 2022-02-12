

// Struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Initialize
fn initialize_struct() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}

// When params have the same name you can just pass them!
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

// Updating
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

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Method (uses "&self")
    fn details(&self) -> String {
        String::from(&self.last_name)
    }
    // Associated function (does not use "&self")
    fn more_details() -> String {
        String::from("nonsense")
    }
}

fn example() {
    let george = Person {
        first_name: String::from("George"),
        last_name: String::from("Lopez"),
    };
    println!("{}", george.details());
    // **
    println!("{}", Person::more_details());
}