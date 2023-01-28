use crate::conversion::unit::{ConversionResponse, Unit};

pub async fn convert(unit: &Unit) -> Result<String, String> {
    println!("Parsing!");
    println!("Got the API key, sending req");

    let accommodated_unit = accommodate_symbols(&unit);

    let conversions = get_currency_conversions(&accommodated_unit).await?;

    let calculated_conversions = calc_currency_conversions(&accommodated_unit, conversions);

    Ok(calculated_conversions)
}

fn calc_currency_conversions(currency: &Unit, conversions: ConversionResponse) -> String {
    let mut result = format!("{} {} is\n", currency.value, currency.base);
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
