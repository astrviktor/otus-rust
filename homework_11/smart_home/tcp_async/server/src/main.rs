use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::sync::Arc;
use smart_home::device::Socket;

async fn handle_command(stream: &mut TcpStream, device: Arc<Mutex<Socket>>, command: &str) -> Result<(), std::io::Error> {
    let mut device = device.lock().await;

    match command {
        "TURN_ON" => {
            device.turn_on();
            stream.write_all(b"Socket turned on\n").await
        },
        "TURN_OFF" => {
            device.turn_off();
            stream.write_all(b"Socket turned off\n").await
        },
        "GET_STATE" => {
            let state = if device.get_state() { "on" } else { "off" };
            stream.write_all(format!("Socket is {}\n", state).as_bytes()).await
        },
        "GET_POWER" => {
            let power = device.get_power_consumption();
            stream.write_all(format!("Power consumption is {}\n", power).as_bytes()).await
        },
        _ => {
            stream.write_all(b"Invalid command\n").await
        },
    }
}

async fn handle_client(mut stream: TcpStream, device: Arc<Mutex<Socket>>) -> Result<(), std::io::Error> {
    let mut buffer = [0 as u8; 1024];

    loop {
        match stream.read(&mut buffer).await {
            Ok(size) => {
                if size == 0 {
                    println!("Client closed connection: {}", stream.peer_addr().unwrap());
                    return Ok(());
                }

                let command = std::str::from_utf8(&buffer[0..size])
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
                    .trim();

                if let Err(e) = handle_command(&mut stream, device.clone(), command).await {
                    println!("Error writing to stream: {}", e);
                    return Ok(());
                }
            },
            Err(e) => {
                println!("An error occurred, terminating connection with {}: {}", stream.peer_addr().unwrap(), e);
                return Ok(());
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Socket device
    let device = Arc::new(Mutex::new(Socket::new(
        "Socket".to_string(),
        "Используется для подключения устройств к электросети".to_string(),
        false,
        100.0,
    )));

    let listener = TcpListener::bind("0.0.0.0:8888").await?;
    println!("Server listening on port 8888");
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let device_clone = Arc::clone(&device);
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream, device_clone).await {
                        println!("Error handling client: {}", e);
                    }
                });
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

