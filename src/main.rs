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
use cst8002_practical_project_010_pawinee_mahantamak::models::natural_gas_liquid_export::NaturalGasLiquidExport;
use cst8002_practical_project_010_pawinee_mahantamak::presentation::menu::show_menu;
use persistence::csv_reader::read_csv_file;


fn main() {

    //Open and read the dataset
    let file_path = "data/natural-gas-liquids-exports-monthly.csv";

     //Display my full name (always visible)
    const MY_FULL_NAME: &str = "Pawinee Mahantamak";
   
    println!("Author: {}", MY_FULL_NAME);
    // Initialize dataset (load up to 100 records)
    let mut records: Vec<NaturalGasLiquidExport> = match read_csv_file(file_path) {
        Ok(mut records) => {
            records.truncate(100); // Limit to 100 records
            println!("Successfully loaded {} records!", records.len());
            records
        }
        Err(e) => {
            eprintln!("Error loading dataset: {}", e);
            Vec::new()
        }
    };

    // Start interactive menu
    show_menu(&mut records, file_path);
 

}
