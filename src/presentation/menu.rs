/* 
Author: Pawinee Mahantamak
Course: CST8002 - Practical Project 2
Professor: Stanley Pieda
Due Date: 2025-02-16
*/

/*! 
# Presentation Layer (`menu.rs`)
This module provides a command-line interface (CLI) for managing the natural gas 
liquid export dataset. It allows users to:
- **Reload data from CSV**
- **Display records**
- **Create new records**
- **Edit existing records**
- **Delete records**
- **Save changes to CSV**
*/

use std::io::{self, Write};
use crate::business::manager::*;
use crate::persistence::csv_reader::read_csv_file;
use crate::persistence::csv_writer::write_csv_file;
use crate::models::natural_gas_liquid_export::NaturalGasLiquidExport;

/// Displays an interactive menu for managing the dataset.
///
/// # Arguments
/// * `records` - A mutable reference to a vector of `NaturalGasLiquidExport` objects.
/// * `file_path` - The file path of the CSV dataset.
///
/// # Behavior
/// * Provides a menu with options for managing records.
/// * Calls appropriate business layer functions for each operation.
/// * Handles invalid input gracefully.
pub fn show_menu(file_path: &str) {
    let mut record_type = String::from("full"); // Default to full records
    let mut records = read_csv_file(file_path, &record_type).expect("Failed to load data");

    loop {
        println!("\n Program by Pawinee Mahantamak");
        println!("\n=== Natural Gas Liquid Export Management ===");
        println!("1. Display Records");
        println!("2. Create a New Record");
        println!("3. Edit a Record");
        println!("4. Delete a Record");
        println!("5. Save Data to CSV");
        println!("6. Exit");
        print!("Select an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                println!("Select record type:");
                println!("1. Full Record");
                println!("2. Summary Record");
                println!("3. Financial Record");
                print!("Enter choice (1/2/3): ");
                io::stdout().flush().unwrap();

                let mut record_choice = String::new();
                io::stdin().read_line(&mut record_choice).unwrap();

                record_type = match record_choice.trim() {
                    "1" => "full".to_string(),
                    "2" => "summary".to_string(),
                    "3" => "financial".to_string(),
                    _ => {
                        println!("Invalid selection. Defaulting to Full Record.");
                        "full".to_string()
                    }
                };

                // Reload data with selected record type
                records = read_csv_file(file_path, &record_type).expect("Failed to reload data");
                println!("Loaded records as: {}", record_type);
                    // Ask the user how many records they want to display
                println!("Enter the number of records to display: ");
                let mut num_records = String::new();
                io::stdin().read_line(&mut num_records).unwrap();

                if let Ok(limit) = num_records.trim().parse::<usize>() {
                display_records(&records, limit);
                } else {
                println!("Invalid number. Showing all records.");
                display_records(&records, records.len()); // Default to showing all if invalid input
                }
            }
            "2" => {
                let new_record = create_record_interactively();
                add_record(new_record);
            }
            "3" => {
                println!("Enter the index of the record to edit:");
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).unwrap();
            
                if let Ok(index) = index_str.trim().parse::<usize>() {
                    let total_records = records.len(); // Get latest record count
            
                    if index == 0 || index > total_records {
                        println!("Invalid index. Please enter a value between 1 and {}.", total_records);
                    } else {
                        let updated_record = create_record_interactively();
                        edit_record(index, updated_record);
                    }
                } else {
                    println!("Invalid input. Please enter a valid number.");
                }
            }
            "4" => {
                println!("Enter the index of the record to delete:");
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).unwrap();
            
                if let Ok(index) = index_str.trim().parse::<usize>() {
                    let total_records = records.len(); // Get latest record count
            
                    if index == 0 || index > total_records {
                        println!("Invalid index. Please enter a value between 1 and {}.", total_records);
                    } else {
                        delete_record(index);
                    }
                } else {
                    println!("Invalid input. Please enter a valid number.");
                }
            }
            "5" => {
                // Convert Vec<Box<dyn ExportRecord>> to Vec<NaturalGasLiquidExport>
                let converted_records: Vec<NaturalGasLiquidExport> = records.iter()
                .filter_map(|record| record.as_any().downcast_ref::<NaturalGasLiquidExport>().cloned())
                .collect();

                // Save data
                match write_csv_file(&converted_records) {
                Ok(output_file) => println!("Data saved to {}", output_file),
                Err(e) => println!("Error saving data: {}", e),
                }
            }
            "6" => {
                println!("Exiting program...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
/// Prompts the user for input and creates a new `NaturalGasLiquidExport` record interactively.
///
/// # Returns
/// * A new `NaturalGasLiquidExport` object based on user input.
///
/// # Behavior
/// * Asks the user for each field's value.
/// * Converts numerical fields safely, using default values if input is invalid.
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
        prompt("Enter value (CN$): ").parse::<f64>().unwrap_or(0.0),
        prompt("Enter value (USD$): ").parse::<f64>().unwrap_or(0.0),
        prompt("Enter price per L (CN cents): ").parse::<f64>().unwrap_or(0.0),
        prompt("Enter price per gallon (US cents): ").parse::<f64>().unwrap_or(0.0),
    )
}