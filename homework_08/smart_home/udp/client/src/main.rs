use std::net::UdpSocket;
use std::{thread, time::Duration};
use smart_home::device::Thermo;
use rand::Rng;

fn main() {
    // Адрес и порт сервера
    let server_addr = "127.0.0.1:9999";

    // Создание UDP-сокета для отправки данных
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Не удалось создать сокет");

    // Данные термометров
    let mut thermo1 = Thermo::new("Kitchen".to_string(), 22.5);
    let mut thermo2 = Thermo::new("Bedroom".to_string(), 24.0);

    // Отправка данных на сервер с случайными показателями температуры
    for _ in 0..10 {
        thread::sleep(Duration::from_secs(3));

        // Термометр Kitchen
        let temperature1 = rand::thread_rng().gen_range(28.0..30.0);
        thermo1.set_temperature(temperature1);

        socket.send_to(thermo1.data_to_string().as_bytes(), server_addr).expect("Не удалось отправить данные");
        println!("Отправлены данные: {}", thermo1.data_to_string());

        // Термометр Bedroom
        let temperature2 = rand::thread_rng().gen_range(24.0..27.0);
        thermo2.set_temperature(temperature2);

        socket.send_to(thermo2.data_to_string().as_bytes(), server_addr).expect("Не удалось отправить данные");
        println!("Отправлены данные: {}", thermo2.data_to_string());
    }

    println!("Done");
}
