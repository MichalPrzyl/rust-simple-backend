use std::io;

fn main() {
    println!("CALC!");

    println!("Enter first value");
    let mut first_number = String::new();
    io::stdin().read_line(&mut first_number).expect("Cannot read nubmer");
    let first_number: f64 = first_number.trim().parse().expect("It's not a number");

    println!("Enter second value");
    let mut second_number = String::new();
    io::stdin().read_line(&mut second_number).expect("Cannot read number");
    let second_number: f64 = second_number.trim().parse().expect("It's not a number");

    println!("Select operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Cannot read operation");
    let operation = operation.trim();

    // COMPUTING
    let result = if operation == "+" {
        first_number + second_number
    } else if operation == "-" {
        first_number - second_number
    } else if operation == "*" {
        first_number * second_number
    } else if operation == "/" {
        if second_number == 0.0 {
            println!("Error: Zero division");
            return;
        }
        first_number / second_number
    } else {
        println!("Unknown operation");
        return;
    };

    // Show result
    println!("Wynik: {}", result);
}

