struct LineItem {

    name: String,
    count: i32,
}

fn print_name(name: &str) { // this function requires &str borrowed from struct
    println!("name: {}", name);
}


fn main() {
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(), // since struct requires owned string
            count: 3,
        },
        LineItem {
            name: String::from("fruit"), // same - struct requires owned string
            count: 2,
        },


    ];

    for item in receipt {
        print_name(&item.name); // this will create string slice borrowed from the String defined in struct
        println!("count: {}", item.count);
    }
}