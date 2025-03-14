/* 
Author: Pawinee Mahantamak
Course: CST8002 - Practical Project 2
Professor: Stanley Pieda
Due Date: 2025-02-16
*/

/*! 
# CSV Reader Module (`csv_reader.rs`)
This module provides functionality for reading CSV files 
containing natural gas liquid export data. It parses each record 
into `NaturalGasLiquidExport` objects and handles missing or 
malformed records gracefully.
*/

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use encoding_rs::UTF_8;
use encoding_rs_io::DecodeReaderBytesBuilder;
use csv::ReaderBuilder;
use crate::models::{export_financial::ExportFinancial, export_record::ExportRecord, export_summary::{self, ExportSummary}, natural_gas_liquid_export::{NaturalGasLiquidExport, NATURALGASLIQUID_HEADERS}};

/// Reads a CSV file and returns a vector of `NaturalGasLiquidExport` objects.
///
/// # Arguments
/// * `file_path` - The path to the CSV file.
///
/// # Returns
/// * `Ok(Vec<NaturalGasLiquidExport>)` - A vector of successfully parsed records.
/// * `Err(Box<dyn Error>)` - An error if file reading or parsing fails.
///
/// # Behavior
/// * Skips malformed records that do not match expected column count.
/// * Logs errors but continues reading the remaining records.
pub fn read_csv_file(file_path: &str, record_type: &str) -> Result<Vec<Box<dyn ExportRecord>>, Box<dyn Error>> {
    let mut records: Vec<Box<dyn ExportRecord>> = Vec::new();

    // Open the file
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);

    // Decode to UTF-8
    let transcoded_reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(UTF_8))
        .build(buf_reader);

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new()
        .has_headers(true) // Ensure the first row is treated as headers
        .from_reader(transcoded_reader);

    for (index, result) in rdr.records().enumerate() {
        match result {
            Ok(record) => {
                // Ensure correct number of columns before parsing
                if record.len() == NATURALGASLIQUID_HEADERS.len() {
                    let period = record[0].to_string();
                    let year = record[1].parse::<u16>()?;
                    let month = record[2].to_string();
                    let product = record[3].to_string();
                    let volume_m3 = record[7].parse::<f64>()?;
                    let value_cad = record[9].parse::<f64>()?;
                    let value_usd = record[10].parse::<f64>()?;

                         // Instantiate different objects based on the selected type
                         let export_record: Box<dyn ExportRecord> = match record_type {
                            "full" => {
                                Box::new(NaturalGasLiquidExport::new(
                                    record[0].to_string(),
                                    record[1].parse::<u16>()?, 
                                    record[2].to_string(),
                                    record[3].to_string(),
                                    record[4].to_string(),
                                    record[5].to_string(),
                                    record[6].to_string(),
                                    record[7].parse::<f64>()?,
                                    record[8].parse::<f64>()?,
                                    record[9].parse::<f64>()?,
                                    record[10].parse::<f64>()?,
                                    record[11].parse::<f64>()?,
                                    record[12].parse::<f64>()?,
                                ))
                            },
                            "summary" => 
                             { Box::new(ExportSummary {
                                period: record[0].to_string(),
                                year: record[1].parse::<u16>()?,
                                product: record[3].to_string(),
                                volume_m3: record[7].parse::<f64>()?,
                             })
                             },
                            "financial" =>
                            {
                                Box::new(ExportFinancial {
                                    period: record[0].to_string(),
                                    year: record[1].parse::<u16>()?,
                                    product: record[3].to_string(),
                                    value_cad: record[9].parse::<f64>()?,
                                    value_usd: record[10].parse::<f64>()?,
                                })
                            },
                            _ => {
                                eprintln!("Invalid record type. Defaulting to Full Record.");
                                Box::new(NaturalGasLiquidExport::new(
                                    period, year, month, product,
                                    record[4].to_string(), record[5].to_string(),
                                    record[6].to_string(), volume_m3, 
                                    record[8].parse::<f64>()?, value_cad, value_usd,
                                    record[11].parse::<f64>()?, record[12].parse::<f64>()?
                                ))
                            }
                        };
    
                        records.push(export_record);
                } else {
                    eprintln!("Skipping malformed record at row {}", index + 1);
                }
            }
            Err(e) => {
                eprintln!("Error parsing record {}: {}", index + 1, e);
            }
        }
    }

    Ok(records)
}