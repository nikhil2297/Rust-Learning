
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


fn keyword_example() {
    // let fn  = 32;
        // ^^ expected identifier, found keyword
}

fn shadowing_example() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn main() {
    immutable_example();
    mutable_example();
    const_example();

    println!("Your Global varialbe hours in a day is {}", HOURS_IN_DAY);
}
