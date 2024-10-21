pub struct Socket {
    pub name: String,
    pub description: String,
    pub is_on: bool,
    pub power_consumption: f32,
}

impl Socket {
    pub fn new(
        &mut self,
        name: String,
        description: String,
        is_on: bool,
        power_consumption: f32,
    ) -> &mut Self {
        self.name = name;
        self.description = description;
        self.is_on = is_on;
        self.power_consumption = power_consumption;

        self
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn describe(&self) -> String {
        self.description.clone()
    }

    pub fn _turn_on(&mut self) {
        self.is_on = true;
    }

    pub fn _turn_off(&mut self) {
        self.is_on = false;
    }

    pub fn get_state(&self) -> bool {
        self.is_on
    }

    pub fn get_power_consumption(&self) -> f32 {
        self.power_consumption
    }

    pub fn info(&self) -> String {
        format!(
            "Device info - Socket name: {}, description: {}, power consumption: {}, state: {} \n",
            self.get_name(),
            self.describe(),
            self.get_power_consumption(),
            self.get_state(),
        )
    }
}
