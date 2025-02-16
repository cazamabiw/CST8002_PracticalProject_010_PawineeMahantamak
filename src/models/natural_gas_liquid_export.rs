/* 
Author: Pawinee Mahantamak
Course: CST8002 - Practical Project 2
Professor: Stanley Pieda
Due Date: 2025-02-16
*/

/* Description:
This struct holds data about natural gas liquid exports. It includes fields for the period, year, 
month, product, origin, destination, transportation mode, and various financial and volume details. 
It provides methods for creating a new record, accessing field values, and setting or updating those fields.
*/
#[derive(Debug)]
pub struct NaturalGasLiquidExport {
    period: String,
    year: u16,
    month: String,
    product: String,
    origin: String,
    destination: String,
    mode_of_transport: String,
    volume_m3: f64,
    volume_bbl: f64,
    value_cad: f64,
    value_usd: f64,
    price_per_l_cents: f64,
    price_per_gal_cents: f64,
}

// Constants for dataset column names
pub const NATURALGASLIQUID_HEADERS: [&str; 13] = [
    "Period", "Year", "Month", "Product", "Origin", "Destination",
    "Mode of Tr.", "Volume (m³)", "Volume (bbl)", "Value (CN$)",
    "Value (US$)", "Price (CN cents/L)", "Price (US cents/gallon)"
];

impl NaturalGasLiquidExport {
    // Constructor
    pub fn new(
        period: String,
        year: u16,
        month: String                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               ,
        product: String,
        origin: String,
        destination: String,
        mode_of_transport: String,
        volume_m3: f64,
        volume_bbl: f64,
        value_cad: f64,
        value_usd: f64,
        price_per_l_cents: f64,
        price_per_gal_cents: f64,
    ) -> Self {
        Self {
            period,
            year,
            month,
            product,
            origin,
            destination,
            mode_of_transport,
            volume_m3,
            volume_bbl,
            value_cad,
            value_usd,
            price_per_l_cents,
            price_per_gal_cents,
        }
    }

    // Getters
    pub fn period(&self) -> &str {
        &self.period
    }

    pub fn year(&self) -> u16 {
        self.year
    }

    pub fn month(&self) -> &str {
        &self.month
    }

    pub fn product(&self) -> &str {
        &self.product
    }

    pub fn origin(&self) -> &str {
        &self.origin
    }

    pub fn destination(&self) -> &str {
        &self.destination
    }

    pub fn mode_of_transport(&self) -> &str {
        &self.mode_of_transport
    }

    pub fn volume_m3(&self) -> f64 {
        self.volume_m3
    }

    pub fn volume_bbl(&self) -> f64 {
        self.volume_bbl
    }

    pub fn value_cad(&self) -> f64 {
        self.value_cad
    }

    pub fn value_usd(&self) -> f64 {
        self.value_usd
    }

    pub fn price_per_l_cents(&self) -> f64 {
        self.price_per_l_cents
    }

    pub fn price_per_gal_cents(&self) -> f64 {
        self.price_per_gal_cents
    }

    // Setters
    pub fn set_period(&mut self, period: String) {
        self.period = period;
    }

    pub fn set_year(&mut self, year: u16) {
        self.year = year;
    }

    pub fn set_month(&mut self, month: String) {
        self.month = month;
    }

    pub fn set_product(&mut self, product: String) {
        self.product = product;
    }

    pub fn set_origin(&mut self, origin: String) {
        self.origin = origin;
    }

    pub fn set_destination(&mut self, destination: String) {
        self.destination = destination;
    }

    pub fn set_mode_of_transport(&mut self, mode_of_transport: String) {
        self.mode_of_transport = mode_of_transport;
    }

    pub fn set_volume_m3(&mut self, volume_m3: f64) {
        self.volume_m3 = volume_m3;
    }

    pub fn set_volume_bbl(&mut self, volume_bbl: f64) {
        self.volume_bbl = volume_bbl;
    }

    pub fn set_value_cad(&mut self, value_cad: f64) {
        self.value_cad = value_cad;
    }

    pub fn set_value_usd(&mut self, value_usd: f64) {
        self.value_usd = value_usd;
    }

    pub fn set_price_per_l_cents(&mut self, price_per_l_cents: f64) {
        self.price_per_l_cents = price_per_l_cents;
    }

    pub fn set_price_per_gal_cents(&mut self, price_per_gal_cents: f64) {
        self.price_per_gal_cents = price_per_gal_cents;
    }
}
