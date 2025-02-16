/* 
Author: Pawinee Mahantamak
Course: CST8002 - Practical Project 2
Professor: Stanley Pieda
Due Date: 2025-02-16
*/

/* 
Description:
This module provides a command-line menu interface for interacting 
with the natural gas liquid exports dataset.
*/

use std::io::{self, Write};
use crate::business::manager::*;
use crate::persistence::csv_writer::write_csv_file;
use crate::models::natural_gas_liquid_export::NaturalGasLiquidExport;

pub fn show_menu(records: &mut Vec<NaturalGasLiquidExport>, file_path: &str) {
    loop {
        println!("\n=== Natural Gas Liquid Export Management ===");
        println!("1. Reload Data from CSV");
        println!("2. Display Records");
        println!("3. Create a New Record");
        println!("4. Edit a Record");
        println!("5. Delete a Record");
        println!("6. Save Data to CSV");
        println!("7. Exit");
        print!("Select an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                match reload_data(file_path) {
                    Ok(new_records) => {
                        *records = new_records;
                        println!("Data reloaded successfully.");
                    }
                    Err(e) => println!("Error reloading data: {}", e),
                }
            }
            "2" => {
                println!("How many records would you like to display? ");
                let mut num_records = String::new();
                io::stdin().read_line(&mut num_records).unwrap();
                if let Ok(limit) = num_records.trim().parse::<usize>() {
                    display_records(records, limit);
                } else {
                    println!("Invalid number.");
                }
            }
            "3" => {
                println!("Enter details for the new record:");
                let new_record = create_record_interactively();
                records.push(new_record);
                println!("Record added successfully.");
            }
            "4" => {
                println!("Enter the index of the record to edit:");
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).unwrap();
                if let Ok(index) = index_str.trim().parse::<usize>() {
                    if index < records.len() {
                        println!("Editing record at index {}...", index);
                        let updated_record = create_record_interactively();
                        edit_record(records, index, updated_record);
                    } else {
                        println!("Invalid index.");
                    }
                } else {
                    println!("Invalid input.");
                }
            }
            "5" => {
                println!("Enter the index of the record to delete:");
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).unwrap();
                if let Ok(index) = index_str.trim().parse::<usize>() {
                    delete_record(records, index);
                } else {
                    println!("Invalid input.");
                }
            }
            "6" => {
                match write_csv_file(&records) {
                    Ok(output_file) => println!("Data saved to {}", output_file),
                    Err(e) => println!("Error saving data: {}", e),
                }
            }
            "7" => {
                println!("Exiting program...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

/// Helper function to create a record interactively
fn create_record_interactively() -> NaturalGasLiquidExport {
    
    fn prompt(text: &str) -> String {
        let mut input = String::new();
        print!("{}", text);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    NaturalGasLiquidExport::new(
        prompt("Enter period: "),
        prompt("Enter year: ").parse::<u16>().unwrap_or(0),
        prompt("Enter month: "),
        prompt("Enter product: "),
        prompt("Enter origin: "),
        prompt("Enter destination: "),
        prompt("Enter mode of transport: "),
        prompt("Enter volume (m³): ").parse::<f64>().unwrap_or(0.0),
        prompt("Enter volume (bbl): ").parse::<f64>().unwrap_or(0.0),
        prompt("Enter value (CAD$): ").parse::<f64>().unwrap_or(0.0),
        prompt("Enter value (USD$): ").parse::<f64>().unwrap_or(0.0),
        prompt("Enter price per L (CN cents): ").parse::<f64>().unwrap_or(0.0),
        prompt("Enter price per gallon (US cents): ").parse::<f64>().unwrap_or(0.0),
    )
}
