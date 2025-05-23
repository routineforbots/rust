# Data Types

- memory only store binary data

- anything can be represented in binary

- program determines what the binary represents

- basic types that are universally useful are provided by the language

## Basic Data Types

Boolean:

> true, false

Integer:

> 1, 2, 50, 99, -2

Double / Float:

> 1.1, 5.5, 200.0001, 2.0

Character:

> 'A', 'B', 'c', '6', '$'

String:

> "Hello", "string", "this is a sting", "it's 42"

# Variables

- assign data to a temporary memory location

- allows programmer to easily work with memory

- can be set to any value & type

- immutable by default, but can be muttable

  -- immutable: cannot be changed (are faster since there is no need to check for var changes)

  -- mutable: can be changed

## Variables examples

```rust
let two = 2; // let keyword is used to create variable

let hello = "hello"; // this is string

let j = 'j'; // and this is character

let my_half = 0.5;

let mut my_name = "Bill"; // mut keyword means this variable is mutable

let quite_program = false; // boolean variable

let your_half = my_half; //

```

# Functions

- a way to encapsulate program functionality

- optionally accept data

- optionally return data

- utilized for code organization and makes code easier to read

## Function example

```rust
fn main() {
    let x = add(1, 7);
    print!("{x}");
}


fn add(a: i32, b: i32) -> i32 { // i32 means type of function parameter, 32 bit integer; -> i32 means type of data returned
    return a + b;
}

```

## Println macros

- macros expands into additional rust code

```rust
let life = 42;
println!("hello"); // ! sign indicates that we use macro instead of function

println!("{:?}", life); // {:?} - we take external value (life) and include it into the macro using token :? which indicates that we want to use DEBUG PRINT -> 42

println!("{:?} {:?}", life, life); // we use DEBUG PRINT for both "life" variables (=twice) -> 42 42

println!("the meaning is {:?}", life); // here we use {:?} to substitute "life" variable's value (42) into the initial string -> the meaning is 42

// other ways:
println!("{life:?}"); // direct declaration of printing DEBUG version

println!("{life}"); // printing out END USER DISPLAY version instead of DEBUG version

```

# Control flow using "if" statement

- code executed line-by-line

- actions are performed & control flow may change

- specific conditions can change control flow: "if", "else", "esle if""

## Example for "if" and "else"

![](/pics/if-else.png)

```rust
let a = 99;

if a > 99 { // condition evaluates statement for true or false

println!("Big number");

} else {

println!("Small number);
}
```

## Example for nested if..else statement

![](/pics/nested_if-else.png)

```rust
let a = 99;

if a > 99 {
  if a > 200 {
    println!("Huge number");
  } else {
    println!("Big number");
  }
} else {
  println!("Small number");
}

```

## Example for if..else if..else statement

- if and else..if are logically on the same level!

- try to always include "else", unless there truly is no alternative case

![](/pics/if-else_if-else.png)

```rust
let a = 99;

//

if a > 200 { // if this statement is false then execution is switched to else if
  println!("Huge number");
} // if true then execution jumps out of all conditions checking

else if a > 99 { // same - if true then execution jumps out of all other conditions checking

  println!("Big number");

} else { // will be executed if previous 2 checks are false

  println!("Small number);

}

```

# Repetition using loops

- Called "looping"or "iterations"

- multiple types of loops available:

  -- "loop" - infinite loop

  -- "while" - conditional loop

- both types of loops can exit using "break" keyword

![](/pics/loop.png)

```rust
fn main() {

    let mut a = 0;

    loop {
        if a == 5 {
            break;
        }

        println!("{:?}", a);
        a += 1;
    }
}

```

![](/pics/while.png)

```rust
let mut a = 0;

while a != 5 {
  println!("{:?}, a");
  a += 1;
}
```

# How to compile and run rust code

1st we need to initialize cargo project

```bash
cargo init
```

2nd we build from the source file located in ./src/bin/a1.rs

```bash
cargo build --bin a1
```

3rd step - we run it. But actually this command builds & runs so 2nd step can be skipped

```bash
cargo run --bin a1

```

# Basic Arithmetics Examples

```rust
fun sub(a: i32, b: i32) -> i32 {
    a - b;
}

fn main() {

    let sum = 2 + 2;

    let value = 10 - 5;

    let division = 10 / 2;

    let mult = 5 * 5;

    let five = sub(8 , 3);

    // useful for checking if number is odd OR even
    let remainder_div = 6 % 3; // 4

    let rem2 = 6 % 2; // 2
}
```

# Basic Math Examples

```rust
fn main() {
let d = sum_numbers(5, 3);

display_sum_result(d);

}

fn sum_numbers(a: i32, b: i32) -> i32 {

    return a + b;
}

fn display_sum_result(c: i32) {

    println!("{:?}", c);

}
```

# Control flow with if & else

```rust
fn main() {

    let age = 18;

    if age >= 21 {
        println!("you can buy it");
    } else {
        println!("you are too young");
    }
}
```

another example

```rust
fn main() {

    let input = true;

    if input {

        println!("hello");

    } else {

        println!("goodbye");

    }

```

example for if & else..if

```rust
fn main() {

    let input = 7;

    if input < 5 {

        println!("<5");

    } else if input == 5 {

        println!("=5");

    } else {
        println!(">5");
    }

}
```

# Match Expressions

- add logic to program

- similar to if..else

- must be exhaustive == all possible values must be coded in it's logic (think switch)

- match will be checked by the complier and if a new possiblity is added, you will be notified when this occurs

- else..if is not checked by the compiler and code may contain a bug, if a new posibility is added

```rust
fn main() {
  let some_bool = true;

  match some_bool {
    // match works on expressions and not statements - that is why comma is used instead of ;
    true => println!("its true"),
    false => println!("its false"),

  }

}

```

another example

```rust
fn main() {
  let some_int = 3;
  match some_int {
    1 => println!("its 1"),
    2 => println!("its 2"),
    3 => println!("its 3"),
    _ => println!("its something else"), // sign _ means every possible value

  }

}

```

making decisions with match

```rust
fn main() {

  let my_name = "Bob";

  match my_name {
    "Jayson" => println!("That is my name"),
    "Bob"=> println!("Not my name"),
    "Alice" => println!("Hello Alice"),
    _ => println!("Nice to meet you"),
  }
}

```

basic match expressions

```rust
fn main() {

    let my_bool = false;


    match my_bool {

        true => println!("it's true"),
        false => println!("it's false"),
    }

}

```

basic match expressions 2

```rust
fn main() {
    let my_var = 3;

    match my_var {

        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }

}

```

repetition using loop

```rust

fn main() {

  let mut my_increment = 3;

  loop {

    println!("{:?}", my_increment);

    my_increment -= 1;

    if my_increment == 0 {
      println!("loop done");
      break;

    }
  }
}

```

another repetition using loop

```rust
fn main() {

    let mut my_var = 1;

    loop {
        println!("{:?}", my_var.to_string());

        if my_var == 4 {
            break;
        }

        my_var += 1;

    }
}

```

Repetition using while

```rust
fn main() {
    let mut counter = 1;

    while i <= 3 {

        println!("{:?}", i);

        i += 1;
    }


}

```

# Working with the Data | enum

- enumeration is a data that can be one of multiple different possibilities

- custom enum provides possiblity to create your own type in Rust

- each possibility is called a "variant"

- provides information about your program to the compilers (= more robust programs when paired with match expression)

![](/pics/enum.png)

```rust

enum Direction { // created outside of main function
    Left,
    Right

}


fn main() {

    let go = Direction::Left;

    match go {
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
    }

}

```

Custom enum example

```rust
enum Colours {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet
}

fn print_colour(i: Colours) {
      match i {
        Colours::Red => println!("Red"),
        Colours::Orange => println!("Orange"),
        Colours::Yellow => println!("Yellow"),
        Colours::Green => println!("Green"),
        Colours::Blue => println!("Blue"),
        Colours::Indigo => println!("Indigo"),
        Colours::Violet => println!("Violet"),
    }


}

fn main() {
    let my_colour = Colours::Indigo;

    print_colour(my_colour);


}

```

# Working with the Data | struct

- data type that contains multiple pieces of data, each piece of of data is called a "field"

- all fileds must be present to create a struct, fields can beaccessed using a dot (.)

- makes with data easier and similar data can be grouped together

![](/pics/struct.png)

Struct example 1

```rust
struct GroceryItem {
  stock: i32,
  price: f64,
}

fn main() {
  let cereal = GroceryItem {stock: 3, price: 10.11};

  println!("stock: {:?}", cereal.stock);

  println!("price: {:?}", cereal.price);

}

```

Struct example 2

```rust

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

```

# Working with the Data | Tuples

- type of record which consists of "lines of information on a piece of paper"

- tuples is way to access each piece of data within that line OR within that record

- data is stored anonymously and there is no need to name fields

- used to return pairs of data from functions

- can be "destructured" easily into variables

- can contain any number of fields but it is better to use struct when more than 2 or 3 fields

![](/pics/tuples.png)

```rust
fn main() {
  let coord = (2, 3);
  println!("{:?} - {:?}", coord.0, coord.1);

  // how to destructure
  let (x, y) = (2, 3);
  println!("{:?} - {:?}", x, y);

  let (name, age) = ("Emma", 20);

  // sometimes it is better to make structs instead of tuples to not loose context access tuples elements with X.NUMBER notation
  let favourites = ("red", 14, "pizza", "home");

  let food = favourites.2;

}
```

# Expressions

- rust is an expression-based language which means most things are evaluated and return some value

- expression value coalesce to a single point which can be used for nesting logic

![](/pics/expressions.png)

![](/pics/expressions2.png)

![](/pics/expressions3.png)

Example 1

```rust
enum Access {
    Admin,
    Manager,
    User,
    Guest
}

fn main() {
    // user access level
    let access_lvl = Access::Guest;

    // checking if user has needed access level
    let can_access_file = match access_lvl {
        Access::Admin => true,
        _ => false,
    };

    println!("can access: {:?}", can_access_file);

}

```

Example 2

```rust

fn print_msg (gt_100: bool) {
    match gt_100 {
        true => println!("its big"),
        _ => println!("its small")
    }

}


fn main() {

    let input_variable = 101;

    let is_gt_100 = input_variable > 100;

    print_msg(is_gt_100);

}

```

# Intermediate Memory

- memory is stored using binary: 0 or 1 bit

- computer is optimized for bytes: 1 byte = 8 bit

- memory uses addresses & offsets

- addresses are permanent, data differs

- all data in memory has an address

  -- used to locate data

  -- always the same, only data changes

- usually don't utilize address directly - variables handle most of the work

- memory offsets can be used to locate items at the address => "index" into some data

- offsets always begin at 0

- they represent the number of bytes away from the original address

  -- normally deal with indexes instead of offsets directly

![](/pics/addresses_offsets.png)

Example 2:

![](/pics/addresses_offsets2.png)

- location here is: Address 16, Offset 3

- variable (data) will automatically handle the address

- index [4] will automaticlly get mapped to the offset by the compiler

<br>

# Ownership

- it allows rust to execute code in a preformant manner

- ensures that compiled code executes correctly under various circumstances

- all programs must track their memory usage otherwise "leak" occurs

- rust utilizes an ownership model to manage memory

- the owner of memory is responsible for cleaning up the memory; in rust owner is a function

- cleaning occurs automatically at the end of the scope

- cleaning is also call "a drop"

- memory can either be "moved"or "borrowed"

- default behaviour is to "move" memory to a new owner

## Move memory

Example:

![](/pics/ownership.png)

the program here will not compile since we are calling "display_light(dull)" twice because of the ownership model:

- when we create "Light::Dull" and assign it to "dull" variable it is onwed by the main() function -> let dull = Light::Dull;

- when we call "display_light" with "dull", this "Light::Dull" moves into a new function (defined outside of "display_light" function) -> display_light(dull);

- now that the data has been moved into the "display_light" function, it is now owned by this function

- any function which owns data is required to delete the data once the function completes (deleted after first "display_light" function finishes)

- so it is no longer available to use in the same function call

- to fix this if we still want to call this function twice we need to use "Borrow" instead of "Moving" (& sign before vaiables):

## Borrow memory

![](/pics/ownership2.png)

- with ampersand "&" sign we indicated that we borrowing data OR referencing data

- in our example ownership will not transfer from main() function (owner) and will not be deleted

- it is needed is the memmory efficency - imagine you need to copy huge variable each time instead of referencing

## Ownership examples

```rust
struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("pages = {:?}", book.pages);
}

fn display_rating(book: &Book) {
    println!("rating = {:?}", book.rating);
}


fn main() {
    let book = Book {
        pages: 5,
        rating: 9,
    };

    display_page_count(&book);
    display_rating(&book);
}

```

```rust
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

```

# Keyword impl

- impl allows you to implement functionality on specific enumerations & structs
- helps to better organize the code

```rust
// example of impl usage to resturcture the code

struct Temperature {
    degrees_f: f64,
}

/* old location of a code block
fn snow_temp(temp: Temp) {
    println!("{:?} degrees F", temp.degrees_f);
}
*/


// this extends struct Temperature with additional functionality
impl Temperature {
    fn show_temp(temp: Temperature) {
    println!("{:?} degrees F", temp.degrees_f);
    }
}


fn main() {
    let hot = Temperature {degrees_f: 99.9};
//    show_temp(hot);
    Temperature::show_temp(hot);
}
```

additional way to optimize this code it is to add reference to itself - Temperature struct here:

```rust

struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn show_temp(&self) { // &self means that we already implemented is somewhere in a program

    println!("{:?} degrees F", self.degrees_f); // we can now directly address to the struct's variables
    }

    fn freezing() -> Self { // Self here means we creating a new one or refering to this specific object
        Self {
            degrees_f: 32.0
        }
    }
}

fn main() {
    let hot = Temperature {degrees_f: 99.9};
//    show_temp(hot);
    hot.show_temp(); // and now this is how we can call the function on Temperature types - directly!

    let cold = Temperature::freezing();
    cold.show_temp();
}

```

Another example how we can reshape the following code with impl keyword

```rust
struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Colors,
}

impl Box {

    fn new(weight: f64, color: Colors, dimensions: Dimensions) -> Self {

        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
        println!("\n");

    }

}


enum Colors {
    Black,
    Red,
    Blue,
}

impl Colors {
    fn print(&self) {
        match self {
            Colors::Black => println!("color: black"),
            Colors::Red => println!("color: red"),
            Colors::Blue => println!("color: blue"),
            _ => println!("color was not defined"),
        }
    }

}


struct Dimensions {
    width: f64,
    height: f64,
    dep: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("dep: {:?}", self.dep);
    }
}


fn main() {
    let small_dimensions  = Dimensions {
        width: 1.0,
        height: 2.0,
        dep: 3.0,
    };

    let small_box = Box::new(5.0, Colors::Red, small_dimensions);
    small_box.print();


    let medium_dimensions  = Dimensions {
        width: 10.0,
        height: 20.0,
        dep: 30.0,
    };

    let medium_box = Box::new(23.3, Colors::Black, medium_dimensions);
    medium_box.print();


    let large_dimensions  = Dimensions {
        width: 100.0,
        height: 200.0,
        dep: 300.0,
    };

    let large_box = Box::new(199.3, Colors::Blue, large_dimensions);
    large_box.print();

}


```

# Data Types | Vector

- multiple pieces of data, must be the same type => are used to organize similar data within the code

- used for lists of information

- can add, remove and traverse the entries

- the vec! macro can be used to make vectors

- use for..in to iterate through items of a vector

![](/pics/vectors.png)

```rust
fn main() {
    // vector creation with vec macro
    let my_numbers = vec![1, 2, 3];

    println!("{:?}", my_numbers); // [1, 2, 3]

    // alternative way to define vector
    let mut my_numbers = Vec::new();

    // pushing
    my_numbers.push(1);
    my_numbers.push(2);
    my_numbers.push(3);

    // removes last element from the vector - 3
    my_numbers.pop();

    // getting number of items from the vector
    my_numbers.len(); // this is 2

    println!("{:?}", my_numbers);

    // accessing vector items via slice notation with indexes, starting from 0
    let two = my_numbers[1]; // 2

    // vector of predefined size 5 filled with default values 0
    let my_numbers2 = vec![0; 5]; // [0, 0, 0, 0, 0]

    // iteration through vector
    let my_numbers = vec![1, 2, 3];

    for num in my_numbers {
        println!("{:?}", num)
    }


}
```

Another example of vector usage

```rust
struct Test {
    score: i32
}


fn main() {
    // lets create a vector with several Test objects
    let my_scores = vec![
        Test { score: 90 },
        Test { score: 88 },
        Test { score: 77 },
        Test { score: 93 },
    ];

    for num in my_scores {
        println!("{}", num.score);
    }

}

```

Additional example of vector usage

```rust


fn main() {

    let my_vector = vec![ 10, 20, 30, 40 ];

    // for loop gets the ownership of my_vector so we need to borrow it otherwise it will be deleted after loop execution is completed
    for num in &my_vector {
        /*
        // one way to do that:

        if num == 30 {
            println!("thirty");
        } else {
            println!("{:?}", num);
        }

        */

        match num {
            30 => println!("thirty"),
            _ => println!("{:?}", num),
        }

    }
    println!("\n");

    println!("Total number of elements in a vector: {}", my_vector.len());

}

```

# Data Types | Strings

- Two types are commonly used:

  - String - owned

  - &str - borrowed String slice

- Use .to_owned() or String::from() to create an owned copy of a string slice

- Must use an owned String to store in a struct

- Use &str when passing to a function

![](/pics/strings1.png)

```rust
fn print_it(data: &str) {
    println!("{:?}", data);
}

fn main() {
    print_it("a string slice"); // by default string created like that is automatically Borrowed

    // two options on how to create owned string:
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another");

    // and now we need to borrow them
    print_it(&owned_string);
    print_it(&another_owned);
}
```

Another example

```rust
struct Employee {
    name: String,
}

fn main() {

    let emp_name = "Jayson".to_owned();

    // alternative way:
    // let emp_name = String::from("Jayson");

    let emp = Employee {
        name: emp_name
    };
}
```

And additional example

```rust
struct LineItem {

    name: String,
    count: i32,
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(), // since struct requires owned string
            count: 3,
        },
        LineItem {
            name: String::from("fruit"), // same
            count: 2,
        },


    ];

    for item in receipt {
        println!("name: {}, count: {}", item.name, item.count);
    }
}

```

<br>

# Derive macro

- functionality can be automatically implemented for enums and structs with Derive macro
- special macro that is applied to enum and struct
- all fields within the Derived element must be also Derived
- there are several Derives that can be used: Debug, Clone, Copy

## Derive macro | Debug

```rust
#[derive(Debug)] // derive must be put right before the element - enum here
enum Position {
    Manager,
    Supervisor,
    Worker
}

#[derive(Debug)] // derive must be put right before the element - struct here
struct Employee {
    position: Position,
    work_hours: i64
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40

    };

    println!("{:?}", me.position);// Worker
    println!("{:?}", me); // Employee { position: Worker, work_hours: 40 }

}
```

## Derive macro | Clone & Copy

- allows compiler to automatically make a copy when you store it in struct OR function
- this means that ownership is no longer transferd when you Move and Copy is made instead
- shouldn't be used on big enums and structs OR multiple times -> memory optimizations

```rust
#[derive(Debug, Clone, Copy)] // derive must be put right before the element - enum here
enum Position {
    Manager,
    Supervisor,
    Worker
}

#[derive(Debug, Clone, Copy)] // derive must be put right before the element - struct here
struct Employee {
    position: Position,
    work_hours: i64
}

fn print_employee(emp: Employee) { // notice that we don't Borrow here (no & sign)
    println!("{:?}", emp);
}


fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40

    };

    print_employee(me);
    print_employee(me); // we didn't borrow Employee but since we used Clone, Copy there will be no error - two copies of Employee were created

}

```

# Type Annotations

- are mostly optional within function bodies

- required for function signatures:

```rust
 fn add(a: i32, b: i32) -> i32 { // i32 types is explicitly defined in a "function signature"
    a + b
}
```

- types are usually inferred by Rust compiler and you don't need to explicitly specify types for variables

- can also be specified in code -> explicit type annotations

```rust
let x: i32 = 5;
let name: String = String::from("John");
let a: char = 'a';
```

- usage of generics (like vector)

```rust
enum Mouse {
    LeftClick,
    RightClick,
}

// we can explicitly define elements for vector
let numbers: Vec<i32> = vec![1, 2, 3];
let letters: Vec<char> = vec!['a', 'b'];
let clicks: Vec<Mouse> = vec![
    Mouse::LeftClick,
    Mouse::RightClick,
    Mouse::LeftClick,
]
```

# Enum revisited

- enum is a type that can represent one item at a time

- each item is called a variant

- enum is not limited to just plain variants, each variant can contain additional data

- enum variants can optionally contain data; the data can be another enum

- can mix plain identifiers and data-containing variants within the same enum

- more that one piece of data can be associated with a variant

```rust
enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32),
    Move(i32, i32),

}

enum PromoDiscount {
    NewUser,
    Holiday(String), // optionally contain String
}

enum Discount {
    Percent(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String), // optionally contain String
}

```

# Advanced match

```rust
enum Discount {
    Percent(i32),
    Flat(i32)
}

struct Ticket {
    event: String,
    price: i32,

}

// better way to refer to anything else variant

fn main() {
    let n = 100;
    match n {
        3 => println!("three"), // if it is 3
        other => println!("number: {:?}", other), // if it something else; we can refer to that something else as the "other" variable
        // _ => println!("nothing", n), // _ means anything else but we ignore any value; the best way is to do it is with "other" example
    }

    let flat = Discount::Flat(2);
    // how to match on enum that have extra data on a variants
    match flat {
        Discount::Flat(2) => println!("flat 2 "),
        Discount::Flat(amount) => println!("flat discount of {:?}", amount), // match anything and it will be accessible the variable name
        _ => (), // this is how we can ignore everything else and will return nothing
    }

    // matching on a struct
    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50,
    };

    match concert {
        // matching on specific price and capturing the event name:
        Ticket {price: 50, event} => println!("event @ 50 {:?}", event),

        // matching on price only
        Ticket {price, ..} => println!("price = {:?}", price),
    }
}

```

# Working With Optional Data with Option Type

- it may be some data of specified type OR nothing

  - Some(variable_name) - means data is available
  - None - means no data available

- used when data may not be required or os unavailiable

  - unable to find something
  - ran out of items in a list
  - form field not filled out

- Use Option<type> to declare an optional type

```rust

// the Option type is part of rust programming library and is defined as enum with 2 variants:
enum Option<T> { // <T> means Option contains some type but we don't know which one
    Some(T), // 1st variable Some represents some data
    None // 2nd variable None represents no data
}

```

![](/pics/option.png)

![](/pics/option2.png)

```rust
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
                // early return from a function with return keyword
                return Some(item.qty); // some represents the variants within the Option type
            }
        }
        None // implicit return from a function
    }

    let test = find_quantity("meat");
    println!("{:?}", test);
}

```

# Documentation

- can be generated automatically with 3 slashes /// - documentation comment
- after doc comments are place use cargo doc --open
- better to use Chrome

```rust
/// A favorite color
enum Color {
    Red,
    Blue,
}
/// A piece of mail
struct Mail {
    /// The destination address
    address: String,
}

/// Adds two numbers together
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main () {


}
```

# Standard Library in Rust

- can be accessed with the command:

```bash
rustup doc
```

- and section in browser "Rust API documentation"

- example: is_empty method of vector can be found in

```html
Module vec -> Structs Vec
```

```rust
fn main() {

    let numbers = vec![1, 2, 3];

    match numbers.is_empty() {
        true => println!("vec is empty"),
        false => println!("vec is NOT empty"),
    }
}


```

# Working with Data

- Result - is a data type that contains one of two types of data:
  -- "Successful" data with Ok(variable_name)
  -- "Error" data with Err(variable_name)

- used in scenarious where an action needs to be taken, but has the possibility of failure, like:
  -- copying a file
  -- connecting to website

- result is defined in a following way:

```rust
enum Result<T, E> { // T and E are used as placeholders for any types
    Ok(T), // Ok varian means operation was completed
    Err(E) // Err means falure
}
```

Example 1

```rust
// SoundData is a dummy data type here
    fn get_sound(name: &str) -> Result<SoundData, String> { // if result is OK it will return SoundData; Error will be return with a string with details
        if name == "alert" {
            Ok(SoundData::new("alert")), // function was successful and returns
        } else {
            Err("unable to find sound data".to_owned()) // otherwise we return error with a string
        }
    }

    let sound = get_sound("alert");
    match sound {
        Ok(_) => println!("sound data located"),
        Err(e) => println!("error: {:?}", e), // this is where we capture the error message as e and print it
    }
```

Example 2

<span style="background-color:rgb(140, 0, 255)">
В Rust оператор ? используется для лаконичной обработки ошибок при извлечении значения из Result или Option. В частности, когда вы используете ? для значения Result<T, E>, он делает одно из двух:

- Если Result — это Ok(value), он извлекает и возвращает value (типа T).
- Если Result — это Err(error), он немедленно возвращает Err(error) (типа E) вызывающему коду, передавая ошибку вверх по стеку вызовов.

Это особенно полезно в функциях, которые сами возвращают тип Result, позволяя избежать явного сопоставления с помощью match или unwrap.

</span>

```rust
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit
}

// Function returns Ok or Err: MenuChoice will be wrapped in Ok, String will be wrapped in Err
fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("no such option available".to_owned()), // catch all options
    }

}

fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> { // Ok variant here will be nothing, defined with unit type: ()
    let choice: MenuChoice = get_choice(input)?; // ? operator here performs match operation:
    // for output of get_choice() function:
    // if Result is Ok variant - inner data will be placed in that choice
    // an if it is Err variant - the error will be returned as the Err message from the function
    print_choice(&choice);
    Ok(()) // this will be returned as nothing defined as unit type
}

fn main() {
/*
    let choice: Result<MenuChoice, _> = get_choice("start"); // _ here means anything so that compiler can figure it out that it is a String

    // this is one of the ways how we can access data from the Result output
    match choice {
        Ok(inner_choice) => print_choice(&inner_choice),
        Err(e) => println!("error = {:?}", e),
    }
*/
    // another way will be with ? mark operator:
    let choice = pick_choice("re"); // choice variable is always gonna be a Result, Ok or Err
    println!("choice value = {:?}", choice);


}

}
```

![](/pics/result.png)

# Data Structures | Hashmap

- usefull for storing and retrieving information when you know what you are looking for (= have a key)
- hashmap is a collection that stores data as key-value pairs
  -- data is located and accessed using the "key"
  -- the data is the "value"
- similar to definitions in a dictionary
- very fast to retrieve data using the key
- data is stored in random order (differs from vectors where everything comes in the same order as was placed)

## Find the data in hashmap

- first value in () is the key and second one is the actual data:
- method get(key) will retrive the data based on provided key;

```rust
people.insert("Susan", 21);

people.get("Susan");
```

![](/pics/hashmap.png);

## Iterate on hashmap

- method iter() is used to get both keys and values in a tuple
- method keys() is used to get keys only
- method values() is used to get values only

![](/pics/hashmap2.png);

Example

```rust
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



```
