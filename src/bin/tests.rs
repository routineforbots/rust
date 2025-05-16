#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit
}

// Function returns Ok or Err: MenuChoice will be wrapped in Ok, String will be wrapped in Err
fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("no such option available".to_owned()), // catch all options
    }
    
}

fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> { // Ok variant here will be nothing, defined with unit type: ()
    let choice: MenuChoice = get_choice(input)?; // ? operator here performs match operation: 
    // if Result is Ok variant - inner data will be placed in that choice
    // an if it is Err variant - the error will be returned as the Err message from the function
    print_choice(&choice);
    Ok(()) // this will be returned as nothing defined as unit type
}

fn main() {
/*
    let choice: Result<MenuChoice, _> = get_choice("start"); // _ here means anything so that compiler can figure it out that it is a String

    // this is one of the ways how we can access data from the Result output
    match choice {
        Ok(inner_choice) => print_choice(&inner_choice),
        Err(e) => println!("error = {:?}", e),
    }
*/
    // another way will be with ? mark operator:
    let choice = pick_choice("re"); // choice variable is always gonna be a Result, Ok or Err
    println!("choice value = {:?}", choice);


}