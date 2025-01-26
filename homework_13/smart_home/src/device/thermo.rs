use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ThermoError {
    #[error("Invalid input format")]
    InvalidInputFormat,
    #[error("Failed to parse temperature: {0}")]
    ParseTemperatureFailure(std::num::ParseFloatError),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Thermo {
    pub name: String,
    pub temperature: f32,
}

impl Thermo {
    pub fn new(name: String, temperature: f32) -> Thermo {
        Thermo { name, temperature }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }

    pub fn set_temperature(&mut self, new_temperature: f32) {
        self.temperature = new_temperature;
    }

    // Method to convert data to string
    pub fn data_to_string(&self) -> String {
        format!("{} {:.1}", self.name, self.temperature)
    }

    // Method to parse string and set data
    pub fn string_to_data(&mut self, input: &str) -> Result<(), ThermoError> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(ThermoError::InvalidInputFormat);
        }

        match (parts[0], parts[1].parse::<f32>()) {
            (name, Ok(temperature)) => {
                self.name = name.to_string();
                self.temperature = temperature;
                Ok(())
            }
            (_, Err(e)) => Err(ThermoError::ParseTemperatureFailure(e)),
        }
    }

    pub fn info(&self) -> String {
        format!(
            "Device info - Thermo name: {}, temperature: {:.1} \n",
            self.get_name(),
            self.get_temperature(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Thermo, ThermoError};
    use rand::Rng;

    #[test]
    fn test_new_thermo() {
        let thermo = Thermo::new("Thermometer".to_string(), 30.5);

        assert_eq!(thermo.name, "Thermometer");
        assert_eq!(thermo.temperature, 30.5);
    }

    #[test]
    fn test_get_name() {
        let thermo = Thermo {
            name: String::from("Thermometer"),
            temperature: 25.0,
        };
        assert_eq!(thermo.get_name(), "Thermometer");
    }

    #[test]
    fn test_get_temperature() {
        let thermo = Thermo {
            name: String::from("Thermometer"),
            temperature: 30.5,
        };
        assert_eq!(thermo.get_temperature(), 30.5);
    }

    #[test]
    fn test_set_temperature() {
        let mut thermo = Thermo::new("Thermometer".to_string(), 30.5);
        let new_temperature = rand::thread_rng().gen_range(0.0..100.0);
        thermo.set_temperature(new_temperature);

        assert_eq!(thermo.temperature, new_temperature);
        assert_eq!(
            thermo.info(),
            format!(
                "Device info - Thermo name: Thermometer, temperature: {:.1} \n",
                new_temperature
            )
        );
    }

    #[test]
    fn test_data_to_string() {
        let thermo = Thermo::new("Thermometer".to_string(), 30.5);
        assert_eq!(thermo.data_to_string(), "Thermometer 30.5");
    }

    #[test]
    fn test_string_to_data_success() {
        let mut thermo = Thermo::new(String::from(""), 0.0);
        let input = "Thermometer 25.3";
        assert!(thermo.string_to_data(input).is_ok());
        assert_eq!(thermo.name, "Thermometer");
        assert_eq!(thermo.temperature, 25.3);
    }

    #[test]
    fn test_string_to_data_invalid_format() {
        let mut thermo = Thermo::new(String::from(""), 0.0);
        let input = "InvalidInput";
        match thermo.string_to_data(input) {
            Err(ThermoError::InvalidInputFormat) => (),
            _ => panic!("Expected InvalidInputFormat error"),
        }
    }

    #[test]
    fn test_string_to_data_parse_failure() {
        let mut thermo = Thermo::new(String::from(""), 0.0);
        let input = "Thermometer invalid_temperature";
        match thermo.string_to_data(input) {
            Err(ThermoError::ParseTemperatureFailure(_)) => (),
            _ => panic!("Expected ParseTemperatureFailure error"),
        }
    }

    #[test]
    fn test_info() {
        let thermo = Thermo {
            name: String::from("Thermometer"),
            temperature: 20.0,
        };
        assert_eq!(
            thermo.info(),
            "Device info - Thermo name: Thermometer, temperature: 20.0 \n"
        );
    }
}
