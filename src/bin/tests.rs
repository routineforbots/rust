struct Customer {
    age: Option<i32>,
    email: String,
}


fn main() {
    let mark = Customer {
        age: Some(22),
        email: "mark@example.com".to_owned(),
    };
    
    let becky = Customer {
        age: None,
        email: "mark@example.com".to_owned(),
    };

    match mark.age {
        Some(age) => println!("customer is {:?} years old", age),
        None => println!("customer age is not provided"),
    };

    match becky.age {
        Some(age) => println!("customer is {:?} years old", age),
        None => println!("customer age is not provided"),
    };

}