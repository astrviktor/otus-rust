use smart_home::device::Device;
use smart_home::device::{Socket, Thermo};
use smart_home::house::House;
use smart_home::room::Room;

fn main() {
    // Инициализация устройств
    let socket1 = Socket::new("Socket1".to_string(), "Socket1".to_string(), false, 20.0);
    let socket2 = Socket::new("Socket2".to_string(), "Socket2".to_string(), false, 50.0);

    let thermo1 = Thermo::new("Thermo1".to_string(), 22.5);
    let thermo2 = Thermo::new("Thermo2".to_string(), 24.0);

    // Инициализация комнат
    let mut kitchen = Room::new("Kitchen".to_string());

    kitchen.add_device(socket1.get_name(), Device::SocketDevice(socket1));
    kitchen.add_device(thermo1.get_name(), Device::ThermoDevice(thermo1));

    let mut bedroom = Room::new("Bedroom".to_string());

    bedroom.add_device(socket2.get_name(), Device::SocketDevice(socket2));
    bedroom.add_device(thermo2.get_name(), Device::ThermoDevice(thermo2));

    // Инициализация дома
    let mut house = House::new("House".to_string());

    let _ = house.add_room(kitchen);
    let _ = house.add_room(bedroom);

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
