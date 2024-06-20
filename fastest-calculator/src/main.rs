use std::io::{self, Write};
fn main() {
    println!("==================================================");
    println!("Welcome to super fast calculator");
    println!("==================================================");
    println!("1: Operation");
    println!("2: exit");
    println!("----------------------------------------------------");

    let exit_var: bool = false;
    while exit_var == false {
        print!("Enter your choice: ");
        let _res = std::io::stdout().flush().expect("Failed to flush stdout");

        let mut temp1 = String::new();

        io::stdin()
            .read_line(&mut temp1)
            .expect("Failed to read line");
        let choice: i8 = temp1.trim().parse().unwrap();
        match choice {
            1 => operate(),
            2 => {
                println!("Exiting the program");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

fn operate() {
    print!("Input: ");
    let _res = std::io::stdout().flush().expect("Failed to flush stdout");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    let input: Vec<&str> = temp.trim().split(" ").collect();
    let num1: f32 = input[0].parse().unwrap();
    let num2: f32 = input[2].parse().unwrap();
    match input[1] {
        "+" => println!(
            "Result of addition of {} and {} is {}",
            num1,
            num2,
            add(num1, num2)
        ),
        "-" => println!(
            "Result of substraction of {} and {} is {}",
            num1,
            num2,
            sub(num1, num2)
        ),
        "*" => println!(
            "Result of multiplication of {} and {} is {}",
            num1,
            num2,
            mul(num1, num2)
        ),
        "/" => println!(
            "Result of division of {} and {} is {}",
            num1,
            num2,
            div(num1, num2)
        ),
        "%" => println!(
            "Result of modulo of {} and {} is {}",
            num1,
            num2,
            md(num1, num2)
        ),
        "^" => println!(
            "Result of power of {} and {} is {}",
            num1,
            num2,
            pow(num1, num2)
        ),
        _ => println!("Invalid operator"),
    }
}

fn add(a: f32, b: f32) -> f32 {
    return a + b;
}
fn sub(a: f32, b: f32) -> f32 {
    return a - b;
}

fn mul(a: f32, b: f32) -> f32 {
    return a * b;
}
fn div(a: f32, b: f32) -> f32 {
    return a / b;
}
fn md(a: f32, b: f32) -> f32 {
    return a % b;
}
fn pow(a: f32, b: f32) -> f32 {
    return a.powf(b);
}
