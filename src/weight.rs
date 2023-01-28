use crate::unit::Unit;

pub(crate) fn convert(unit: &Unit) -> String {
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
