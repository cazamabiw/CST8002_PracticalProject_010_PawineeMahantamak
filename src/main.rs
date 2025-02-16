/* 
Author: Pawinee Mahantamak
Course: CST8002 - Practical Project 1
Professor: Stanley Pieda
Due Date: 2025-01-26
*/

/* Description:
This program reads data from a CSV file containing natural gas liquid export records, 
parses the data into structured records, and displays them. It handles errors when 
reading or parsing the file. The developer’s name is displayed at the top of the output.
*/

mod persistence; // Declare persistence module

use cst8002_practical_project_010_pawinee_mahantamak::models;
use persistence::csv_reader::read_csv_file;
use persistence::csv_writer::write_csv_file;

fn main() {

    //Open and read the dataset
    let file_path = "data/natural-gas-liquids-exports-monthly.csv";

     //Display my full name (always visible)
    const MY_FULL_NAME: &str = "Pawinee Mahantamak";
   

    println!("Testing CSV Read...");

    match read_csv_file(file_path) {
        Ok(records) => {
            println!("Successfully read {} records!", records.len());
            if records.is_empty() {
                println!("Warning: No records found in the dataset.");
            } else {
                println!("Sample Record: {:?}", records[0]); // Print one record
            }

            println!("\nTesting CSV Write...");
            match write_csv_file(&records) {
                Ok(output_file) => println!("Data saved successfully to: {}", output_file),
                Err(e) => eprintln!("Error writing CSV: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
    println!("Author: {}", MY_FULL_NAME);

}
