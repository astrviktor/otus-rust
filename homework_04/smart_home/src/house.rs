use crate::device::Device;
use crate::room::Room;
use std::collections::{HashMap, HashSet};

pub struct House {
    pub name: String,
    pub rooms: HashMap<String, Room>,
}

impl House {
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
}
