/* 
Author: Pawinee Mahantamak
Course: CST8002 - Practical Project 3
Professor: Stanley Pieda
Due Date: 2025-03-16
*/

/* 
Description:
The ExportSummary struct provides a high-level summary of export data.
It overrides the display method to show key summary details.
Applying SOLID Principles:
- Open/Closed Principle (O): Extends the ExportRecord functionality without modifying it.
*/

use std::any::Any;

use crate::models::export_record::ExportRecord;

use super::natural_gas_liquid_export::NaturalGasLiquidExport;

#[derive(Debug)]
pub struct ExportSummary {
    pub period: String,
    pub year: u16,
    pub product: String,
    pub volume_m3: f64,
}

impl ExportSummary {
    pub fn from_full_record(record: &NaturalGasLiquidExport) -> Self {
        ExportSummary {
            period: record.period().to_string(),
            year: record.year(),
            product: record.product().to_string(),
            volume_m3: record.volume_m3(),
        }
    }
}
impl ExportRecord for ExportSummary {
    /// Returns a summary view of the export record.
    fn display(&self) -> String {
        format!(
            "[Summary] Period: {}, Year: {}, Product: {}, Volume: {}m³",
            self.period, self.year, self.product, self.volume_m3
        )
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}
