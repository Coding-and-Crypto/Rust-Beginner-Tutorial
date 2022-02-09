// Processing a guess:
//  Ask for user input
//  Process that input
//  Check that the input is as expected

use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn guessing_game() {

    loop {
        println!("Guess a number:");

        let secret_number = rand::thread_rng().gen_range(1, 10);

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        println!("Your guess: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }

}


fn main() {
    guessing_game()
}
