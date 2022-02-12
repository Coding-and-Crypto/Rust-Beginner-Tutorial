
use std::io;
use rand::Rng;


fn send_bitcoin() {
    println!("\nWe're going to send some Bitcoin!\n");

    let clients = vec!["Homer", "Marge", "Bart", "Lisa"];
    
    println!("Who do you want to send Bitcoin to?\n");
    
    for client in &clients {
        print!("{}  ", client);
    }
    println!("\n");

    let mut recipient = String::new();
    io::stdin().read_line(&mut recipient).expect("Error");

    println!("\nHow many Bitcoin?\n");

    if clients.contains(&recipient.trim()) {
        
        let mut amount = String::new();
        io::stdin().read_line(&mut amount).expect("Error");
        
        println!("\nYou sent {} Bitcoin to {}!\n", amount.trim(), recipient.trim());
        
    } else {
        println!("\n{} is not in your contacts!\n", recipient.trim());
    }

}


fn recieve_bitcoin() {
    println!("\nWe're going to recieve some Bitcoin!\n");
    
    let amount = rand::thread_rng().gen_range(1, 10);

    println!("You just recieved {} Bitcoin!\n", amount);
}


fn exit_console() {
    println!("Invalid entry! Can only be s or r");
    println!("\nThere has been an error, exiting module...\n")
}



fn console() {

    println!("\nLet's have fun with Bitcoin!\n");

    println!("Do you want to send (s) or recieve (r) Bitcoin?\n");

    let mut command = String::new();

    io::stdin().read_line(&mut command).expect("Error");

    if command.trim().eq("s")  {
        send_bitcoin()
    } else if command.trim().eq("r") {
        recieve_bitcoin()
    } else {
        exit_console()
    }

}


fn main() {
    console()
}
