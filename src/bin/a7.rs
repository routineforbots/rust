enum Colours {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet
}

fn print_colour(i: Colours) {
      match i {
        Colours::Red => println!("Red"),
        Colours::Orange => println!("Orange"),
        Colours::Yellow => println!("Yellow"),
        Colours::Green => println!("Green"),
        Colours::Blue => println!("Blue"),
        Colours::Indigo => println!("Indigo"),
        Colours::Violet => println!("Violet"),
    }


}

fn main() {
    let my_colour = Colours::Indigo;

    print_colour(my_colour);


}
