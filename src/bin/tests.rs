#[derive(Debug)]
struct GroceryItem {
    name: String,
    qty: i32,
}
fn main() {
    fn find_quantity(name: &str) -> Option<i32> {
        let groceries = vec![
            GroceryItem {name: "bananas".to_owned(), qty: 4,},
            GroceryItem {name: "eggs".to_owned(), qty: 10,},
            GroceryItem {name: "bread".to_owned(), qty: 1,},
        ];
        for item in groceries {
            if item.name == name {
                return Some(item.qty); // early return from a function
            }
        }
        None // implicit return from a function
    }

    let test = find_quantity("meat");
    println!("{:?}", test);
}