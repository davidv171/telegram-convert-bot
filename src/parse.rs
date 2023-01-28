use crate::unit::ConversionResponse;
use crate::unit::Unit;

pub(crate) async fn conversion(text: String) -> String {
    let converted = convert_currency(text.as_str()).await;
    match converted {
        Ok(_) => converted.unwrap(),
        Err(_) => converted.unwrap_err(),
    }
}

async fn convert_currency(text: &str) -> Result<String, String> {
    println!("Parsing!");
    let currency = Unit::parse_currency(text)?;
    println!("Got the API key, sending req");

    let conversions = get_conversions(&currency).await?;
    let calculated_conversions = calc_conversions(&currency, conversions);
    Ok(calculated_conversions)
}

fn calc_conversions(currency: &Unit, conversions: ConversionResponse) -> String {
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

async fn get_conversions(currency: &Unit) -> Result<ConversionResponse, String> {
    println!("Using api key");
    let api_key = std::env::var("EXCHANGE_API_KEY").unwrap();
    println!("{}", api_key);
    let client = reqwest::Client::new();
    let supported_currencies = std::env::var("SUPPORTED_CURRENCIES").unwrap();
    let mut currencies:Vec<&str> = supported_currencies.split(",").collect();
    currencies.remove(currencies.iter().position(|&r| r == currency.base).unwrap());
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

#[cfg(test)]
mod tests {
    //remove itself
    use super::*;

}
