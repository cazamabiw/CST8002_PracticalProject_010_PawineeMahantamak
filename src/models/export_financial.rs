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

use std::any::Any;

use crate::models::export_record::ExportRecord;

use super::natural_gas_liquid_export::NaturalGasLiquidExport;

#[derive(Debug)]
pub struct ExportFinancial {
    pub period: String,
    pub year: u16,
    pub product: String,
    pub value_cad: f64,
    pub value_usd: f64,
}
impl ExportFinancial {
    pub fn from_full_record(record: &NaturalGasLiquidExport) -> Self {
        ExportFinancial {
            period: record.period().to_string(),
            year: record.year(),
            product: record.product().to_string(),
            value_cad: record.value_cad(),
            value_usd: record.value_usd(),
        }
    }
}
impl ExportRecord for ExportFinancial {
    /// Returns a financial summary of the export record.
    fn display(&self) -> String {
        format!(
            "[Financial] Period: {}, Year: {}, Product: {}, Value (CAD): ${}, Value (USD): ${}",
            self.period, self.year, self.product, self.value_cad, self.value_usd
        )
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}
