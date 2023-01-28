use crate::unit::Unit;
use log::max_level;

pub(crate) fn convert(unit: &Unit) -> String {
    let mut result = format!("{} {} is\n", unit.value, unit.base);
    match unit.base.as_str() {
        "L" | "l" => {
            result.push_str(&format!("{:.3} pt\n", unit.value * 2.11338));
            result.push_str(&format!("{:.3} barrel\n", unit.value * 0.00628981));
            result.push_str(&format!("{:.3} galUK\n", unit.value * 0.219969));
            result.push_str(&format!("{:.3} galUS\n", unit.value * 0.264172));
        }
        "gal" | "galUS" => {
            result.push_str(&format!("{:.3} L\n", unit.value * 3.78541));
            result.push_str(&format!("{:.3} pt\n", unit.value * 8.0));
            result.push_str(&format!("{:.3} barrel\n", unit.value * 0.0238095));
            result.push_str(&format!("{:.3} galUK\n", unit.value * 0.832674));
        }
        "pt" => {
            result.push_str(&format!("{:.3} L\n", unit.value * 0.473176));
            result.push_str(&format!("{:.3} barrel\n", unit.value * 0.00297619));
            result.push_str(&format!("{:.3} galUK\n", unit.value * 0.104084));
            result.push_str(&format!("{:.3} galUS\n", unit.value * 0.125));
        }
        "barrel" => {
            result.push_str(&format!("{:.3} L\n", unit.value * 158.987));
            result.push_str(&format!("{:.3} galUS\n", unit.value * 42.0));
            result.push_str(&format!("{:.3} pt\n", unit.value * 336.0));
            result.push_str(&format!("{:.3} galUK\n", unit.value * 0.832674));
        }
        "galUK" => {
            result.push_str(&format!("{:.3} L\n", unit.value * 4.54609));
            result.push_str(&format!("{:.3} galUS\n", unit.value * 1.20095));
            result.push_str(&format!("{:.3} pt\n", unit.value * 9.6078));
            result.push_str(&format!("{:.3} barrel\n", unit.value * 0.119240));
        }
        _ => {}
    }
    result
}
