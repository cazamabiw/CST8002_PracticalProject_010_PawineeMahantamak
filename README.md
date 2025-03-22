# Natural Gas Liquid Export Management System

## Course: CST8002 - Programming Language Research Project  
**Author:** Pawinee Mahantamak  
**Professor:** Stanley Pieda  
**Project Type:** Practical project  
**Language:** Rust

---

## Project Overview
This project is a **Natural Gas Liquid Export Management System** that processes **CSV data**, supports **CRUD operations**, and follows the **N-Layered Architecture**.

### Key Features
**Reads data from CSV (up to 100 records initially)**  
**Stores data in memory and supports Add/Edit/Delete operations**  
**Displays records interactively with a command-line menu**  
**Saves updated data back to CSV (with a unique UUID filename)**  
**Implements error handling and unit testing**  

---

## Practical 1: Initial Project Implementation
In **Practical 1**, the system was built with **basic functionality**:
1. **Defined the data model** (`NaturalGasLiquidExport`)
2. **Implemented CSV reading functionality**
3. **Displayed records from the dataset**
4. **Structured the project using Rust modules**


---

## Practical 2: Advanced Features & Architecture
In **Practical 2**, the project was refactored into a **fully structured N-Layered Architecture**, with added **CRUD functionality** and **user interaction**.

1. **Implemented N-Layered Architecture**  
2. **Created a CLI-based interactive menu**  
3. **Enabled adding, editing, and deleting records in memory**  
4. **Persisted data back to a CSV file using UUID filenames**  
5. **Implemented unit tests for core functions**  

---


## Practical 3: Object-Oriented Design & Polymorphism
In **Practical 3**, the code was extended to apply **Object-Oriented Programming (OOP) principles** using traits and **polymorphism** in Rust. The project now supports multiple views of the export data (full, summary, and financial) by defining a trait and implementing it across different record types.

1. **Defined the ExportRecord trait as an abstract interface (superclass) with a display() method and dynamic type support via as_any()**
2. **Implemented three concrete record types that override the trait methods:**
  - NaturalGasLiquidExport – full record view
  - ExportSummary – summarized data (e.g., period, product, volume)
  - ExportFinancial – financial data (e.g., CAD and USD values)

3. **Refactored the display functionality to use dynamic dispatch (Box<dyn ExportRecord>) for runtime polymorphism**

4. **Enhanced the menu to allow users to select record type (full, summary, financial) when viewing data**

5. **Used downcasting via as_any() to convert trait objects into concrete types inside the manager.rs module**

6. **Maintained clean separation of concerns by following the N-Layered Architecture (presentation, business, persistence, model)**

7. **Applied SOLID Principles:**
- **Single Responsibility Principle:** Each struct (ExportSummary, ExportFinancial, etc.) is focused on one specific view
- **Open/Closed Principle:** New record types were added without modifying existing logic
- **Liskov Substitution Principle:** Trait objects are used interchangeably across modules
- **Interface Segregation Principle:** ExportRecord trait includes only essential methods
- **Dependency Inversion Principle:** Business logic in manager.rs relies on abstractions (ExportRecord), not concrete implementations


