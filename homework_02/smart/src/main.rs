struct SmartPlug {
    description: String,
    is_on: bool,
    power_consumption: f32,
}

impl SmartPlug {
    fn describe(&self) -> String {
        self.description.clone()
    }

    fn turn_on(&mut self) {
        self.is_on = true;
    }

    fn turn_off(&mut self) {
        self.is_on = false;
    }

    fn get_power_consumption(&self) -> f32 {
        self.power_consumption
    }
}

struct SmartThermometer {
    temperature: f32,
}

impl SmartThermometer {
    fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

fn main() {
    let mut sp = SmartPlug {
        description: String::from("test"),
        is_on: false,
        power_consumption: 1000.0,
    };

    sp.turn_on();
    sp.turn_off();

    println!(
        "SmartPlug description: {},  power consumption: {}",
        sp.describe(),
        sp.get_power_consumption()
    );

    let st = SmartThermometer { temperature: 25.0 };

    println!("SmartThermometer temperature: {}", st.get_temperature());
}
