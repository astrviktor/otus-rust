use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use smart_home::device::Socket;

fn handle_command(mut stream: &TcpStream, device: &mut Socket, command: &str) -> Result<usize, std::io::Error> {
    match command {
        "TURN_ON" => {
            device.turn_on();
            println!("Socket turned on");
            stream.write(b"Socket turned on\n")
        },
        "TURN_OFF" => {
            device.turn_off();
            println!("Socket turned off");
            stream.write(b"Socket turned off\n")
        },
        "GET_STATE" => {
            let state = if device.get_state() { "on" } else { "off" };
            println!("Socket is {}", state);
            stream.write(format!("Socket is {}\n", state).as_bytes())
        },
        "GET_POWER" => {
            let power = device.get_power_consumption();
            println!("Power consumption is {}", power);
            stream.write(format!("Power consumption is {}\n", power).as_bytes())
        },
        _ => {
            stream.write(b"Invalid command\n")
        },
    }
}

fn handle_client(mut stream: TcpStream, device: &mut Socket) {
    let mut data = [0 as u8; 100];
    while match stream.read(&mut data) {
        Ok(size) => {
            if size == 0 {
                // Клиент закрыл соединение
                println!("Client closed connection: {}", stream.peer_addr().unwrap());
                stream.shutdown(std::net::Shutdown::Both).unwrap_or_else(|e| println!("Error shutting down stream: {}", e));
                return;
            }

            let command = from_utf8(&data[0..size]).unwrap().trim();

            let result = handle_command(&stream, device, command);

            match result {
                Ok(_) => true,
                Err(e) => {
                    println!("Error writing to stream: {}", e);
                    stream.shutdown(std::net::Shutdown::Both).unwrap_or_else(|e| println!("Error shutting down stream: {}", e));
                    false
                }
            }
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    // Socket device
    let mut device = Socket::new(
        "Socket".to_string(),
        "Используется для подключения устройств к электросети".to_string(),
        false,
        100.0,
    );

    let listener = TcpListener::bind("0.0.0.0:8888").unwrap();
    println!("Server listening on port 8888");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream, &mut device);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}