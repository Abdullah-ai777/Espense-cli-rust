use std::fs::{self, };
use std::io::{self, };
use chrono::Local;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Expense {
    item_name: String,
    price: f64,
    date: String,
}

fn main() {
    let now = Local::now();
    let mut data: Vec<Expense> = load_data();

    loop {
        println!("Enter a command (add, ls, delete, total, save, load, exit):");
        let command = input();

        match command.as_str() {
            "add" => {
                println!("Enter item name:");
                let item_name = input();
                println!("Enter item price:");
                let price: f64 = loop {
                    if let Ok(value) = input().parse::<f64>() {
                        break value;
                    } else {
                        println!("Invalid price. Please enter a valid number:");
                    }
                };

                data.push(Expense {
                    item_name,
                    price,
                    date: now.to_string(),
                });
                println!("Item added.");
            }

            "ls" => {
                println!("{:<15} {:<10} {:<20}", "Item", "Price", "Date");
                println!("----------------------------------------------------");
                for expense in &data {
                    println!("{:<15} {:<10.2} {:<20}", expense.item_name, expense.price, expense.date);
                }
            }

            "delete" => {
                println!("Enter item name to delete:");
                let item_name = input();
                let original_len = data.len();
                data.retain(|expense| expense.item_name != item_name);

                if data.len() < original_len {
                    println!("Item deleted.");
                } else {
                    println!("Item not found.");
                }
            }

            "total" => {
                let total: f64 = data.iter().map(|expense| expense.price).sum();
                println!("Total Expense: {:.2}", total);
            }

            "save" => {
                save_data(&data);
                println!("Data saved to file.");
            }

            "load" => {
                data = load_data();
                println!("Data loaded from file.");
            }
            "clear" => {
                clear_screen();
                println!("screen clear");
            },

            "exit" => break,

            _ => println!("Invalid command."),
        }
    }
}

fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn save_data(data: &Vec<Expense>) {
    let json = serde_json::to_string_pretty(data).expect("Failed to serialize data");
    fs::write("expenses.json", json).expect("Failed to write to file");
}

fn load_data() -> Vec<Expense> {
    if let Ok(content) = fs::read_to_string("expenses.json") {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn clear_screen() {
    // Use OS-specific commands to clear the screen
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd").arg("/c").arg("cls").status().unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}  