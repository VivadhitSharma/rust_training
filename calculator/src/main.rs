use std::fs::metadata;
use std::io::stdin;
use std::num::ParseIntError;
use std::path::Path;
use std::thread;
use std::thread::{JoinHandle};
use std::fs::File;
use std::io::{self, Read};
use serde::{Deserialize, Serialize};
use serde_json;
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

fn show_menu() -> Option<JoinHandle<()>> {
    println!("Enter the first number: ");
    let first_number = get_input_from_user().expect("Invalid input");

    println!("Enter the second number: ");
    let second_number = get_input_from_user().expect("Invalid input");

    println!("Enter the operation (+, *, -, /): ");
    let operation = get_string_from_user(); // z = "+"

    println!("Do you want to use threads for the operation? (Y/N)");
    let use_thread = get_string_from_user();

    if use_thread.trim() == "Y" || use_thread.trim() == "y" {
        let operation_handle = thread::spawn(move || {
            println!("using thread");
            handle_operation(operation, first_number, second_number);
        });
        return Some(operation_handle);
    } else if use_thread.trim() == "N" || use_thread.trim() == "n" {
        println!("without using thread");
        handle_operation(operation, first_number, second_number);
    } else {
        println!("Invalid Input");
    }
    None
}

fn handle_operation(operation: String, first_name: i32, second_name: i32) {
    match operation.trim() {
        "+" => {
            let res = operations::add(first_name, second_name);
            println!("Addition: {}", res);
        }
        "-" => {
            let res = operations::sub(first_name, second_name);
            println!("Subtraction : {}", res);
        }
        "*" => {
            let res = operations::mul(first_name, second_name);
            println!("Multiplication : {}", res);
        }
        "/" => {
            let result = operations::div(first_name as f32, second_name as f32);
            match result {
                Ok(value) => println!("Result: {:.2}", value),
                Err(e) => println!("Error: {}", e),
            }
        }
        _ => println!("Invalid operation!"),
    }
}


#[derive(Debug, Deserialize)]
struct MyStruct {
    first_number: i32,
    second_number: i32,
    operation: String,
}
impl MyStruct {
    fn perform_operation(&self) -> i32 {
        match self.operation.as_str() {
            "add" => self.first_number + self.second_number,
            "subtract" => self.first_number - self.second_number,
            "multiply" => self.first_number * self.second_number,
            "divide" => self.first_number / self.second_number,
            _ => {
                println!("Unsupported operation: {}", self.operation);
                0
            }
        }
    }
}
fn json_fun() -> io::Result<()> {
    println!("Please enter the path to the JSON file:");
    let file_path = get_string_from_user();
    let file_path = file_path.trim();
    let file_path = Path::new(&file_path);

    match metadata(file_path) {
        Ok(meta) => {
            // Get the file size
            let file_size = meta.len();
            println!("The size of the file is: {} bytes", file_size);
        }
        Err(e) => {
            println!("Failed to get file metadata: {}", e);
        }
    }

    let file = File::open(file_path);
    let mut content = String::new();
    file?.read_to_string(&mut content)?;
    println!("File contents:\n{}", content);

    let my_struct: MyStruct = serde_json::from_str(&content).expect("JSON was not well-formatted");
    // Print the struct
    println!("{:?}", my_struct);

    // Perform and print the operation result
    let result = my_struct.perform_operation();
    println!("Result: {}", result);

    Ok(())
}

fn main() {
    println!("Do you want to use the json file (y/n):");
    let use_json = get_string_from_user();
    if use_json.trim() == "Y" || use_json.trim() == "y" {
        json_fun();
    } else if use_json.trim() == "N" || use_json.trim() == "n" {
        loop {
            let operation_handle = show_menu();
            if let Some(handle) = operation_handle {
                handle.join().expect("Failed to join thread");
            }
            let input_for_conti = get_input_for_conti();
            if input_for_conti.trim() == "Y" || input_for_conti.trim() == "y" {
                continue;
            } else if input_for_conti.trim() == "N" || input_for_conti.trim() == "n" {
                break;
            } else {
                println!("Please type either yes or no!")
            }
        }
    } else {
        println!("Invalid Input");
    }
}

fn get_input_for_conti() -> String {
    println!("Do you want to continue? (y/n)");
    let mut input_for_conti = String::new();
    stdin().read_line(&mut input_for_conti).expect("Failed to read line from stdin!");
    input_for_conti
}





