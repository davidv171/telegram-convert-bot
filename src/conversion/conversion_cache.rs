use crate::conversion::unit::ConversionResponse;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

pub static ALL_CURRENCIES: &str = "AED,AFN,ALL,AMD,ANG,AOA,ARS,AUD,AWG,AZN,BAM,BBD,BDT,BGN,BHD,BIF,BMD,\
BOB,BRL,BSD,BTN,BWP,BYN,BZD,CAD,CDF,CHF,CLF,CLP,CNY,COP,CRC,CUC,CUP,CVE,CZK,DJF,DKK,DZD,EGP,ERN,ETB,\
EUR,FJD,FKP,GBP,GEL,GHS,GIP,GMD,GNF,GTQ,GYD,HKD,HNL,HTG,HUF,IDR,ILS,INR,IQD,IRR,JMD,JOD,JPY,KES,KGS,\
KHR,KMF,KPW,KRW,KWD,KYD,KZT,LAK,LBP,LKR,LRD,LSL,LYD,MAD,MDL,MGA,MKD,MMK,MOP,MUR,MVR,MWK,MXN,MYR,MZN,\
NAD,NGN,NIO,NOK,NPR,NZD,OMR,PAB,PEN,PGK,PHP,PKR,PLN,PYG,QAR,RSD,RUB,RWF,SAR,SBD,SCR,SDG,SEK,SGD,SHP,\
SLE,SLL,SOS,SRD,SVC,SYP,SZL,THB,TJS,TMT,TOP,TRY,TTD,TWD,TZS,UAH,UGX,USD,UYU,UZS,VES,VND,VUV,WST,XAF,\
XAG,XAU,XCD,XDR,XOF,XPF,YER,ZAR,ZMW,ZWL";
lazy_static! {
    pub(crate) static ref CONVERSION_CACHE: Mutex<ExpiringConversionCache> =
        Mutex::new(ExpiringConversionCache::new());
}

pub(crate) struct ExpiringConversionCache {
    // The key is the currency code, the value is the exchange rate to USD (1.0 if the currency is USD)
    // If exists, convert whatever to USD, then forward based on other values
    conversion_table: HashMap<String, f64>,
    last_updated: Instant,
    client: reqwest::blocking::Client,
}

impl ExpiringConversionCache {
    pub fn new() -> Self {
        Self {
            conversion_table: HashMap::new(),
            last_updated: Instant::now(),
            client: reqwest::blocking::Client::new(),
        }
    }
    pub fn populate(&mut self) -> Result<(), String> {
        println!("Populating conversion cache");
        let conversion_response = ExpiringConversionCache::get_all_conversions(self)
            .map_err(|e| format!("Error getting conversion rates: {}", e))?;
        self.fill_cache(&conversion_response);
        Ok(())
    }
    pub fn get_conversion_table(&self) -> &HashMap<String, f64> {
        &self.conversion_table
    }
    pub fn last_updated(&self) -> Instant {
        self.last_updated
    }

    fn fill_cache(&mut self, conversion_response: &ConversionResponse) {
        println!("Refilling conversion cache");
        self.last_updated = Instant::now();
        let usd = *conversion_response.rates.get("USD").unwrap();
        // USD adjusted rates
        for (currency_code, exchange_rate) in &conversion_response.rates {
            self.add(currency_code, exchange_rate / usd);
        }
        // override the base one to USD rate instead
        self.add(conversion_response.base.as_str(), usd);
        self.last_updated = Instant::now();
    }
    fn add(&mut self, currency_code: &str, exchange_rate: f64) {
        self.conversion_table
            .insert(currency_code.to_string(), exchange_rate);
    }

    fn get_all_conversions(&self) -> Result<ConversionResponse, String> {
        println!("Getting all conversions....");
        let api_key = std::env::var("EXCHANGE_API_KEY").map_err(|e| {
            format!(
                "Couldn't get the API key from the environment variable {:?}",
                e
            )
        })?;
        let res = self
            .client
            .get("https://api.apilayer.com/exchangerates_data/latest")
            .header("apikey", api_key)
            .query(&[("symbols", ALL_CURRENCIES), ("base", "USD")])
            .send()
            .map_err(|e| format!("Couldn't send the request {:?}", e))?;

        if !res.status().is_success() {
            println!("Something else happened. Status: {:?}", res.status());
            return Err(format!(
                "HTTP request failed with {:?}, couldn't get conversion",
                res.status()
            ));
        }

        let conversion_response = res
            .json::<ConversionResponse>()
            .map_err(|e| format!("Couldn't read the JSON response properly {:?}", e))?;

        println!("Conversion res is {:?}", conversion_response);
        Ok(conversion_response)
    }
}
