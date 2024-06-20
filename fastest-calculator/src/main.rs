use std::io::{self, Read, Write};
fn main() {
    println!("==================================================");
    println!("Welcome to super fast calculator");
    println!("==================================================");
    print!("Enter the first number: ");
    let _res = std::io::stdout().flush();
    let mut temp1 = String::new();

    io::stdin()
        .read_line(&mut temp1)
        .expect("Failed to read line");
    let number1: f32 = temp1.trim().parse().expect("Please enter a number!");

    print!("Enter the second number: ");
    let _res1 = std::io::stdout().flush();
    let mut temp2 = String::new();
    io::stdin()
        .read_line(&mut temp2)
        .expect("Failed to read line");

    let number2: f32 = temp2.trim().parse().expect("Please enter a number!");
    println!(
        "The addition of {} and {} is {}",
        number1,
        number2,
        add(number1, number2)
    );
    println!(
        "The subtraction of {} and {} is {}",
        number1,
        number2,
        sub(number1, number2)
    );
    println!(
        "The multiplication of {} and {} is {}",
        number1,
        number2,
        mul(number1, number2)
    );
    println!(
        "The division of {} and {} is {}",
        number1,
        number2,
        div(number1, number2)
    );
}
fn add(a: f32, b: f32) -> f32 {
    return a + b;
}
fn sub(a: f32, b: f32) -> f32 {
    return a - b;
}
fn mul(a: f32, b: f32) -> f32 {
    return a + b;
}
fn div(a: f32, b: f32) -> f32 {
    return a + b;
}
