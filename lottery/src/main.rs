// Processing a guess:
//  Ask for user input
//  Process that input
//  Check that the input is as expected

use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn lottery() {

    loop {
        println!("\n\nYou are now entering the lottery!\n\n");
        println!("Guess a number, 1 through 10:");

        let winning_number = rand::thread_rng().gen_range(1, 10);

        let mut your_number = String::new();

        io::stdin().read_line(&mut your_number)
            .expect("Failed to read line");

        println!("Your number: {}", your_number);

        let guess: u32 = match your_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number! Only 1 through 10!");
                continue;
            }
        };

        match guess.cmp(&winning_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("Congratulations, Winner!");
                break;
            }
        }
    }

}


fn main() {
    lottery()
}