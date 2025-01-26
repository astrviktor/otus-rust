use actix_web::{web, App, HttpResponse, HttpServer};
use serde_json::json;
use serde::Serialize;
use smart_home::{
    device::{Device, Socket, Thermo},
    house::House,
    room::Room,
};

#[derive(Serialize)]
struct DeviceResponse {
    name: String,
    description: Option<String>,
    is_on: Option<bool>,
    power_consumption: Option<f32>,
    temperature: Option<f32>,
}

fn devices_in_room(room: &Room) -> Vec<DeviceResponse> {
    room.devices.iter().map(|(_, device)| match device {
        Device::SocketDevice(socket) => DeviceResponse {
            name: socket.get_name(),
            description: Some(socket.describe()),
            is_on: Some(socket.get_state()),
            power_consumption: Some(socket.get_power_consumption()),
            temperature: None,
        },
        Device::ThermoDevice(thermo) => DeviceResponse {
            name: thermo.get_name(),
            description: None,
            is_on: None,
            power_consumption: None,
            temperature: Some(thermo.get_temperature()),
        },
    }).collect()
}

async fn get_devices_in_room(
    house: web::Data<House>,
    path: web::Path<String>,
) -> HttpResponse {
    let room_name = path.into_inner();
    match house.get_room_by_name(&room_name) {
        Ok(room) => {
            let devices = devices_in_room(room);
            HttpResponse::Ok().json(devices)
        }
        Err(_) => HttpResponse::NotFound().body("Room not found"),
    }
}

async fn get_rooms_in_house(
    house: web::Data<House>,
    path: web::Path<String>,
) -> HttpResponse {
    let house_name = path.into_inner();
    if house.name != house_name {
        return HttpResponse::NotFound().body("House not found");
    }

    let rooms: Vec<&Room> = house.rooms.values().collect();
    let room_responses: Vec<_> = rooms.iter().map(|room| json!({
        "name": room.get_name(),
        "devices": devices_in_room(room),
    })).collect();

    HttpResponse::Ok().json(room_responses)
}

async fn get_house_report(
    house: web::Data<House>,
    path: web::Path<String>,
) -> HttpResponse {
    let house_name = path.into_inner();
    if house.name != house_name {
        return HttpResponse::NotFound().body("House not found");
    }

    let report = house.create_report();
    HttpResponse::Ok().json(report)
}

fn create_mock_house() -> House {
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

    house
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Создание mock-данных
    let house = create_mock_house();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(house.clone()))
            .route("/house/{houseName}/rooms", web::get().to(get_rooms_in_house))
            .route("/house/{houseName}/report", web::get().to(get_house_report))
            .route("/room/{roomName}/devices", web::get().to(get_devices_in_room))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
