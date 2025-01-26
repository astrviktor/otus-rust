use crate::device::Device;
use crate::room::Room;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct House {
    pub name: String,
    pub rooms: HashMap<String, Room>,
}

impl House {
    pub fn new(name: String) -> House {
        House {
            name,
            rooms: HashMap::new(),
        }
    }

    pub fn create_report(&self) -> String {
        let mut report: String = "Report: \n".to_string();
        report = [
            report,
            "House name: ".to_string(),
            self.name.clone(),
            "\n".to_string(),
        ]
        .join("");

        for (name, room) in self.rooms.iter() {
            report = [
                report,
                "Room name: ".to_string(),
                name.clone(),
                "\n".to_string(),
            ]
            .join("");
            report = [report, "Devices: ".to_string(), "\n".to_string()].join("");

            for (name, device) in room.get_devices().iter() {
                report = [report, "Device name: ".to_string(), name.clone()].join("");
                match device {
                    Device::SocketDevice(socket) => report = [report, socket.info()].join("\n"),
                    Device::ThermoDevice(thermo) => report = [report, thermo.info()].join("\n"),
                }
            }
        }

        report
    }

    pub fn get_rooms_list(&self) -> HashSet<String> {
        let mut names = HashSet::new();
        for (name, _) in self.rooms.iter() {
            names.insert(name.to_string());
        }

        names
    }

    pub fn get_room_by_name(&self, name: &str) -> Result<&Room, &str> {
        match self.rooms.get(name) {
            Some(room) => Ok(room),
            None => Err("Room not found"),
        }
    }

    pub fn add_room(&mut self, room: Room) -> Result<(), &str> {
        let name = room.name.clone();
        if let std::collections::hash_map::Entry::Vacant(e) = self.rooms.entry(name) {
            e.insert(room);
            Ok(())
        } else {
            Err("Room already exists")
        }
    }

    pub fn remove_room(&mut self, name: &str) -> Result<(), &str> {
        if self.rooms.remove(name).is_some() {
            Ok(())
        } else {
            Err("Room not found")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::room::Room;

    #[test]
    fn test_new_house() {
        let house = House::new("Test House".to_string());

        assert_eq!(house.name, "Test House");
        assert_eq!(house.rooms.len(), 0)
    }

    #[test]
    fn test_get_room_by_name_exists() {
        let house = House {
            name: "Test House".to_string(),
            rooms: HashMap::from([
                ("Bedroom".to_string(), Room::new("Bedroom".to_string())),
                ("Kitchen".to_string(), Room::new("Kitchen".to_string())),
            ]),
        };

        match house.get_room_by_name("Bedroom") {
            Ok(room) => assert_eq!(room.name, "Bedroom"),
            Err(_) => panic!("Room should exist"),
        }

        match house.get_room_by_name("Bathroom") {
            Ok(_) => panic!("Room should not exist"),
            Err(err) => assert_eq!(err, "Room not found"),
        }
    }

    #[test]
    fn test_add_and_remove_room() {
        let mut house = House {
            name: "Test House".to_string(),
            rooms: HashMap::new(),
        };

        // Add a new room
        let bedroom = Room::new("Bedroom".to_string());
        match house.add_room(bedroom) {
            Ok(()) => assert_eq!(house.get_rooms_list().len(), 1),
            Err(_) => panic!("Room should have been added"),
        }

        // Try to add a existent room
        let existing_room = Room::new("Bedroom".to_string());
        match house.add_room(existing_room) {
            Ok(_) => panic!("Room should not have been added"),
            Err(err) => assert_eq!(err, "Room already exists"),
        }

        // Remove the room
        match house.remove_room("Bedroom") {
            Ok(()) => assert_eq!(house.get_rooms_list().len(), 0),
            Err(_) => panic!("Room should have been removed"),
        }

        // Try to remove a non-existent room
        match house.remove_room("Bathroom") {
            Ok(_) => panic!("Room should not have been removed"),
            Err(err) => assert_eq!(err, "Room not found"),
        }
    }
}
