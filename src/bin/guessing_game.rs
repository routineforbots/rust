use std::io;

fn main() {
    println!("Guess the number");

    println!("Enter yours: ");

    let mut guess = String::new(); // we need a variable to be mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Line was empty");
}
