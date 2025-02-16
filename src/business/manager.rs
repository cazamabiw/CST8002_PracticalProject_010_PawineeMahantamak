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