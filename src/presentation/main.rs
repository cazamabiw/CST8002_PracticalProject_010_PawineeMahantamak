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

use CST8002_PracticalProject_010_PawineeMahantamak::utility::csv_reader::read_csv_file;

fn main() {

    //Open and read the dataset
    let file_path = "data/natural-gas-liquids-exports-monthly.csv";

     //Display my full name (always visible)
    const MY_FULL_NAME: &str = "Pawinee Mahantamak";
   

    match read_csv_file(file_path) {
        Ok(records) => {

            //Loop over the data structure and output the record data on screen.
            for record in records.iter() {
                println!("{:?}", record);
            }
            println!("Successfully read {} records", records.len());
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
    println!("Author: {}", MY_FULL_NAME);

}
