use smart_home::device::Device;
use smart_home::device::{Socket, Thermo};
use smart_home::house::House;
use smart_home::room::Room;
use std::collections::HashMap;

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
