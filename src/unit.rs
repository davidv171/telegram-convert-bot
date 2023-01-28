use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;

pub struct Unit {
    pub base: String,
    pub value: f64,
}

pub enum UnitType {
    Weight,
    Length,
    Temperature,
    Currency,
}


impl Unit {
    pub async fn convert(text: &str) -> Result<String, String> {
        let unit = Unit::parse(text)?;
        let unit_type = match unit.base.as_str() {
            "kg" | "g" | "lb" | "oz" => UnitType::Weight,
            "m" | "cm" | "in" | "ft" | "yd" | "mi" => UnitType::Length,
            "°C" | "°F" | "K" => UnitType::Temperature,
            _ => UnitType::Currency,
        };
        let conversion: String = match unit_type {
            UnitType::Currency => {
                Unit::convert_currency(&unit).await?
            }
            UnitType::Weight => {
                Unit::convert_weight(&unit)
            }
            UnitType::Length => {
                Unit::convert_length(&unit)
            }
            UnitType::Temperature => {
                Unit::convert_temperature(&unit)
            }
        };
        return Ok(conversion);
    }
    fn parse(text: &str) -> Result<Unit, String> {
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
        let unit: Unit = Unit {
            base,
            value,
        };
        Ok(unit)
    }
    async fn convert_currency(unit: &Unit) -> Result<String, String> {
        println!("Parsing!");
        println!("Got the API key, sending req");

        let accommodated_unit = Unit::accommodate_symbols(&unit);
        let conversions = Unit::get_currency_conversions(&accommodated_unit).await?;
        let calculated_conversions = Unit::calc_currency_conversions(&accommodated_unit, conversions);
        Ok(calculated_conversions)
    }

    fn calc_currency_conversions(currency: &Unit, conversions: ConversionResponse) -> String {
        let mut result = format!(
            "{} {} is\n",
            currency.value, currency.base
        );
        for (key, value) in conversions.rates {
            result.push_str(&format!("{:.3} {}\n", currency.value * value, key));
        }
        result.push_str(&format!("at {}", conversions.date));
        result
    }

    async fn get_currency_conversions(currency: &Unit) -> Result<ConversionResponse, String> {
        println!("Using api key");
        let api_key = std::env::var("EXCHANGE_API_KEY").unwrap();
        println!("{}", api_key);
        let client = reqwest::Client::new();
        let supported_currencies = std::env::var("SUPPORTED_CURRENCIES").unwrap();
        let mut currencies: Vec<&str> = supported_currencies.split(",").collect();
        currencies.retain(|&x| x != currency.base);
        println!("currencies = {:?}", currencies);

        let res = client
            .get("https://api.apilayer.com/exchangerates_data/latest")
            .header("apikey", api_key)
            .query(&[
                ("symbols", currencies.join(",").as_str()),
                ("base", &currency.base),
            ])
            .send()
            .await
            .map_err(|e| format!("Couldn't send request properly {:?}", e))?;

        if !res.status().is_success() {
            println!("Something else happened. Status: {:?}", res.status());
            return Err(format!(
                "HTTP request failed with {:?}, couldn't get conversion",
                res.status()
            ));
        }

        let conversion_response = res
            .json::<ConversionResponse>()
            .await
            .map_err(|e| format!("Couldn't read the JSON response properly {:?}", e))?;

        Ok(conversion_response)
    }
    fn accommodate_symbols(unit: &Unit) -> Unit {
        let base = match unit.base.as_str() {
            "€" => "EUR".to_string(),
            "$" => "USD".to_string(),
            "£" => "GBP".to_string(),
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
    // Thank you github copilot
    fn convert_weight(unit: &Unit) -> String {
        let mut result = format!(
            "{} {} is\n",
            unit.value, unit.base
        );
        match unit.base.as_str() {
            "kg" => {
                result.push_str(&format!("{:.3} g\n", unit.value * 1000.0));
                result.push_str(&format!("{:.3} lb\n", unit.value * 2.20462));
                result.push_str(&format!("{:.3} oz\n", unit.value * 35.274));
            }
            "g" => {
                result.push_str(&format!("{:.3} kg\n", unit.value * 0.001));
                result.push_str(&format!("{:.3} lb\n", unit.value * 0.00220462));
                result.push_str(&format!("{:.3} oz\n", unit.value * 0.035274));
            }
            "lb" => {
                result.push_str(&format!("{:.3} kg\n", unit.value * 0.453592));
                result.push_str(&format!("{:.3} g\n", unit.value * 453.592));
                result.push_str(&format!("{:.3} oz\n", unit.value * 16.0));
            }
            "oz" => {
                result.push_str(&format!("{:.3} kg\n", unit.value * 0.0283495));
                result.push_str(&format!("{:.3} g\n", unit.value * 28.3495));
                result.push_str(&format!("{:.3} lb\n", unit.value * 0.0625));
            }
            _ => {}
        }
        result
    }
    fn convert_length(unit: &Unit) -> String {
        let mut result = format!(
            "{} {} is\n",
            unit.value, unit.base
        );
        match unit.base.as_str() {
            "m" => {
                result.push_str(&format!("{:.3} ft\n", unit.value * 3.28084));
                result.push_str(&format!("{:.3} in\n", unit.value * 39.3701));
            }
            "cm" => {
                result.push_str(&format!("{:.3} ft\n", unit.value * 0.0328084));
                result.push_str(&format!("{:.3} in\n", unit.value * 0.393701));
            }
            "mm" => {
                result.push_str(&format!("{:.3} ft\n", unit.value * 0.00328084));
                result.push_str(&format!("{:.3} in\n", unit.value * 0.0393701));
            }
            "ft" => {
                result.push_str(&format!("{:.3} m\n", unit.value * 0.3048));
                result.push_str(&format!("{:.3} cm\n", unit.value * 30.48));
                result.push_str(&format!("{:.3} mm\n", unit.value * 304.8));
                result.push_str(&format!("{:.3} in\n", unit.value * 12.0));
            }
            "yd" => {
                result.push_str(&format!("{:.3} m\n", unit.value * 0.9144));
                result.push_str(&format!("{:.3} cm\n", unit.value * 91.44));
                result.push_str(&format!("{:.3} mm\n", unit.value * 914.4));
                result.push_str(&format!("{:.3} ft\n", unit.value * 3.0));
                result.push_str(&format!("{:.3} in\n", unit.value * 36.0));
            }
            _ => {}
        }
        result
    }
    fn convert_temperature(unit: &Unit) -> String {
        let mut result = format!(
            "{} {} is\n",
            unit.value, unit.base
        );

        match unit.base.as_str() {
            "°F" => {
                result.push_str(&format!("{:.3} °F\n", unit.value * 1.0));
                result.push_str(&format!("{:.3} °C\n", (unit.value - 32.0) * 5.0 / 9.0));
                result.push_str(&format!("{:.3} K\n", (unit.value - 32.0) * 5.0 / 9.0 + 273.15));
            }
            "°C" => {
                result.push_str(&format!("{:.3} °F\n", unit.value * 9.0 / 5.0 + 32.0));
                result.push_str(&format!("{:.3} °C\n", unit.value * 1.0));
                result.push_str(&format!("{:.3} K\n", unit.value + 273.15));
            }
            "K" => {
                result.push_str(&format!("{:.3} °F\n", (unit.value - 273.15) * 9.0 / 5.0 + 32.0));
                result.push_str(&format!("{:.3} °C\n", unit.value - 273.15));
                result.push_str(&format!("{:.3} K\n", unit.value * 1.0));
            }
            _ => {}
        }

        result
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
