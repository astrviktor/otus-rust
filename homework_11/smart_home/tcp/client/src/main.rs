use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;

fn send_command_and_print_response(mut stream: &TcpStream, msg: &[u8]) {
    println!("Send: {}", std::str::from_utf8(msg).unwrap());

    stream.write(msg).unwrap();
    let mut data = [0 as u8; 100];
    match stream.read(&mut data) {
        Ok(_) => {
            let text = from_utf8(&data).unwrap();
            println!("Received: {}", text);
        },
        Err(e) => {
            println!("Failed to receive data: {}", e);
        }
    }
}

fn main() {
    match TcpStream::connect("localhost:8888") {
        Ok(stream) => {
            println!("Successfully connected to server in port 8888");

            // Включение розетки
            send_command_and_print_response(&stream, b"TURN_ON");

            // Получение состояния
            send_command_and_print_response(&stream, b"GET_STATE");

            // Получение потребляемой мощности
            send_command_and_print_response(&stream, b"GET_POWER");

            // Выключение розетки
            send_command_and_print_response(&stream, b"TURN_OFF");

            // Получение состояния
            send_command_and_print_response(&stream, b"GET_STATE");

            // Ошибочная команда
            send_command_and_print_response(&stream, b"GET_NAME");

            // Закрытие соединения
            stream.shutdown(std::net::Shutdown::Both).unwrap();
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Done");
}