/* 
Author: Pawinee Mahantamak
Course: CST8002 - Practical Project 2
Professor: Stanley Pieda
Due Date: 2025-02-16
*/

/* 
Description:
Comprehensive unit test covering:
1. Reading CSV and verifying correct field assignment.
2. Adding a new record to the in-memory list.
3. Updating an existing record.
4. Removing a record from the list.
5. Handling missing file errors.
*/

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::persistence::csv_reader::read_csv_file;
    use crate::models::natural_gas_liquid_export::NaturalGasLiquidExport;
    use crate::business::manager::{create_record, edit_record, delete_record};

    /// Comprehensive test covering multiple aspects:
    /// - Reads CSV and checks if data is parsed correctly.
    /// - Adds a new record and verifies it was added.
    /// - Updates an existing record and checks changes.
    /// - Deletes a record and confirms removal.
    /// - Handles missing file gracefully.
    #[test]
    fn test_csv_operations_and_data_modifications() {
        let valid_file = "data/natural-gas-liquids-exports-monthly.csv";
        let invalid_file = "data/non_existent_file.csv";

        //Test reading a valid CSV file
        let mut records = match read_csv_file(valid_file) {
            Ok(records) => {
                assert!(!records.is_empty(), "CSV read successfully but no records found!");
                
                // Verify first record is populated
                let first_record = &records[0];
                assert!(!first_record.period().is_empty(), "Period should not be empty");
                assert!(first_record.year() >= 1990, "Year should be a valid number");
                assert!(!first_record.month().is_empty(), "Month should not be empty");

                records
            }
            Err(e) => panic!("Failed to read valid CSV file: {}", e),
        };

        //Test adding a new record
        let new_record = NaturalGasLiquidExport::new(
            "01-01-2025".to_string(), 2025, "January".to_string(), "Butane".to_string(),
            "Ontario".to_string(), "Total".to_string(), "Railway".to_string(),
            5000.0, 3000.0, 15000.0, 12000.0, 30.0, 90.0
        );
        let original_len = records.len();
        create_record(&mut records, new_record);
        assert_eq!(records.len(), original_len + 1, "New record should be added!");

        //Test updating an existing record
        let updated_record = NaturalGasLiquidExport::new(
            "02-01-2025".to_string(), 2025, "February".to_string(), "Propane".to_string(),
            "Quebec".to_string(), "PADD II".to_string(), "Truck".to_string(),
            6000.0, 3500.0, 18000.0, 14000.0, 35.0, 95.0
        );
        edit_record(&mut records, original_len, updated_record);
        assert_eq!(records[original_len].month(), "February", "Record should be updated!");

        //Test deleting a record
        delete_record(&mut records, original_len);
        assert_eq!(records.len(), original_len, "Record should be deleted!");

        //Test handling of a missing file
        let result = read_csv_file(invalid_file);
        assert!(result.is_err(), "Should return an error when file is missing!");
    }
}
