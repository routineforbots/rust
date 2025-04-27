struct Item {
    quantity: i32,
    id: i32,
}

fn display_quantity(i: &Item) {
    println!("The quantity of an item: {:?}", i.quantity);
}

fn display_id(j: Item) {
    println!("The id number of an item: {:?}", j.id);
}

fn main() {
    let my_item = Item{quantity: 3, id: 5};

    display_quantity(&my_item);

    display_id(my_item);

}

