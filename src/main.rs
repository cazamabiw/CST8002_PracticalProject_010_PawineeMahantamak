//Author: Pawinee Mahantamak
use CST8002_PracticalProject_010_PawineeMahantamak::utility::csv_reader::read_csv_file;

fn main() {

    //Open and read the dataset
    let file_path = "data/natural-gas-liquids-exports-monthly.csv";

    match read_csv_file(file_path) {
        Ok(records) => {
            println!("Successfully read {} records:", records.len());
            //Loop over the data structure and output the record data on screen.
            for record in records.iter() {
                println!("{:?}", record);
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }


}
