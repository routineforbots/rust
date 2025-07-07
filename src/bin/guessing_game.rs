use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number");

    let secret = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret);

    println!("Enter yours: ");

    let mut guess = String::new(); // we need a variable to be mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Line was empty");

    println!("You guessed: {}", guess);
}
