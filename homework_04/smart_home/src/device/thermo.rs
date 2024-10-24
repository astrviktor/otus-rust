pub struct Thermo {
    pub name: String,
    pub temperature: f32,
}

impl Thermo {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature
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
    use super::Thermo;

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
