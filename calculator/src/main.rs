use std::io;

fn main() {
    loop {
        let mut input = String::new();
        println!("Enter first number: ");
        io::stdin().read_line(&mut input).unwrap();
        let num1: f64 = input.trim().parse().unwrap();
        let mut operator = String::new();
        println!("Enter operator (+,-,*,/): ");
        io::stdin().read_line(&mut operator).unwrap();
        let mut input = String::new();
        println!("Enter second number: ");
        io::stdin().read_line(&mut input).unwrap();
        let num2: f64 = input.trim().parse().unwrap();
        if operator.trim() == "+" {
            println!("Result: {}", num1 + num2);
        }
        else if operator.trim() == "-" {
            println!("Result: {}", num1 - num2);
        }
        else if operator.trim() == "*" {
            println!("Result: {}", num1 * num2);
        }
        else if operator.trim() == "/" {
            println!("Result: {}", num1 / num2);
        }
        else {
            println!("Operator is invalid");
        }
        println!("Calculate again? (y/n): ");
        let mut again = String::new();
        io::stdin().read_line(&mut again).unwrap();
        if again.trim() != "y" {
            break;
        }
    }
}
