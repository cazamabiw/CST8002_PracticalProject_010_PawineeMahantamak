/* 
Author: Pawinee Mahantamak
Course: CST8002 - Practical Project 3
Professor: Stanley Pieda
Due Date: 2025-03-16
*/

/* 
Description:
The `ExportRecord` trait serves as a base interface/super class for different types of export records. 
This enables polymorphism by allowing multiple implementations of record display formats.
Applying SOLID Principles:
- Open/Closed Principle (O): New record types can be added without modifying existing code.
- Dependency Inversion (D): High-level modules depend on abstractions rather than concrete implementations.
*/

use std::any::Any;

/// Defines an interface/super class for all export records.
pub trait ExportRecord {
    /// A method that must be implemented by all record types to define how they should be displayed.
    fn display(&self) -> String;
    /// Enables runtime type checking for downcasting in polymorphic operations.
    fn as_any(&self) -> &dyn Any;
}
