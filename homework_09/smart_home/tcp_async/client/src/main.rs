use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
use std::error::Error;

async fn send_command_and_print_response(stream: &mut BufReader<TcpStream>, msg: &[u8]) -> Result<(), Box<dyn Error>> {
    println!("Send: {}", String::from_utf8_lossy(msg));

    // Write the message to the stream
    stream.get_mut().write_all(msg).await?;

    // Read the response from the server
    let mut buffer = Vec::new();
    // Assuming responses are newline-terminated
    match stream.read_until(b'\n', &mut buffer).await {
        Ok(_) => {
            if !buffer.is_empty() {
                let text = String::from_utf8_lossy(&buffer);
                println!("Received: {}", text.trim_end()); // Trim to remove any trailing newlines
            } else {
                println!("No data received from the server.");
            }
        },
        Err(e) => {
            println!("Failed to read data: {}", e);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match TcpStream::connect("localhost:8888").await {
        Ok(stream) => {
            // Wrap the TcpStream in a BufReader
            let mut reader = BufReader::new(stream);

            println!("Successfully connected to server in port 8888");

            // Включение розетки
            send_command_and_print_response(&mut reader, b"TURN_ON\n").await?;

            // Получение состояния
            send_command_and_print_response(&mut reader, b"GET_STATE\n").await?;

            // Получение потребляемой мощности
            send_command_and_print_response(&mut reader, b"GET_POWER\n").await?;

            // Выключение розетки
            send_command_and_print_response(&mut reader, b"TURN_OFF\n").await?;

            // Получение состояния
            send_command_and_print_response(&mut reader, b"GET_STATE\n").await?;

            // Ошибочная команда
            send_command_and_print_response(&mut reader, b"GET_NAME\n").await?;

            // Закрытие соединения
            reader.get_mut().shutdown().await?;
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }

    println!("Done");
    Ok(())
}
