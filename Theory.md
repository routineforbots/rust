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

- qssign data to a temporary memory location

- qllows programmer to easily work with memory

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

Working with the Data | enum
