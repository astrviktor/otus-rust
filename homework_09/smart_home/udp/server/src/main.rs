use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use smart_home::device::Thermo;


fn main() {
    // Адрес и порт для прослушивания UDP-сокета
    let addr = "127.0.0.1:9999";

    // Создание UDP-сокета для прослушивания
    let socket = UdpSocket::bind(addr).expect("Не удалось привязать сокет");
    socket.set_read_timeout(Some(Duration::from_secs(30))).expect("Не удалось установить таймаут");
    println!("UDP сервер запущен на {}", addr);

    // Хранение данных термометров в HashMap (безопасно для доступа из разных потоков)
    let thermos: Arc<Mutex<HashMap<String, Thermo>>> = Arc::new(Mutex::new(HashMap::new()));

    // Клонируем арк для использования в отдельном потоке
    let thermos_clone = thermos.clone();

    // Запуск цикла получения данных в отдельном потоке
    thread::spawn(move || {
        let mut buf = [0; 1024]; // Буфер для приема данных

        loop {
            // Получение датаграммы
            match socket.recv_from(&mut buf) {
                Ok((len, addr)) => {
                    // Преобразование буфера в строку
                    let data = String::from_utf8_lossy(&buf[..len]).trim().to_string();

                    // Парсинг данных с использованием метода string_to_data из Thermo
                    let mut thermo = Thermo::new(String::from(""), 0.0);
                    if let Err(e) = thermo.string_to_data(&data) {
                        eprintln!("Неверный формат данных от {}: {}", addr, e);
                        continue;
                    }
                    println!("Получены данные от {}: {}", addr, thermo.info());

                    // Обновление или добавление термометра в HashMap
                    let mut thermos_map = thermos_clone.lock().unwrap();
                    thermos_map.insert(thermo.get_name(), thermo);
                },
                Err(e) => {
                    eprintln!("Ошибка при приеме данных: {}", e);
                }
            }
        }
    });

    // Периодический вывод информации о термометрах
    loop {
        thread::sleep(Duration::from_secs(10));

        println!("Информация о термометрах:");
        let thermos_map = thermos.lock().unwrap();
        for (_name, thermo) in &*thermos_map {
            print!("{}", thermo.info());
        }
        println!();
    }
}

