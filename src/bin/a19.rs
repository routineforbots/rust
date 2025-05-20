use std::collections::HashMap;

#[derive(Debug)]
enum Furniture {
    Chairs,
    Beds,
    Tables,
    Couches,
}

#[derive(Debug)]
struct Stock {
    item: Furniture,
    amount: i32,
}


fn main() {

let mut in_stock = HashMap::new();

in_stock.insert(1, Stock {item: Furniture::Chairs, amount: 5 });
in_stock.insert(2, Stock {item: Furniture::Beds, amount: 3 });
in_stock.insert(3, Stock {item: Furniture::Tables, amount: 2 });
in_stock.insert(4, Stock {item: Furniture::Couches, amount: 0 });

let mut total_amount = 0;

println!("\nIn stock: \n");

for value in in_stock.values() {
    if value.amount == 0 {
        println!("{:?} are out of stock", value.item);
    } else {
        println!("{:?} - {:?}", value.item, value.amount);
        total_amount += value.amount;
    }
    
}
println!("\nTotal amount of items is: {:} \n", total_amount);


}