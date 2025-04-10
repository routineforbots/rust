enum Colours {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet
}

fn main() {
    let my_colour = Colours::Indigo;

    match my_colour {
        Colours::Red => println!("Red"),
        Colours::Orange => println!("Orange"),
        Colours::Yellow => println!("Yellow"),
        Colours::Green => println!("Green"),
        Colours::Blue => println!("Blue"),
        Colours::Indigo => println!("Indigo"),
        Colours::Violet => println!("Violet"),
    }

}