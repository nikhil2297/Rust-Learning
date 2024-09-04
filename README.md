# Rust Learning Guide

## Table of Contents
- [Ownership Model for Memory Management](#ownership-model-for-memory-management)
- [Mutable and Immutable](#mutable-and-immutable)
- [Error Handling in Rust](#error-handling-in-rust)
- [Programming Concepts](#programming-concepts)
- [Guessing Game](#guessing-game)
  - [Phase 1: User Input](#phase-1-user-input)
  - [Phase 2: Generate Random Number and Compare](#phase-2-generate-random-number-and-compare)
  - [Phase 3: Looping and Break](#phase-3-looping-and-break)

## Ownership Model for Memory Management

In dynamic programming languages like JavaScript, scope management is used to control variable access. For example:

```javascript
function myFun() {
  const obj = {
    fname: "Nikhil"
  };
  
  printName(obj);
  
  console.log(obj);
}

function printName(obj) {
  obj.fname = "Nikhil Lohar";
  console.log(obj);
}
```

In this JavaScript function, the `obj` variable is passed to the `printName` function, and its attribute value is changed. The updated value will be used wherever `obj` is accessed, meaning the original value is updated. This approach provides limited control over memory management.

Rust, on the other hand, gives us full control over memory. In Rust, we can transfer ownership of a variable or allow other functions to borrow the variable's reference.

Let's look at an example of transferring ownership:

```rust
pub mod memory {
  pub fn my_fn() {
    let fname: String = String::from("Nikhil");
    
    print_name(fname);
    
    // This line will cause an error: borrow of moved value
    // println!("Name has been printed, {}", fname);
  }
  
  pub fn print_name(name: String) {
    println!("Name is {}", name);
  }
}
```

In this code, ownership of `fname` is transferred to the `print_name` function. The reference is deleted from memory once ownership is transferred, which is why the second `println!` would cause an error.

To allow passing variables to other functions without transferring ownership, we can use the concept of borrowing:

```rust
pub mod memory {
  pub fn my_fn() {
    let fname: String = String::from("Nikhil");
    
    print_name(&fname);
    
    println!("Name has been printed, {}", fname);
  }
  
  pub fn print_name(name: &str) {
    println!("Name is {}", name);
  }
}
```

Here, we're not transferring ownership but allowing the `print_name` function to borrow the reference of the variable using `&`. This way, `fname` can be accessed after the `print_name` function completes without causing an error.

## Mutable and Immutable

In Rust, all variables are immutable by default, meaning their values cannot be changed. To make a variable mutable, we need to use the `mut` keyword when creating it:

```rust
let mut fname: String = String::from("Nikhil");
```

Only the owner of the variable can change its value. If we want to allow other functions to change the value of a variable they don't own, we need to pass a mutable reference:

```rust
pub fn print_name(name: &mut String)
```

And when passing the variable to the function:

```rust
&mut variable_name
```

This is how mutability is handled in Rust.

## Error Handling in Rust

Rust has two types of errors: recoverable errors and unrecoverable errors.

### Unrecoverable Errors

```rust
fn main() {
  println!("{}", divide(5, 0));
}

fn divide(x: u32, y: u32) -> u32 {
  x / y
}
```

In this code, we're trying to divide by zero, which causes the program to panic and crash. This is an unrecoverable error.

### Recoverable Errors

```rust
fn main() {
  let result: u32 = match divide(5, 0) {
    Ok(num) => num,
    Err(err) => {
      println!("Error: {}", err);
      0
    }
  };
  
  println!("Result: {}", result);
}

fn divide(x: u32, y: u32) -> Result<u32, String> {
  if y == 0 {
    return Err(String::from("Cannot divide by 0"));
  }
  
  Ok(x / y)
}
```

Here, we're making the error recoverable by using the `Result` type. The `divide` function returns a `Result` which can be either a `u32` or a `String` for an error.

We use `Err` to create an error response and `Ok` for a success response. The calling function can use a `match` expression to handle both `Ok` and `Err` cases, similar to promises in JavaScript.

## Programming Concepts
#### Keywords
In every programming language there is list of keywords that are associated with that language and it cannot be used to define variables. Same is with the Rust, there list of words like `﻿let` , `﻿fn` , `﻿as` , `﻿in` , `﻿if` , `﻿else` , `﻿break` etc. You take a look at this link [﻿Appendix A](https://doc.rust-lang.org/book/appendix-01-keywords.html).
```rust
fn keyword_example() { 
	let fn = 32; 
		^^ expected identifier, found keyword 
}
```
In the above code, we try create a variable with a name that are mentioned in keyboard then we get the error `expected identifier, found keyword`
  
 #### Variables
 
| Let | const | 
| ----- | ----- | 
| All variable are immutalbe by default and can be made mutable by adding `﻿mut` before identifier. | In const, we cannot use `﻿mut` keyboard because consts are strictly immutable by default and cannot be changed | 
| Let keyword infer type by default eg: `﻿let age = 32` , In this age will automatically infer type as `i32` | In const we need to explicitly add the type. | 
| Let are binded by scope. | const can be declare globally and they can also binded by scope | 
| Let value are can be set during compile time and runtime as well | const value are set and known during compile time, not during runtime. |

Code Example

```rust
const HOURS_IN_DAY : i32 = 24;

fn immutable_example() {
    let age = 25;
    println!("Your immutalbe age is  {}", age);
}

fn mutable_example() {
    let mut age = 25;
    println!("Your mutalbe age is  {}", age);
    age = 26;
    println!("Your mutalbe new age is  {}", age);
}

fn const_example() {
    const AGE : i32 = 25;
    println!("You const varialbe age is {}", AGE);
}
```
#### Shadowing
- Decaling a variable with same name within the scope it means shadowing, i.e second variable osershadow the first varialbe. 
-  We can also shadow the variable with different data types 
-  Shadow varialbe destroy when scope ends. 
-  Shadowing is different from marking a variable as `mut` because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword

```rust
let x = 5; 
let x = x + 1; 
{ 
	let x = x * 2; 
	println!("The value of x in the inner scope is: {x}"); 
} 
println!("The value of x is: {x}");
```
In the given code, we first declare `x` as an `i32` type and bind it to `5`. Then, we shadow the first `x` with a new value, `x + 1`. Inside a new scope, we shadow `x` again, this time modifying it to `x * 2`. This inner `x` only exists within this scope and is destroyed when the scope ends. Finally, we print the value of `x` in the outer scope.

## Guessing Game

### Phase 1: User Input

To understand how to take input from a user, we'll use Rust's standard input/output library:

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

We use `use std::io` to import the standard I/O library. In Rust, there's a concept called "prelude" which includes a list of predefined libraries that come with every program. `std::io` is not part of the prelude, so we need to import it explicitly.

Variables in Rust are immutable by default. To make them mutable, we add `mut` when creating a variable: `let mut guess`.

The `read_line` method appends the input to the variable rather than overwriting it. That's why it needs a mutable reference `&mut`. A mutable reference means that it needs permission from the variable's owner to append input to it.

`String::new()` indicates that `new` is an associated function of `String`. This is a common way to create a new instance of a type.

`read_line` returns a `Result` type, which is an enum with two states: `Ok` and `Err`. We use `expect` to handle potential errors.

### Phase 2: Generate Random Number and Compare

To generate a random number, we'll add the `rand` crate to our project:

```
cargo add rand
```

This command adds the `rand` library to `Cargo.toml`. After adding it, run `cargo build` to compile the project.

Here's how we generate a random number:

```rust
use rand::Rng;

fn generate_random_number() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret Number: {secret_number}");
}
```

We add `use rand::Rng` to bring the `Rng` trait into scope, which defines methods for random number generators.

To compare the user's guess with the secret number:

```rust
use std::cmp::Ordering;

let guess: i32 = guess.trim().parse().expect("Failed to parse!");

match guess.cmp(&secret_number) {
    Ordering::Equal => println!("You won"),
    Ordering::Greater => println!("Your number is greater"),
    Ordering::Less => println!("Your number is smaller")
}
```

We use `trim()` to remove whitespace and `parse()` to convert the string to an integer. The `match` expression handles the comparison result using the `Ordering` enum.

### Phase 3: Looping and Break

To create a complete guessing game, we'll use a loop to allow multiple guesses:

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn ask_input_from_user() -> String {
    println!("Guess the number!");
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read user input");

    guess
}

fn generate_random_number() -> i32 {
    rand::thread_rng().gen_range(1..=100)
}

fn compare_numbers(guess: &String, secret_number: &i32) -> bool {
    let guess: i32 = guess.trim().parse().expect("Error in parsing");
    
    match guess.cmp(secret_number) {
        Ordering::Equal => {
            println!("You won");
            true
        },
        Ordering::Greater => {
            println!("Your number is greater");
            false
        },
        Ordering::Less => {
            println!("Your number is smaller");
            false
        }
    }
}

fn main() {
    let secret_number = generate_random_number();
    loop {
        let guess = ask_input_from_user();

        let is_won = compare_numbers(&guess, &secret_number);

        if is_won {
            break;
        }
    }
}
```

In the `main` function, we use an infinite loop to repeatedly ask for user input. The `compare_numbers` function returns a boolean indicating whether the guess was correct. We use a `break` statement to exit the loop when the correct number is guessed.

This completes our Rust guessing game implementation!
