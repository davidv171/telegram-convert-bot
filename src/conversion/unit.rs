use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

pub struct Unit {
    pub base: String,
    pub value: f64,
}

pub enum UnitType {
    Weight,
    Length,
    Temperature,
    Currency,
    Volume,
}
impl Unit {
    pub(super) async fn convert(&self) -> Result<String, String> {
        let unit_type = match self.base.as_str() {
            "kg" | "g" | "lb" | "oz" | "st" => UnitType::Weight,
            "m" | "cm" | "in" | "ft" | "yd" | "mi" => UnitType::Length,
            "°C" | "°F" | "K" => UnitType::Temperature,
            "L" | "l" | "gal" | "galUK" | "galUS" | "pt" => UnitType::Volume,
            _ => UnitType::Currency,
        };
        let conversion: String = match unit_type {
            UnitType::Currency => crate::conversion::currency::convert(self).await?,
            UnitType::Weight => crate::conversion::weight::convert(self),
            UnitType::Length => crate::conversion::distance::convert(self),
            UnitType::Temperature => crate::conversion::temp::convert(self),
            UnitType::Volume => crate::conversion::volume::convert(self),
        };
        return Ok(conversion);
    }
    pub(super) fn parse(text: &str) -> Result<Unit, String> {
        let mut value_chars = Vec::new();
        let mut unit = Vec::new();

        for ch in text.chars() {
            if ch == ' ' {
                break;
            }

            if ch.is_numeric() || ch == '.' {
                value_chars.push(ch);
                continue;
            }

            if ch == ',' {
                continue;
            }

            unit.push(ch);
        }
        let value = value_chars
            .iter()
            .collect::<String>()
            .parse::<f64>()
            .map_err(|_e| format!("Missing value, no numbers?"))?;

        let base: String = unit.iter().collect::<String>();
        if base.trim().is_empty() {
            return Err("Missing unit, no letters?".to_string());
        }
        let unit: Unit = Unit { base, value };
        Ok(unit)
    }
    // Thank you github copilot
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversionResponse {
    pub base: String,
    pub date: String,
    pub rates: HashMap<String, f64>,
    pub success: bool,
    pub timestamp: i64,
}
