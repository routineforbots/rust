enum Flavors {
    Sweet,
    Bitter,
    Sour,
    Salty,
    Spicy,
}



struct Drink {
    title: String,
    flavor: Flavors,
    fluid_ounce: f64,
}

fn print_drink_flavors (i: Drink) {
    
    println!("{}", i.title);

    match i.flavor {
        Flavors::Sweet => println!("sweet"),
        Flavors::Bitter => println!("bitter"),
        Flavors::Sour => println!("sour"),
        Flavors::Salty => println!("salty"),
        Flavors::Spicy => println!("spicy"),
    };
    println!("oz: {:?}\n", i.fluid_ounce);
}



fn main() {

    let mochito = Drink{
        title: "Mochito".to_string(),
        flavor: Flavors::Sweet,
        fluid_ounce: 1.0,
    };

    let espresso = Drink {
        title: "Espresso".to_string(),
        flavor: Flavors::Bitter,
        fluid_ounce: 1.5,
    };

    let lemonade = Drink {
        title: "Lemonade".to_string(),
        flavor: Flavors::Sour,
        fluid_ounce: 12.0,
    };

    let margarita = Drink {
        title: "Margarita".to_string(),
        flavor: Flavors::Salty,
        fluid_ounce: 8.0,
    };

    let bloody_mary = Drink {
        title: "Bloody Mary".to_string(),
        flavor: Flavors::Spicy,
        fluid_ounce: 10.0,
    };

    print_drink_flavors(mochito);
    print_drink_flavors(espresso);
    print_drink_flavors(lemonade);
    print_drink_flavors(margarita);
    print_drink_flavors(bloody_mary);

  
}

