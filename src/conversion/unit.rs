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
        let unit_type = self.extract_unit_type();

        let conversion: String = match unit_type {
            UnitType::Currency => crate::conversion::currency::convert(self)?,
            UnitType::Weight => crate::conversion::weight::convert(self),
            UnitType::Length => crate::conversion::distance::convert(self),
            UnitType::Temperature => crate::conversion::temp::convert(self),
            UnitType::Volume => crate::conversion::volume::convert(self),
        };
        return Ok(conversion);
    }

    fn extract_unit_type(&self) -> UnitType {
        let unit_type = match self.base.as_str() {
            "kg" | "g" | "lb" | "lbs" | "oz" | "st" => UnitType::Weight,
            "m" | "cm" | "in" | "ft" | "yd" | "mi" => UnitType::Length,
            "°C" | "°F" | "K" => UnitType::Temperature,
            "L" | "l" | "gal" | "galUK" | "galUS" | "pt" => UnitType::Volume,
            _ => UnitType::Currency,
        };
        unit_type
    }
    pub(super) fn parse(text: &str) -> Result<Unit, String> {
        let mut value_chars = Vec::new();
        let mut unit = Vec::new();

        for ch in text.chars() {
            if ch == ' ' {
                continue;
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversionResponse {
    pub base: String,
    pub date: String,
    pub rates: HashMap<String, f64>,
    pub success: bool,
    pub timestamp: i64,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let unit = Unit::parse("1 kg").unwrap();
        assert_eq!(unit.base, "kg");
        assert_eq!(unit.value, 1.0);
    }
    #[test]
    fn test_parse_with_spaces() {
        let unit = Unit::parse("1 kg ").unwrap();
        assert_eq!(unit.base, "kg");
        assert_eq!(unit.value, 1.0);
    }
    #[test]
    fn test_parse_with_commas() {
        let unit = Unit::parse("1,000 kg").unwrap();
        assert_eq!(unit.base, "kg");
        assert_eq!(unit.value, 1000.0);
    }
    #[test]
    fn test_parse_with_multiple_spaces() {
        let unit = Unit::parse("1  kg").unwrap();
        assert_eq!(unit.base, "kg");
        assert_eq!(unit.value, 1.0);
    }
    #[test]
    fn test_parse_with_no_spaces() {
        let unit = Unit::parse("1kg").unwrap();
        assert_eq!(unit.base, "kg");
        assert_eq!(unit.value, 1.0);
    }
    #[test]
    fn test_parse_with_no_value() {
        let unit = Unit::parse("kg");
        assert!(unit.is_err());
    }
    #[test]
    fn test_parse_with_no_unit() {
        let unit = Unit::parse("1");
        assert!(unit.is_err());
    }
    #[test]
    fn test_parse_with_nothing() {
        let unit = Unit::parse("");
        assert!(unit.is_err());
    }
    #[test]
    fn test_parse_with_nothing_but_spaces() {
        let unit = Unit::parse("   ");
        assert!(unit.is_err());
    }
    #[test]
    fn test_parse_with_nothing_but_commas() {
        let unit = Unit::parse(",,,");
        assert!(unit.is_err());
    }
    #[test]
    fn test_parse_with_nothing_but_spaces_and_commas() {
        let unit = Unit::parse(" , , , ");
        assert!(unit.is_err());
    }
    #[test]
    fn test_parse_with_nothing_but_spaces_and_commas_and_value() {
        let unit = Unit::parse(" , , , 1");
        assert!(unit.is_err());
    }
    #[test]
    fn test_parse_with_nothing_but_spaces_and_commas_and_unit() {
        let unit = Unit::parse(" , , , kg");
        assert!(unit.is_err());
    }

    #[test]
    fn test_parse_with_nothing_but_spaces_and_commas_and_value_and_unit() {
        let unit = Unit::parse(" , , , 1 kg").unwrap();
        assert_eq!(unit.base, "kg");
        assert_eq!(unit.value, 1.0);
    }
    #[test]
    fn test_parse_with_nothing_but_spaces_and_commas_and_value_and_unit_and_spaces() {
        let unit = Unit::parse(" , , , 1 kg ").unwrap();
        assert_eq!(unit.base, "kg");
        assert_eq!(unit.value, 1.0);
    }
    #[test]
    fn test_parse_with_nothing_but_spaces_and_commas_and_value_and_unit_and_spaces_and_commas() {
        let unit = Unit::parse(" , , , 1 kg ,").unwrap();
        assert_eq!(unit.base, "kg");
        assert_eq!(unit.value, 1.0);
    }
    #[test]
    fn test_parse_with_nothing_but_spaces_and_commas_and_value_and_unit_and_spaces_and_commas_and_more_spaces(
    ) {
        let unit = Unit::parse(" , , , 1 kg ,  ").unwrap();
        assert_eq!(unit.base, "kg");
        assert_eq!(unit.value, 1.0);
    }
    #[test]
    fn test_parse_with_nothing_but_spaces_and_commas_and_value_and_unit_and_spaces_and_commas_and_more_spaces_and_more_commas(
    ) {
        let unit = Unit::parse(" , , , 1 kg ,  ,  ").unwrap();
        assert_eq!(unit.base, "kg");
        assert_eq!(unit.value, 1.0);
    }
    #[test]
    fn test_parse_with_nothing_but_spaces_and_commas_and_value_and_unit_and_spaces_and_commas_and_more_spaces_and_more_commas_and_value(
    ) {
        let unit = Unit::parse(" , , , 1 kg ,  ,  , 1").unwrap();
        assert_eq!(unit.base, "kg");
        assert_eq!(unit.value, 1.0);
    }
    #[test]
    fn test_parse_with_nothing_but_spaces_and_commas_and_value_and_unit_and_spaces_and_commas_and_more_spaces_and_more_commas_and_value_and_unit(
    ) {
        let unit = Unit::parse(" , , , 1 kg ,  ,  , 1 kg").unwrap();
        assert_eq!(unit.base, "kg");
        assert_eq!(unit.value, 1.0);
    }
    #[test]
    fn test_parse_with_nothing_but_spaces_and_commas_and_value_and_unit_and_spaces_and_commas_and_more_spaces_and_more_commas_and_value_and_unit_and_spaces(
    ) {
        let unit = Unit::parse(" , , , 1 kg ,  ,  , 1 kg ").unwrap();
        assert_eq!(unit.base, "kg");
        assert_eq!(unit.value, 1.0);
    }
}
