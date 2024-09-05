fn floating_type() {
    let my_f32 : f32 = 21.321654651651651;
    let my_f64 : f64  = 21.21354651654165165416;

    println!("My F32 : {}", my_f32);
    println!("My F64 : {}", my_f64);
}

fn numeric_operation() {
    let sum = 5 + 5;

    let difference = 5.5 - 6.23;

    let multiply = 5.5 * 20 as f64;

    let divide = (5 as f64) / 5.5;

    println!("Sum : {}, Difference : {}, Multiple : {}, Divide : {}", sum, difference, multiply, divide);
}

fn main() {
    floating_type();
    numeric_operation();
}
