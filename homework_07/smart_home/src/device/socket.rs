pub struct Socket {
    pub name: String,
    pub description: String,
    pub is_on: bool,
    pub power_consumption: f32,
}

impl Socket {
    pub fn new(name: String, description: String, is_on: bool, power_consumption: f32) -> Socket {
        Socket {
            name,
            description,
            is_on,
            power_consumption,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn describe(&self) -> String {
        self.description.clone()
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    pub fn get_state(&self) -> bool {
        self.is_on
    }

    pub fn get_power_consumption(&self) -> f32 {
        self.power_consumption
    }

    pub fn info(&self) -> String {
        format!(
            "Device info - Socket name: {}, description: {}, power consumption: {:.1}, state: {} \n",
            self.get_name(),
            self.describe(),
            self.get_power_consumption(),
            self.get_state(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Socket;

    #[test]
    fn test_new_socket() {
        let socket = Socket::new(
            "Socket".to_string(),
            "Этот сокет используется для подключения устройств к электросети".to_string(),
            false,
            100.0,
        );

        assert_eq!(socket.name, "Socket");
        assert_eq!(
            socket.description,
            "Этот сокет используется для подключения устройств к электросети"
        );
        assert!(!socket.is_on);
        assert_eq!(socket.power_consumption, 100.0);
    }

    #[test]
    fn test_turn_on_socket() {
        let mut socket = Socket::new(
            "Socket".to_string(),
            "Этот сокет используется для подключения устройств к электросети".to_string(),
            false,
            100.0,
        );

        socket.turn_on();

        assert!(socket.is_on);
    }

    #[test]
    fn test_turn_off_socket() {
        let mut socket = Socket::new(
            "Socket".to_string(),
            "Этот сокет используется для подключения устройств к электросети".to_string(),
            true,
            100.0,
        );

        socket.turn_off();

        assert!(!socket.is_on);
    }

    #[test]
    fn test_info_socket() {
        let socket = Socket::new(
            "Socket".to_string(),
            "Этот сокет используется для подключения устройств к электросети".to_string(),
            false,
            100.0,
        );

        assert_eq!(socket.info(), "Device info - Socket name: Socket, description: Этот сокет используется для подключения устройств к электросети, power consumption: 100.0, state: false \n");
    }
}
