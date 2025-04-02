# Data Types

- Memory only store binary data

- Anything can be represented in binary

- Program determines what the binary represents

- Basic types that are universally useful are provided by the language

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

- Assign data to a temporary memory location

- Allows programmer to easily work with memory

- Can be set to any value & type

- Immutable by default, but can be muttable

  -- Immutable: cannot be changed (are faster since there is no need to check for var changes)

  -- Mutable: can be changed

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

- A way to encapsulate program functionality

- Optionally accept data

- Optionally return data

- Utilized for code organization and makes code easier to read

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
