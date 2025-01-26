/* 
Author: Pawinee Mahantamak
Course: CST8002 - Practical Project 1
Professor: Stanley Pieda
Due Date: 2025-01-26
*/

/* Description:
This function reads a CSV file containing natural gas liquid export data. It decodes the file to UTF-8, 
parses each record into `NaturalGasLiquidExport` objects, and stores them in a vector. If any errors 
occur while reading or parsing the file, they are logged.
*/

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use encoding_rs::UTF_8;
use encoding_rs_io::DecodeReaderBytesBuilder;
use csv::ReaderBuilder;
use crate::models::natural_gas_liquid_export::NaturalGasLiquidExport;

pub fn read_csv_file(file_path: &str) -> Result<Vec<NaturalGasLiquidExport>, Box<dyn Error>> {
    let mut records = Vec::new();

    // Open the file
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(file);


    //As Rust's CSV reader expects UTF-8 by default so decode the file to UTF-8 (handles invalid UTF-8 gracefully)
    let transcoded_reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(UTF_8)) // Force UTF-8 decoding
        .build(buf_reader);

    // Create a CSV reader with the transcoded input
    let mut rdr = ReaderBuilder::new()
        .flexible(true) // Allow variable-length records
        .from_reader(transcoded_reader);

    for (index, result) in rdr.records().enumerate() {
        match result {
            Ok(record) => {
                let export = NaturalGasLiquidExport::new(
                    record[0].to_string(),                 // period
                    record[1].parse::<u16>()?,            // year
                    record[2].to_string(),                // month
                    record[3].to_string(),                // product
                    record[4].to_string(),                // origin
                    record[5].to_string(),                // destination
                    record[6].to_string(),                // mode_of_transportation
                    record[7].parse::<f64>()?,            // volume_m3
                    record[8].parse::<f64>()?,            // volume_bbl
                    record[9].parse::<f64>()?,            // value_cad
                    record[10].parse::<f64>()?,           // value_usd
                    record[11].parse::<f64>()?,           // price_cents_per_l
                    record[12].parse::<f64>()?,           // price_cents_per_gallon
                );
                records.push(export);
            }
            Err(e) => {
                eprintln!("Error parsing record {}: {}", index + 1, e);
            }
        }
    }

    Ok(records)
}
