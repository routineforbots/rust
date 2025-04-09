// importing library to capture input
use std::io;

fn display_first_and_last_name() {
    let mut first_name = String::new();
    let mut last_name = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut first_name).expect("Failed to read name");

    println!("Enter you lastname: ");
    io::stdin().read_line(&mut last_name).expect("Failed to read last name");

    println!("Hello, {} {}!", first_name.trim(), last_name.trim());

}

fn main() {

    display_first_and_last_name();

}