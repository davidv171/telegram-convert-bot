use crate::conversion::conversion_cache::CONVERSION_CACHE;
use crate::conversion::unit::{ConversionResponse, Unit};
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::time::Instant;

pub fn convert(unit: &Unit) -> Result<String, String> {
    let accommodated_unit = accommodate_symbols(&unit);

    Ok(calc_cached_conversions_usd_base(&accommodated_unit)?)
}

fn calc_cached_conversions_usd_base(unit: &Unit) -> Result<String, String> {
    let conversions: std::collections::HashMap<String, f64> = CONVERSION_CACHE
        .lock()
        .unwrap()
        .get_conversion_table()
        .clone();
    let supported_currencies = std::env::var("SUPPORTED_CURRENCIES").unwrap();
    let last_updated = CONVERSION_CACHE.lock().unwrap().last_updated();
    create_cached_conversions(unit, conversions, supported_currencies, last_updated)
}

fn create_cached_conversions(
    unit: &Unit,
    conversions: HashMap<String, f64>,
    supported_currencies: String,
    last_updated: Instant,
) -> Result<String, String> {
    let mut supported_currencies: Vec<&str> = supported_currencies.split(',').collect();
    supported_currencies.retain(|&x| x != unit.base);

    let mut result = format!("{} {} is\n", unit.value, unit.base);
    let Some(base_to_usd) = conversions.get(&unit.base) else {
        return Err(format!(
            "Couldn't find the base currency {} in the conversion table",
            unit.base
        ));
    };

    for currency in supported_currencies {
        let Some(usd) = conversions.get(currency) else {
            return Err(format!("Couldn't find the currency {} in the conversion table",
            currency
            ))};
        let converted = (unit.value) / base_to_usd * usd;
        result.push_str(&format!("{:.3} {}\n", converted, currency));
    }
    let seconds = last_updated.elapsed().as_secs() % 60;
    let minutes = (last_updated.elapsed().as_secs() / 60) % 60;
    let hours = (last_updated.elapsed().as_secs() / 60) / 60;

    result.push_str(&format!(
        "last updated {}h {}m {}s ago",
        hours, minutes, seconds
    ));
    Ok(result)
}

fn accommodate_symbols(unit: &Unit) -> Unit {
    let base = match unit.base.as_str() {
        "€" | "EURO" => "EUR".to_string(),
        "$" | "$USD" | "USD$" | "US$" | "$US" => "USD".to_string(),
        "£" | "£GBP" | "GBP£" => "GBP".to_string(),
        "¥" => "JPY".to_string(),
        "₽" => "RUB".to_string(),
        "A$" | "AU$" => "AUD".to_string(),
        y => y.to_string().to_uppercase(),
    };
    Unit {
        base,
        value: unit.value,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_accommodate_symbols() {
        let mut unit = accommodate_symbols(&Unit {
            base: "$".to_string(),
            value: 1.0,
        });
        assert_eq!(unit.base, "USD");

        unit = accommodate_symbols(&Unit {
            base: "$USD".to_string(),
            value: 1.0,
        });

        unit = accommodate_symbols(&Unit {
            base: "€".to_string(),
            value: 1.0,
        });
        assert_eq!(unit.base, "EUR");

        unit = accommodate_symbols(&Unit {
            base: "£".to_string(),
            value: 1.0,
        });
        assert_eq!(unit.base, "GBP");

        unit = accommodate_symbols(&Unit {
            base: "¥".to_string(),
            value: 1.0,
        });
        assert_eq!(unit.base, "JPY");

        unit = accommodate_symbols(&Unit {
            base: "₽".to_string(),
            value: 1.0,
        });
        assert_eq!(unit.base, "RUB");

        unit = accommodate_symbols(&Unit {
            base: "A$".to_string(),
            value: 1.0,
        });
        assert_eq!(unit.base, "AUD");
    }
    #[test]
    fn test_cached_conversions() {
        let mut conversions = HashMap::new();
        conversions.insert("USD".to_string(), 1.0);
        conversions.insert("EUR".to_string(), 0.9);
        conversions.insert("GBP".to_string(), 0.8);
        conversions.insert("JPY".to_string(), 110.0);
        conversions.insert("RUB".to_string(), 70.0);
        conversions.insert("AUD".to_string(), 1.5);
        let unit = Unit {
            base: "USD".to_string(),
            value: 1.0,
        };
        let supported_currencies = "USD,EUR,GBP,JPY,RUB,AUD".to_string();
        let last_updated = Instant::now();
        let res = create_cached_conversions(&unit, conversions, supported_currencies, last_updated);
        assert!(res.is_ok());
        println!("RES: {}", &res.unwrap());
    }
}
