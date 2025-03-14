/* 
Author: Pawinee Mahantamak
Course: CST8002 - Practical Project 1
Professor: Stanley Pieda
Due Date: 2025-01-26
*/

/* Description:  
This program provides a command-line interface (CLI) for managing  
natural gas liquid export records. Users can reload data, display records,  
add new entries, edit, delete, and save changes back to CSV.  
*/

mod persistence; // Declare persistence module

use cst8002_practical_project_010_pawinee_mahantamak::models;
use cst8002_practical_project_010_pawinee_mahantamak::models::natural_gas_liquid_export::NaturalGasLiquidExport;
use cst8002_practical_project_010_pawinee_mahantamak::presentation::menu::show_menu;
use persistence::csv_reader::read_csv_file;

/// The main function initializes the dataset from CSV, 
/// then launches the interactive menu system for user interaction.
fn main() {

    //Open and read the dataset
    let file_path = "data/natural-gas-liquids-exports-monthly.csv";

    /// Displays the author's name (per requirement)
    const MY_FULL_NAME: &str = "Pawinee Mahantamak";
   
    println!("Author: {}", MY_FULL_NAME);
    /// Initialize dataset (load up to 100 records)
    let mut records: Vec<NaturalGasLiquidExport> = match read_csv_file(file_path) {
        Ok(boxed_records) => {
            boxed_records.into_iter()
                .filter_map(|record| record.as_any().downcast_ref::<NaturalGasLiquidExport>().cloned())
                .take(100) // Limit to first 100 records
                .collect()
        }
        Err(e) => {
            eprintln!("Error loading dataset: {}", e);
            Vec::new()
        }
    };

    /// Launches the CLI-based menu for user interaction
    show_menu(file_path);
 

}
