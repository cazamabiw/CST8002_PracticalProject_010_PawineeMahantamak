/* 
Author: Pawinee Mahantamak
Course: CST8002 - Practical Project 2
Professor: Stanley Pieda
Due Date: 2025-02-16
*/

/*! 
# CSV Writer Module (`csv_writer.rs`)
This module provides functionality for writing natural gas liquid export records
to a CSV file. The filename is automatically generated using a UUID to ensure uniqueness.
*/

use std::error::Error;
use std::fs::File;
use csv::Writer;
use uuid::Uuid;
use crate::models::natural_gas_liquid_export::{NaturalGasLiquidExport, NATURALGASLIQUID_HEADERS};

/// Writes a vector of `NaturalGasLiquidExport` records to a CSV file.
///
/// # Arguments
/// * `records` - A slice of `NaturalGasLiquidExport` objects to write.
///
/// # Returns
/// * `Ok(String)` - The generated filename if the operation succeeds.
/// * `Err(Box<dyn Error>)` - An error if writing to file fails.
///
/// # Behavior
/// * Creates a unique CSV filename using a UUID.
/// * Writes column headers first, followed by each record.
/// * Logs any file-writing errors.
pub fn write_csv_file(records: &[NaturalGasLiquidExport]) -> Result<String, Box<dyn Error>> {
    let file_name = format!("export_{}.csv", Uuid::new_v4()); // Generates a UUID filename
    let file_path = format!("data/{}", file_name);
    let file = File::create(&file_path)?;

    let mut wtr = Writer::from_writer(file);

    // Write header
    wtr.write_record(&NATURALGASLIQUID_HEADERS)?;

    // Write records
    for record in records {
        wtr.write_record(&[
            record.period().to_string(),
            record.year().to_string(),
            record.month().to_string(),
            record.product().to_string(),
            record.origin().to_string(),
            record.destination().to_string(),
            record.mode_of_transport().to_string(),
            record.volume_m3().to_string(),
            record.volume_bbl().to_string(),
            record.value_cad().to_string(),
            record.value_usd().to_string(),
            record.price_per_l_cents().to_string(),
            record.price_per_gal_cents().to_string(),
        ])?;
    }

    wtr.flush()?; // Ensure data is written to file
    println!("Data successfully written to {}", file_path);

    Ok(file_path)
}
