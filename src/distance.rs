use crate::unit::Unit;

pub(crate) fn convert(unit: &Unit) -> String {
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
