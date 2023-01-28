use crate::unit::ConversionResponse;
use crate::unit::Unit;

pub(crate) async fn conversion(text: String) -> String {
    let converted = Unit::convert(text.as_str()).await;
    match converted {
        Ok(_) => converted.unwrap(),
        Err(_) => converted.unwrap_err(),
    }
}
