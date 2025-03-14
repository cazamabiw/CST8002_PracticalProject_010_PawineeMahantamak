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
    use crate::models::export_financial::ExportFinancial;
    use crate::models::export_record::ExportRecord;
    use crate::models::export_summary::ExportSummary;
    use crate::persistence::csv_reader::read_csv_file;
    use crate::models::natural_gas_liquid_export::NaturalGasLiquidExport;
    use crate::business::manager::{add_record, delete_record, edit_record, load_initial_data};

    /// Comprehensive test covering:
    /// - Reads CSV and checks if data is parsed correctly.
    /// - Adds a new record and verifies it was added.
    /// - Updates an existing record and checks changes.
    /// - Deletes a record and confirms removal.
    /// - Handles missing file gracefully.
    #[test]
    fn test_csv_operations_and_data_modifications() {
        let valid_file = "data/natural-gas-liquids-exports-monthly.csv";
        let invalid_file = "data/non_existent_file.csv";

        // Load initial dataset (up to 100 records)
        let records = load_initial_data(valid_file).expect("Failed to load initial data");
        let total_records = records.len(); // Get current count dynamically

        assert!(
            !records.is_empty(),
            "CSV read successfully but no records found!"
        );

        // Verify first record has values
        let first_record = &records[0];
        assert!(
            !first_record.period().is_empty(),
            "Period should not be empty"
        );
        assert!(first_record.year() >= 1990, "Year should be a valid number");
        assert!(!first_record.month().is_empty(), "Month should not be empty");

        // Test adding a new record (1-based index)
        let new_record = NaturalGasLiquidExport::new(
            "01-01-2025".to_string(),
            2025,
            "January".to_string(),
            "Butane".to_string(),
            "Ontario".to_string(),
            "Total".to_string(),
            "Railway".to_string(),
            5000.0,
            3000.0,
            15000.0,
            12000.0,
            30.0,
            90.0,
        );

        let new_index = total_records + 1;
        add_record(new_record);
        assert_eq!(
            records.len(),
            total_records + 1,
            "New record should be added!"
        );

        // Test updating the newly added record
        let updated_record = NaturalGasLiquidExport::new(
            "02-01-2025".to_string(),
            2025,
            "February".to_string(),
            "Propane".to_string(),
            "Quebec".to_string(),
            "PADD II".to_string(),
            "Truck".to_string(),
            6000.0,
            3500.0,
            18000.0,
            14000.0,
            35.0,
            95.0,
        );

        edit_record(new_index, updated_record); // User enters `101` instead of `100`
        assert_eq!(
            records[new_index - 1].month(),
            "February",
            "Record should be updated!"
        );

        // Test deleting the record (1-based index)
        delete_record(new_index);
        assert_eq!(
            records.len(),
            total_records,
            "Record should be deleted!"
        );

        // Test handling of a missing file
        let result = read_csv_file(invalid_file);
        assert!(result.is_err(), "Should return an error when file is missing!");
    }

    
    /// Polymorphism Test
    /// - Ensures that different record types (`ExportSummary`, `ExportFinancial`) properly override `display()`.
    #[test]
    fn test_polymorphic_display() {
        // Create a sample full export record
        let record = NaturalGasLiquidExport::new(
            "03-01-2025".to_string(),
            2025,
            "March".to_string(),
            "Ethane".to_string(),
            "Alberta".to_string(),
            "PADD III".to_string(),
            "Pipeline".to_string(),
            7000.0,
            4000.0,
            20000.0,
            16000.0,
            40.0,
            100.0,
        );
        // Convert the full record to a `Summary` and `Financial` export record
        let summary_record: Box<dyn ExportRecord> = Box::new(ExportSummary::from_full_record(&record));
        let financial_record: Box<dyn ExportRecord> = Box::new(ExportFinancial::from_full_record(&record));
        // Ensure `display()` method is correctly overridden for summary records
        assert!(
            summary_record.display().contains("[Summary]"),
            "ExportSummary display format is incorrect!"
            
        );
        // Ensure `display()` method is correctly overridden for financial records
        assert!(
            financial_record.display().contains("[Financial]"),
            "ExportFinancial display format is incorrect!"
        );
        // Print outputs to verify correct display formatting
        println!("========================================================================================");
        println!("Full Display Output: {}", record.display());
        println!("Summary Display Output: {}", summary_record.display());
        println!("Financial Display Output: {}", financial_record.display());
        println!("Polymorphism test passed: Summary and Financial records displayed correctly.");
        println!("========================================================================================");
        println!("Author: Pawinee Mahantamak");
        println!("========================================================================================");
    }
}
