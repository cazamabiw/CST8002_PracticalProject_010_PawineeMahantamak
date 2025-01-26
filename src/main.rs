//Author: Pawinee Mahantamak
use CST8002_PracticalProject_010_PawineeMahantamak::{
    natural_gas_liquid_export::NaturalGasLiquidExport,
   };
fn main() {
    //Create a record object
    let record = NaturalGasLiquidExport::new(
        "01-01-90".to_string(),
        1990,
        "January".to_string(),
        "Butane".to_string(),
        "Alberta".to_string(),
        "PADD II".to_string(),
        "Railway".to_string(),
        995.0,
        6258.351,
        131571.5,
        113223.7964,
        13.223266,
        42.732711,
    );
    // Access data using getters
    println!("Period: {}", record.period());
    println!("Year: {}", record.year());

}
