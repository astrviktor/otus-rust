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
            "Device info - Thermo name: {}, temperature: {} \n",
            self.get_name(),
            self.get_temperature(),
        )
    }
}
