use std::collections::HashMap;
use std::collections::HashSet;

struct Socket {
    name: String,
    description: String,
    is_on: bool,
    power_consumption: f32,
}

impl Socket {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn describe(&self) -> String {
        self.description.clone()
    }

    fn _turn_on(&mut self) {
        self.is_on = true;
    }

    fn _turn_off(&mut self) {
        self.is_on = false;
    }

    fn get_state(&self) -> bool {
        self.is_on
    }

    fn get_power_consumption(&self) -> f32 {
        self.power_consumption
    }
}

struct Thermo {
    name: String,
    temperature: f32,
}

impl Thermo {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

trait Info {
    fn info(&self) -> String;
}

impl Info for Socket {
    fn info(&self) -> String {
        format!(
            "Device info - Socket name: {}, description: {}, power consumption: {}, state: {} \n",
            self.get_name(),
            self.describe(),
            self.get_power_consumption(),
            self.get_state(),
        )
    }
}

impl Info for Thermo {
    fn info(&self) -> String {
        format!(
            "Device info - Thermo name: {}, temperature: {} \n",
            self.get_name(),
            self.get_temperature(),
        )
    }
}

enum Device {
    SocketDevice(Socket),
    ThermoDevice(Thermo),
}

struct Room {
    name: String,
    devices: HashMap<String, Device>,
}

impl Room {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_devices(&self) -> &HashMap<String, Device> {
        &self.devices
    }

    fn print_room_devices(&self) {
        println!("Room name: {}", self.name);
        let mut devices = String::new();
        for (name, _) in self.devices.iter() {
            devices = [devices, name.clone()].join(" ");
        }
        println!("Room devices: {}", devices);
    }
}

struct House {
    name: String,
    rooms: HashMap<String, Room>,
}

impl House {
    fn create_report(&self) -> String {
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

    fn get_rooms_list(&self) -> HashSet<String> {
        let mut names = HashSet::new();
        for (name, _) in self.rooms.iter() {
            names.insert(name.to_string());
        }

        names
    }
}

fn main() {
    // Инициализация устройств
    let socket1 = Socket {
        name: "Socket1".to_string(),
        description: "Socket1".to_string(),
        is_on: false,
        power_consumption: 20.0,
    };

    let socket2 = Socket {
        name: "Socket2".to_string(),
        description: "Socket2".to_string(),
        is_on: false,
        power_consumption: 50.0,
    };

    let thermo1 = Thermo {
        name: "Thermo1".to_string(),
        temperature: 22.5,
    };

    let thermo2 = Thermo {
        name: "Thermo2".to_string(),
        temperature: 24.0,
    };

    // Инициализация комнат
    let mut kitchen = Room {
        name: "Kitchen".to_string(),
        devices: HashMap::new(),
    };
    kitchen
        .devices
        .insert(socket1.get_name(), Device::SocketDevice(socket1));
    kitchen
        .devices
        .insert(thermo1.get_name(), Device::ThermoDevice(thermo1));

    let mut bedroom = Room {
        name: "Bedroom".to_string(),
        devices: HashMap::new(),
    };
    bedroom
        .devices
        .insert(socket2.get_name(), Device::SocketDevice(socket2));
    bedroom
        .devices
        .insert(thermo2.get_name(), Device::ThermoDevice(thermo2));

    // Инициализация дома
    let mut house = House {
        name: "House".to_string(),
        rooms: HashMap::new(),
    };
    house.rooms.insert(kitchen.get_name(), kitchen);
    house.rooms.insert(bedroom.get_name(), bedroom);

    // Получение списка комнат
    let rooms_list = house.get_rooms_list();
    print!("Rooms: ");
    for name in rooms_list.iter() {
        print!(" {}", name);
    }
    println!("\n");

    // Получение списка комнат и печать списка устройств в них
    for (_, room) in house.rooms.iter() {
        room.print_room_devices();
    }
    println!("\n");

    // Получение отчета
    println!("{}", house.create_report());
}
