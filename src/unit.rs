use serde::Deserialize;
use serde::Serialize;


pub struct Currency {
    pub base: String,
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversionResponse {
    pub base: String,
    pub date: String,
    pub rates: Rates,
    pub success: bool,
    pub timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rates {
    #[serde(rename = "GBP")]
    pub gbp: f64,
    #[serde(rename = "PLN")]
    pub pln: f64,
    #[serde(rename = "USD")]
    pub usd: f64,
    #[serde(rename = "EUR")]
    pub eur: f64,
}
