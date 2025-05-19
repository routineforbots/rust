#[derive(Debug)]
struct Adult {
    name: String,
    age: u8

}

impl Adult {
    fn new(_name: &str, _age: u8) -> Result<Self, (&str, u8)> { //Self means that Adult will be returned if everything is OK
        if _age >=21 {
            Ok(Self{name: _name.to_string(), age: _age})
        } else {
            //Err("younger than 21 y.o.".to_owned())
            Err((_name, _age))
        }

    }
}

fn print_result(input: Result<Adult, (&str, u8)>) {
    match input {
        Ok(person) => println!("created: {:}, {:}", person.name, person.age),
        Err((name, age)) => println!("not created: {:} is {:} which is less than 21", name, age),
    }
}


fn main() {
    let person1: Result<Adult, (&str, u8)> = Adult::new("Jake", 32);
    print_result(person1);


    let person2: Result<Adult, (&str, u8)> = Adult::new("Andy", 15);
    print_result(person2);

}