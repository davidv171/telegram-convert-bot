use crate::conversion::unit::Unit;

pub mod conversion_cache;
mod currency;
mod distance;
mod temp;
mod unit;
mod volume;
mod weight;

pub(crate) async fn convert(text: &str) -> Result<String, String> {
    let parsed = Unit::parse(text);
    let converted = parsed?.convert().await?;
    Ok(converted)
}
