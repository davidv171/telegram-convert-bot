use crate::conversion::unit::Unit;

pub(super) fn convert(unit: &Unit) -> String {
    let mut result = format!("{} {} is\n", unit.value, unit.base);
    match unit.base.as_str() {
        "kg" => {
            result.push_str(&format!("{:.3} lb\n", unit.value * 2.20462));
            result.push_str(&format!("{:.3} oz\n", unit.value * 35.274));
            result.push_str(&format!("{:.3} st\n", unit.value * 0.157473));
        }
        "g" => {
            result.push_str(&format!("{:.3} lb\n", unit.value * 0.00220462));
            result.push_str(&format!("{:.3} oz\n", unit.value * 0.035274));
            result.push_str(&format!("{:.3} st\n", unit.value * 0.000157473));
        }
        "lb" | "lbs" => {
            result.push_str(&format!("{:.3} kg\n", unit.value * 0.453592));
            result.push_str(&format!("{:.3} g\n", unit.value * 453.592));
            result.push_str(&format!("{:.3} oz\n", unit.value * 16.0));
            result.push_str(&format!("{:.3} st\n", unit.value * 0.0714286));
        }
        "oz" => {
            result.push_str(&format!("{:.3} kg\n", unit.value * 0.0283495));
            result.push_str(&format!("{:.3} g\n", unit.value * 28.3495));
            result.push_str(&format!("{:.3} lb\n", unit.value * 0.0625));
            result.push_str(&format!("{:.3} st\n", unit.value * 0.00446429));
        }
        "st" => {
            result.push_str(&format!("{:.3} kg\n", unit.value * 6.35029));
            result.push_str(&format!("{:.3} g\n", unit.value * 6350.29));
            result.push_str(&format!("{:.3} lb\n", unit.value * 14.0));
            result.push_str(&format!("{:.3} oz\n", unit.value * 224.0));
        }
        _ => {}
    }
    result
}
