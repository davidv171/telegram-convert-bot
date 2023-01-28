use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;

pub struct Unit {
    pub base: String,
    pub value: f64,
}

impl Unit {
    pub fn parse_currency(text: &str) -> Result<Self, String> {
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
            .map_err(| _e | format!("Missing value, no numbers?"))?;

        let base = match unit.iter().collect::<String>().as_str() {
            "€"  => Some("EUR".to_string()),
            "$"  => Some("USD".to_string()),
            "£"  => Some("GBP".to_string()),
            "¥"  => Some("JPY".to_string()),
            "₽"  => Some("RUB".to_string()),
            "A$" | "AU$" => Some("AUD".to_string()),
            "" => None,
            y => Some(y.to_string()),
        }.ok_or(format!("Missing unit, no currency symbol?"))?;

        Ok(Self {
            base,
            value
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversionResponse {
    pub base: String,
    pub date: String,
    pub rates: HashMap<String, f64>,
    pub success: bool,
    pub timestamp: i64,
}
