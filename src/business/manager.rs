/* 
Author: Pawinee Mahantamak
Course: CST8002 - Practical Project 2
Professor: Stanley Pieda
Due Date: 2025-02-16
*/

/*! 
# Business Logic Layer (`manager.rs`)
This module provides functions for managing `NaturalGasLiquidExport` records, including:
- Reloading dataset from CSV
- Displaying records
- Creating new records
- Editing existing records
- Deleting records
*/
use std::error::Error;
use crate::models::natural_gas_liquid_export::NaturalGasLiquidExport;
use crate::persistence::csv_reader::read_csv_file;

/// Stores the dataset in memory for persistent operations
static mut RECORD_DATA: Option<Vec<NaturalGasLiquidExport>> = None;

/// Loads data on first run, and persists updates in memory.
/// Does NOT reset when adding, editing, or deleting records.
pub fn load_initial_data(file_path: &str) -> Result<&'static mut Vec<NaturalGasLiquidExport>, Box<dyn Error>> {
    unsafe {
        if RECORD_DATA.is_none() {
            let mut records = read_csv_file(file_path)?;
            records.truncate(100); // Load only first 100 records initially
            RECORD_DATA = Some(records);
        }
        Ok(RECORD_DATA.as_mut().unwrap())
    }
}

/// Displays records from memory.
pub fn display_records(records: &Vec<NaturalGasLiquidExport>, limit: usize) {
    if records.is_empty() {
        println!("No records available.");
    } else {
        let display_count = limit.min(records.len());
        for (i, record) in records.iter().take(display_count).enumerate() {
            println!("Record {}: {:?}", i+1, record);

            if (i + 1) % 10 == 0 {
                println!("Program by Pawinee Mahantamak");
            }
        }
        println!("Displayed {}/{} records.", display_count, records.len());
    }
}

/// Adds a new record and updates the in-memory dataset.
pub fn add_record(new_record: NaturalGasLiquidExport) {
    unsafe {
        if let Some(records) = &mut RECORD_DATA {
            records.push(new_record);
            println!("New record added. Total records: {}", records.len());
        }
    }
}

/// Edits a record and keeps the change in memory.
pub fn edit_record(index: usize, updated_record: NaturalGasLiquidExport) {
    unsafe {
        if let Some(records) = &mut RECORD_DATA {
            let total_records = records.len(); // Get dynamic record count

            if index == 0 || index > total_records {
                println!(
                    "Invalid index. Please enter a value between 1 and {}.",
                    total_records
                );
                return;
            }

            let actual_index = index - 1; // Convert 1-based index to 0-based

            records[actual_index] = updated_record;
            println!("Record {} updated successfully!", index);
        }
    }
}


/// Deletes a record from memory.
pub fn delete_record(index: usize) {
    unsafe {
        if let Some(records) = &mut RECORD_DATA {
            let total_records = records.len(); // Get current length dynamically

            if index == 0 || index > total_records {
                println!(
                    "Invalid index. Please enter a value between 1 and {}.",
                    total_records
                );
                return;
            }

            let actual_index = index - 1; // Convert 1-based index to 0-based
            records.remove(actual_index);
            println!(
                "Record {} deleted. Total remaining records: {}",
                index, records.len()
            );
        }
    }
}

/// Retrieves the current records stored in memory.
pub fn get_records() -> &'static mut Vec<NaturalGasLiquidExport> {
    unsafe { RECORD_DATA.as_mut().unwrap() }
}
