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
