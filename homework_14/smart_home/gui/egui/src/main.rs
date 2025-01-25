use eframe::egui;
use std::net::TcpStream;
use std::io::{Read, Write};
use smart_home::Socket;

struct App {
    socket: Socket,
    stream: Option<TcpStream>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            socket: Socket::new("My Smart Socket".to_string(), "A smart power socket".to_string(), false, 50.0),
            stream: None,
        }
    }
}

impl App {
    fn connect_to_server(&mut self) -> Result<(), std::io::Error> {
        let stream = TcpStream::connect("localhost:8888")?;
        self.stream = Some(stream);
        Ok(())
    }

    fn send_command(&mut self, command: &str) -> Result<String, std::io::Error> {
        if let Some(ref mut stream) = self.stream {
            stream.write_all(command.as_bytes())?;
            stream.flush()?;

            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(size) => Ok(String::from_utf8_lossy(&buffer[0..size]).trim().to_string()),
                Err(e) => Err(e),
            }
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Not connected to server"))
        }
    }

    fn update_socket_info(&mut self) -> Result<(), std::io::Error> {
        if let Ok(state_response) = self.send_command("GET_STATE") {
            match state_response.as_str() {
                "Socket is on" => self.socket.turn_on(),
                _ => self.socket.turn_off(),
            }
        }

        if let Ok(power_response) = self.send_command("GET_POWER") {
            if let Some(power) = power_response.strip_prefix("Power consumption is ") {
                if let Ok(power_consumption) = power.trim().parse::<f32>() {
                    self.socket.power_consumption = power_consumption;
                }
            }
        }

        Ok(())
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Smart Socket Control");

            if ui.button("Connect").clicked() {
                match self.connect_to_server() {
                    Ok(_) => println!("Successfully connected to server"),
                    Err(e) => println!("Failed to connect: {}", e),
                }
                match self.update_socket_info() {
                    Ok(_) => println!("Successfully update socket info"),
                    Err(e) => println!("Failed to update socket info: {}", e),
                }
            }

            ui.label(format!("Name: {}", self.socket.name));
            ui.label(format!("Description: {}", self.socket.description));
            ui.label(format!("Power Consumption: {:.1} W", self.socket.power_consumption));
            ui.label(format!("State: {}", if self.socket.get_state() { "on" } else { "off" }));

            if ui.button("Turn On").clicked() {
                if let Ok(response) = self.send_command("TURN_ON") {
                    println!("{}", response);
                    if let Err(e) = self.update_socket_info() {
                        println!("Failed to update socket info: {}", e);
                    }
                }
            }

            if ui.button("Turn Off").clicked() {
                if let Ok(response) = self.send_command("TURN_OFF") {
                    println!("{}", response);
                    if let Err(e) = self.update_socket_info() {
                        println!("Failed to update socket info: {}", e);
                    }
                }
            }
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 300.0)),
        ..Default::default()
    };

    eframe::run_native("Smart Socket Control", options, Box::new(|_cc| Box::new(App::default())))?;

    Ok(())
}
