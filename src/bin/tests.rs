use std::collections::HashMap;


struct Contents {
    content: String,

}

fn main() {

    let mut lockers = HashMap::new();

    lockers.insert(1, Contents {content: "stuff".to_owned()});
    lockers.insert(2, Contents {content: "shirts".to_owned()});
    lockers.insert(3, Contents {content: "pants".to_owned()});

    for (key, value) in lockers {
        println!("the key: {:?} -- the value: {}", key, value.content)
    }

    

}