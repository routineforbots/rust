struct Person {
    name: String,
    age: i32,
    fav_color: String,
}
 
fn print_item(name: &str, color: &str) {
    println!("name: {} \nfavorite number: {} \n \n", &name, &color);
}

fn main() {

    let people = vec![
        Person {
            name: "Alice".to_owned(),
            age: 5,
            fav_color: String::from("pink"),
        },
        
        Person {
            name: "Bob".to_owned(),
            age: 49,
            fav_color: String::from("red"),
        },

        Person {
            name: "Mike".to_owned(),
            age: 9,
            fav_color: String::from("blue"),
        },

    ];

    for item in people {

        if item.age <= 10 {
            print_item(&item.name, &item.fav_color);
        }
    }



}