use crate::unit::Unit;

pub(crate) fn convert(unit: &Unit) -> String {
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
