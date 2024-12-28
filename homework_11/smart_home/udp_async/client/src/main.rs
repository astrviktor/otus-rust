use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration};
use smart_home::device::Thermo;
use rand::Rng;

#[tokio::main]
async fn main() {
    // Server address and port
    let server_addr = "127.0.0.1:9999";

    // Create a UDP socket for sending data
    let socket = UdpSocket::bind("0.0.0.0:0").await.expect("Failed to create socket");

    // Initialize thermostats with initial temperatures
    let mut thermo1 = Thermo::new("Kitchen".to_string(), 22.5);
    let mut thermo2 = Thermo::new("Bedroom".to_string(), 24.0);

    // Send temperature data to the server with random values
    for _ in 0..10 {
        sleep(Duration::from_secs(3)).await;

        // Kitchen thermostat
        let temperature1 = rand::thread_rng().gen_range(28.0..30.0);
        thermo1.set_temperature(temperature1);

        socket.send_to(thermo1.data_to_string().as_bytes(), server_addr)
            .await.expect("Не удалось отправить данные");
        println!("Отправлены данные: {}", thermo1.data_to_string());

        // Bedroom thermostat
        let temperature2 = rand::thread_rng().gen_range(24.0..27.0);
        thermo2.set_temperature(temperature2);

        socket.send_to(thermo2.data_to_string().as_bytes(), server_addr)
            .await.expect("Не удалось отправить данные");
        println!("Отправлены данные: {}", thermo2.data_to_string());
    }

    println!("Done");
}