use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use std::collections::HashMap;
use tokio::time::Duration;
use std::sync::Arc;
use smart_home::device::Thermo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Адрес и порт для прослушивания UDP-сокета
    let addr = "127.0.0.1:9999";

    // Создание UDP-сокета для прослушивания
    let socket = UdpSocket::bind(addr).await?;
    println!("UDP сервер запущен на {}", addr);

    // Хранение данных термометров в HashMap
    let thermos: Arc<Mutex<HashMap<String, Thermo>>> = Arc::new(Mutex::new(HashMap::new()));

    // Клонируем арк для использования в отдельной задаче
    let thermos_clone = thermos.clone();

    // Запуск цикла получения данных в отдельной задаче
    tokio::spawn(async move {
        let mut buf = [0; 1024]; // Буфер для приема данных

        loop {
            match socket.recv_from(&mut buf).await {
                Ok((len, addr)) => {
                    if len == 0 {
                        eprintln!("Получены пустые данные от {}", addr);
                        continue;
                    }

                    let data = String::from_utf8_lossy(&buf[..len]).trim().to_string();

                    let mut thermo = Thermo::new(String::from(""), 0.0);
                    if let Err(e) = thermo.string_to_data(&data) {
                        eprintln!("Неверный формат данных от {}: {}", addr, e);
                        continue;
                    }
                    println!("Получены данные от {}: {}", addr, thermo.info());

                    let mut thermos_map = thermos_clone.lock().await;
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
        tokio::time::sleep(Duration::from_secs(10)).await;

        println!("Информация о термометрах:");
        let thermos_map = thermos.lock().await;
        for (_name, thermo) in &*thermos_map {
            print!("{}", thermo.info());
        }
        println!();
    }
}