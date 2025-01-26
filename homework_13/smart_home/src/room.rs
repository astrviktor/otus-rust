use crate::device::Device;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    pub name: String,
    pub devices: HashMap<String, Device>,
}

impl Room {
    pub fn new(name: String) -> Room {
        Room {
            name,
            devices: HashMap::new(),
        }
    }

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

    pub fn get_device_by_name(&self, name: &str) -> Option<&Device> {
        self.devices.get(name)
    }

    pub fn add_device(&mut self, name: String, device: Device) {
        self.devices.insert(name, device);
    }

    pub fn remove_device(&mut self, name: &str) -> Option<Device> {
        self.devices.remove(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::Thermo;
    use std::collections::HashMap;

    #[test]
    fn test_new_room() {
        let room = Room::new("Kitchen".to_string());

        assert_eq!(room.name, "Kitchen");
        assert_eq!(room.devices.len(), 0)
    }

    #[test]
    fn test_get_device_by_name() {
        let thermo = Thermo {
            name: "Thermo".to_string(),
            temperature: 22.5,
        };

        let room = Room {
            name: "Kitchen".to_string(),
            devices: HashMap::from([("Thermo".to_string(), Device::ThermoDevice(thermo))]),
        };

        let device_found = room.get_device_by_name("Thermo");
        assert!(device_found.is_some());

        let device_not_found = room.get_device_by_name("Socket");
        assert!(device_not_found.is_none());
    }

    #[test]
    fn test_add_and_remove_device() {
        let mut room = Room {
            name: "Kitchen".to_string(),
            devices: HashMap::new(),
        };

        let thermo = Thermo {
            name: "Thermo".to_string(),
            temperature: 22.5,
        };

        room.add_device("Thermo".to_string(), Device::ThermoDevice(thermo));
        assert!(room.get_device_by_name("Thermo").is_some());

        let removed_device = room.remove_device("Thermo");
        assert!(removed_device.is_some());
        assert!(room.get_device_by_name("Thermo").is_none());
    }
}
