/* 
Author: Pawinee Mahantamak
Course: CST8002 - Practical Project 2
Professor: Stanley Pieda
Due Date: 2025-02-16
*/

use std::error::Error;

use crate::{models::natural_gas_liquid_export::NaturalGasLiquidExport, persistence::csv_reader::read_csv_file};

pub fn reload_data(file_path: &str) -> Result<Vec<NaturalGasLiquidExport>, Box<dyn Error>> {
    println!("Reloading dataset...");
    let records = read_csv_file(file_path)?;
    println!("Successfully reloaded {} records!", records.len());
    Ok(records)
}
pub fn display_records(records: &Vec<NaturalGasLiquidExport>, limit: usize) {
    for (i, record) in records.iter().take(limit).enumerate() {
        println!("Record {}: {:?}", i + 1, record);
    }
    println!("Displayed {} records.", limit);
}

pub fn create_record(records: &mut Vec<NaturalGasLiquidExport>, new_record: NaturalGasLiquidExport) {
    records.push(new_record);
    println!("New record added successfully!");
}

pub fn edit_record(records: &mut Vec<NaturalGasLiquidExport>, index: usize, updated_record: NaturalGasLiquidExport) {
    if index < records.len() {
        records[index] = updated_record;
        println!("Record at index {} updated successfully!", index);
    } else {
        println!("Invalid index. No record updated.");
    }
}
