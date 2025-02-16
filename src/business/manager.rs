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

use crate::{models::natural_gas_liquid_export::NaturalGasLiquidExport, persistence::csv_reader::read_csv_file};

/// Reloads the dataset from a CSV file.
///
/// # Arguments
/// * `file_path` - The path to the CSV file.
///
/// # Returns
/// * `Ok(Vec<NaturalGasLiquidExport>)` - A vector of records if successfully loaded.
/// * `Err(Box<dyn Error>)` - An error if file reading fails.
pub fn reload_data(file_path: &str) -> Result<Vec<NaturalGasLiquidExport>, Box<dyn Error>> {
    println!("Reloading dataset...");
    let records = read_csv_file(file_path)?;
    println!("Successfully reloaded {} records!", records.len());
    Ok(records)
}

/// Displays a limited number of records on the screen.
///
/// # Arguments
/// * `records` - A reference to a vector of `NaturalGasLiquidExport` objects.
/// * `limit` - The number of records to display.
///
/// # Behavior
/// * Prints each record with an index.
/// * Every 10 records, prints "Program by Pawinee Mahantamak".
pub fn display_records(records: &Vec<NaturalGasLiquidExport>, limit: usize) {
    for (i, record) in records.iter().take(limit).enumerate() {
        println!("Record {}: {:?}", i + 1, record);

        // Print "Program by" every 10 records
        if (i + 1) % 10 == 0 {
            println!("Program by Pawinee Mahantamak");
        }
    }
    println!("Displayed {} records.", limit);
}


/// Adds a new record to the dataset in memory.
///
/// # Arguments
/// * `records` - A mutable reference to a vector of `NaturalGasLiquidExport` objects.
/// * `new_record` - The new `NaturalGasLiquidExport` object to add.
///
/// # Behavior
/// * Adds the record to the vector.
pub fn create_record(records: &mut Vec<NaturalGasLiquidExport>, new_record: NaturalGasLiquidExport) {
    records.push(new_record);
    println!("New record added successfully!");
}

/// Edits an existing record in memory.
///
/// # Arguments
/// * `records` - A mutable reference to a vector of `NaturalGasLiquidExport` objects.
/// * `index` - The index of the record to edit.
/// * `updated_record` - The modified `NaturalGasLiquidExport` object.
///
/// # Behavior
/// * Updates the record at the given index.
/// * If the index is invalid, prints an error message.
pub fn edit_record(records: &mut Vec<NaturalGasLiquidExport>, index: usize, updated_record: NaturalGasLiquidExport) {
    if index < records.len() {
        records[index] = updated_record;
        println!("Record at index {} updated successfully!", index);
    } else {
        println!("Invalid index. No record updated.");
    }
}

/// Deletes a record from memory.
///
/// # Arguments
/// * `records` - A mutable reference to a vector of `NaturalGasLiquidExport` objects.
/// * `index` - The index of the record to delete.
///
/// # Behavior
/// * Removes the record at the given index.
/// * If the index is invalid, prints an error message.
pub fn delete_record(records: &mut Vec<NaturalGasLiquidExport>, index: usize) {
    if index < records.len() {
        records.remove(index);
        println!("Record at index {} deleted successfully!", index);
    } else {
        println!("Invalid index. No record deleted.");
    }
}