use crate::unit::ConversionResponse;
use crate::unit::Currency;

pub(crate) async fn conversion(text: String) -> String {
    println!("Converting!!");
    let converted = convert_currency(text.as_str()).await;
    converted.unwrap()
}

async fn convert_currency(text: &str) -> Result<String, reqwest::Error> {
    println!("Parsing!");
    let currency = parse_currency(text);
    println!("Got the API key, sending req");

    let conversions = get_conversions(&currency).await?;
    let calculated_conversions = calc_conversions(&currency, conversions);
    Ok(calculated_conversions)
}

fn calc_conversions(currency: &Currency, conversions: ConversionResponse) -> String {
    let usd = currency.value * conversions.rates.usd;
    let eur = currency.value * conversions.rates.eur;
    let gbp = currency.value * conversions.rates.gbp;
    let pln = currency.value * conversions.rates.pln;
    format!(
        "{} {} is\n{}USD\n{}EUR\n{}GBP\n{}PLN\nat {}",
        currency.value, currency.base, usd, eur, gbp, pln, conversions.date
    )
}

async fn get_conversions(currency: &Currency) -> Result<ConversionResponse, reqwest::Error> {
    println!("Using api key");
    let api_key = std::env::var("EXCHANGE_API_KEY").unwrap();
    println!("{}", api_key);
    let client = reqwest::Client::new();

    let res = client
        .get("https://api.apilayer.com/exchangerates_data/latest")
        .header("apikey", api_key)
        .query(&[
            ("symbols", "GBP%2CUSD%2CPLN%2CEUR"),
            ("base", &currency.base),
        ])
        .send()
        .await?;

    if res.status().is_success() {
        println!("success!");
    } else if res.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", res.status());
    }
    println!("body = {:?}", res);

    let conversion_response = res.json::<ConversionResponse>().await?;
    println!("Read response {:?}", conversion_response);
    Ok(conversion_response)
}
fn parse_currency(text: &str) -> Currency {
    println!("Parsing currency!");
    let mut value_chars: Vec<char> = Vec::new();
    let mut unit: Vec<char> = Vec::new();

    for ch in text.chars() {
        println!("parsing {}", ch);
        if ch == ' ' {
            break;
        }

        if ch.is_numeric() || ch == '.' || ch == ',' {
            value_chars.push(ch);
            continue;
        }

        unit.push(ch);
    }
    println!("Done reading out units");
    let value: f64 = value_chars.iter().collect::<String>().parse().unwrap();
    let curr = match unit.iter().collect::<String>().as_str() {
        "€" | "EUR" => Currency {
            base: "EUR".to_string(),
            value,
        },
        "$" | "USD" => Currency {
            base: "USD".to_string(),
            value,
        },
        "PLN" => Currency {
            base: "PLN".to_string(),
            value,
        },
        "GBP" | "£" => Currency {
            base: "GBP".to_string(),
            value,
        },

        _ => panic!("uhhh"),
    };
    println!("Value {}", value);
    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_currency() -> Result<(), String> {
        let y = "55.2€";
        convert_currency(y);

        Ok(())
    }
}
