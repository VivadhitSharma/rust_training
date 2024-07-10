use std::io::stdin;
use std::num::ParseIntError;
mod operations;

fn get_input_from_user() -> Result<i32, ParseIntError> {
    let mut input_for_x_y = String::new();
    stdin().read_line(&mut input_for_x_y).unwrap();
    input_for_x_y.trim().parse::<i32>()
}
fn get_string_from_user() -> String {
    let mut input_for_operation = String::new();
    stdin().read_line(&mut input_for_operation).expect("Failed to read line from stdin!");
    return input_for_operation;
}

fn show_menu() {
    println!("Enter the value of x");
    let x = get_input_from_user().expect("Invalid input"); // x = 32

    println!("Enter the value of y");
    let y = get_input_from_user().expect("Invalid input"); // y = 32

    println!("Enter the operation '+,*,-,/'");
    let z = get_string_from_user(); // z = "+"


    match z.trim() {
        "+" => {
            let res = operations::add(x, y);
            println!("Addition: {}", res);
        }
        "-" => {
            let res = operations::sub(x,y);
            println!("Subtraction : {}", res);
        }
        "*" => {
            let res = operations::mul(x,y);
            println!("Multiplication : {}", res);
        }
        "/" => {

            let result = operations::div(x as f32, y as f32);
            match result {
                Ok(value) => println!("Result: {:.2}", value),
                Err(e) => println!("Error: {}", e),
            }
        }
        _ => {}
    }
}

fn get_input_for_conti() -> String {
    println!("Do you want to continue? (y/n)");
    let input_for_go_conti = get_string_from_user();
    return input_for_go_conti;
}

fn main() {
    loop {
        show_menu();
        let b = get_input_for_conti();
        if b.trim() == "y" {
            continue;
        } else if b.trim() == "n" {
            break;
        } else {
            println!("Please type either yes or no!")
        }
    }
}
