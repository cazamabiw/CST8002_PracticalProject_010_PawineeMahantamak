/* 
Author: Pawinee Mahantamak
Course: CST8002 - Practical Project 3
Professor: Stanley Pieda
Due Date: 2025-03-16
*/

/* 
Description:
The ExportFinancial struct focuses on financial details of exports.
It implements the ExportRecord trait for a custom display format.
Applying SOLID Principles:
- Open/Closed Principle (O): This new type extends behavior without modifying base classes.
*/

use crate::models::export_record::ExportRecord;

#[derive(Debug)]
pub struct ExportFinancial {
    pub period: String,
    pub year: u16,
    pub product: String,
    pub value_cad: f64,
    pub value_usd: f64,
}

impl ExportRecord for ExportFinancial {
    /// Returns a financial summary of the export record.
    fn display(&self) -> String {
        format!(
            "[Financial] Period: {}, Year: {}, Product: {}, Value (CAD): ${}, Value (USD): ${}",
            self.period, self.year, self.product, self.value_cad, self.value_usd
        )
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }
}
