#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

pub fn convert_temperature(value: f64, from: TemperatureUnit, to: TemperatureUnit) -> f64 {
    todo!("convert between Celsius, Fahrenheit, and Kelvin")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(left: f64, right: f64) {
        assert!((left - right).abs() < 0.0001, "{left} != {right}");
    }

    #[test]
    fn celsius_to_fahrenheit() {
        approx_eq(
            convert_temperature(100.0, TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit),
            212.0,
        );
    }

    #[test]
    fn fahrenheit_to_celsius() {
        approx_eq(
            convert_temperature(32.0, TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius),
            0.0,
        );
    }

    #[test]
    fn kelvin_to_celsius() {
        approx_eq(
            convert_temperature(273.15, TemperatureUnit::Kelvin, TemperatureUnit::Celsius),
            0.0,
        );
    }

    #[test]
    fn same_unit_is_identity() {
        approx_eq(
            convert_temperature(19.5, TemperatureUnit::Celsius, TemperatureUnit::Celsius),
            19.5,
        );
    }
}
