use crate::device::Device;
use std::collections::HashMap;

pub struct Room {
    pub name: String,
    pub devices: HashMap<String, Device>,
}

impl Room {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_devices(&self) -> &HashMap<String, Device> {
        &self.devices
    }

    pub fn print_room_devices(&self) {
        println!("Room name: {}", self.name);
        let mut devices = String::new();
        for (name, _) in self.devices.iter() {
            devices = [devices, name.clone()].join(" ");
        }
        println!("Room devices: {}", devices);
    }
}
