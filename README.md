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

#### Data Types
Rust is a statically type language, which means that it must the types of all variable at compile time. Compiler can automatically infer the type based on the variable value, But the type is infer to the variable it cannot be changed. But for some cases have to infer the type explicitally. 

```rust
// Compiler can automatically infer type as i32 (default)
let guess = 42;

// We have explicitly infer the type because it doesn't know 
// what parse will return
let guess : u32 = "42".parse().expect("Not a Number!");
```
If we don't add the `﻿: u32`  while parsing a string, compiler will not know into what type we want to parse it. And it will display error as `﻿type must be known at this point` .

There are different type annotations for other data types.

#### Scalar Types
Scalar types represent a single value like a variable can contain single value like a integers, floating-point, booleans and characters which are also the four primary types of Rust.

**Integer Types**

An integer is a number which can be like, -3,-2,-1,0,1,2,3  not decimal or not fraction like 1/2 etc. Integers types can be **signed integer or unsigned integer****.  **Signed integer starts with `﻿i`  and unsigned integers starts with `﻿u` .  If we don't infer the type to integer it's automatically assigned to `i32`. 

| Length | Signed | Unsigned |
| ----- | ----- | ----- |
| 8-bit | `﻿i8`  | `﻿u8`  |
| 16-bit | `﻿i16`  | `﻿u16`  |
| 32-bit | `﻿i32`  | `﻿u32`  |
| 64-bit | `﻿i64`  | `﻿u64`  |
| arch (This consider the machine architeture 32 or 64) | `﻿isize`  | `﻿usize`  |


When we write numbers on paper sometimes we have to denote a number with a sign that is `﻿-`  to show it's a negative number with the same concept when we want a integer type can also be negative we have use `signed integer`. It ranges from `-(2^n-1) to 2^n-1 ` 

Also when we normally write number we always consider it postive, In rust when we that the for cases like age, weight, speed will always be positive we use `﻿unsigned integers` . It ranges from `﻿0 to 2^n-1` .

One more intresting about rust is, We can write integers literals in any form you can see it in below table. Like sometimes integers can come into multiple numeric type to handle it we can use type suffix, such as `﻿58u8`  which can read as `﻿58 as u8` . We can also use `﻿_`  as a visual seperator to make the number in readable format.

```rust
// This is how we normally write the number, Hard to read
let amount = 10000000;

// When we use visual seperator, Easy to read
let amount = 1_00_00_000;
```
The output of the visual speerator will be like this `﻿10000000` .

| Number literals | Example | Description |
| ----- | ----- | ----- |
| Decimal | 92_222 | With single `_`  it consider it decimal and with multiple it consider it visual sperator |
| Hex | 0xff | Hex number literals are denoted with traling `﻿0x`  |
| Octal | 0o77 | Octal number literals are denoted with traling `﻿0o`  |
| Binary | 0b1111_0000 | Binary number literals are denoted with traling `﻿0b` or else it will consider it as normal integer or decimal |
| Byte (u8 only) | b'A' | Byte will be `﻿u8`  that is unsigned 8 bit integer. And it is identifies by `b'`  |


**Integer Overflow**

Let's take an example we have create a varialbe age as `﻿u8`  and values can range from 0 to 255. There can be few case where overflow can happen

**Case 1**

We assign the age value to be 256. It will show an compile time error and it won't build in any mode that is debug or realease

**Case 2**

Suppose we dynamically accept the value of age from user. For debug build : During runtime the application will go into panic mode and will be crashed. For production build : The number we cycled to the new value also called as **complement wrapping** that is `﻿if age is 256 for u8 type, the new value will be come 0` . But it won't crash and it is also consider an error

To explicity hande the error we can use following things

| Method | Description | Code |
| ----- | ----- | ----- |
| wrapping_* | Wrap in all modes and it works like a complement wrapping for debug mode also | <p>`﻿fn getAge() { 200 }` </p><p>`﻿let age = getAge().wrapping_add(100)` </p><p> </p> |
| checked_* | It checkes with value if it overflows or not and returns `Options` with variants as `﻿Some` and `﻿None`  | <p>`﻿fn getAge() { 200 }` </p><p>`let age = match getAge().checked_add(100) {` </p><p>` Some(num) => num,` </p><p>` None => {` </p><p>   `println!("Cannot be added")` </p><p> `}` </p><p>`}` </p> |
| overflowing_* | It detects whether there was overflowing and returns a tuple with `﻿value and boolean`  | <p>`﻿fn getAge() { 200 }` </p><p>`﻿let (age: u8, is_of: bool) =  getAge().overflowing_add(100)` </p> |


**Floating-Point Types :**

Rust has two type of floating-point integers that are `﻿f32` and `﻿f64` , which are 32 bit and 64 bit in size. The major difference between these types are `﻿f32`  type is single precision and `f64`  is double precision. By default the rust infer the `f64`  type.

```rust
let my_f32 : f32 = 21.321654651651651;
let my_f64 : f64  = 21.21354651654165165416;

println!("My F32 : {}", my_f32);
println!("My F64 : {}", my_f64);

//Output
My F32 : 21.321655
My F64 : 21.21354651654165
```
**Numeric Operations**

```rust
let sum = 5 + 5;

let difference = 5.5 - 6.23;

let multiply = 5.5 * 20 as f64;

let divide = (5 as f64) / 5.5;
```
There other expression also available have a look into it [﻿Appendix B](https://doc.rust-lang.org/beta/book/appendix-02-operators.html)﻿.

Also while doing multiple and divide with integer and float. It was not able to perform expression of two different type. We have to make bother value type same by using `﻿as <type>` .

**Boolean Type**

As for boolean there are two types `﻿true`  or `﻿false`  and we can annotate the type with `﻿bool` . They are one byte in size

**Character Type**

Character are annotate with `﻿char`  and value are assigned with single quote and char is single character value. It's a 4 byte in size and represent a Unicode Scaler Value. Normally ASCII value can only contain some letter for english and numbers and symbols. While Unicode contain All ASCII values which make it backward compatible and also all languages character can also be added. Which make that character can store emoji also.

```rust
let c = 'z';
let c2 : &str = "z";
let z: char = 'ℤ'; // with explicit type annotation
let heart_eyed_cat = '😻';
```
#### Compound Type
Compound types can group multiple values like array and tuples.

**Tuple Type**

A tuple is grouping number of elements which can be different type or same type into one compound type. Tuple have a fix length it cannot be changed in size.

To access the values in tuple we have to desctrucutre the tuple using pattern matching format. As shown in the code below :

```
let tup = (500, 6.4, 1);

let (x, y, z) = tup;

println!("The value of y is: {y}");
```
When we declare `﻿tup`  with round brackets and comma seperated values, compiler automatic infer the type as tuple with `﻿i32` , `﻿f64`  and `﻿i32` .

To Destructure the tuple we are creating a same pattern with rounded brackets and comma seperated values `﻿(x, y, z) = tup` .

Then we will be able to access the value.



**Array Type**

If we want large collection of value we can use array. Unlike tuple every element in array are of same type. In rust array have predefined length but the values can be changed in array. 

To have a resizeable array we have to use `﻿vector`  which we will learn about it later.

To annoatate an array we have to `﻿[]` square bracket with type and size of the array. Like 

`﻿let a : [i32, 5] = [1,2,3,4,5]` . Arrays are usefull when you know the size of the array like days in weeks, months in year etc.

There is another way to declare a array in rust. Like this `﻿let a = [10; 5]` . It means we are creating a array of size 5 and all values in the array will be 10;

To access th earray element we can use element index like we do it in all programming language.

Suppose you are try to access the index of array which is not presented in the array list. Then rust will panic and exit the code showing `﻿index out of bounds`  error. Which makes the rust different from low-level languges which allows you to continue the code if after memory access error. Where in rust it exit the code ask us to handle it.

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
