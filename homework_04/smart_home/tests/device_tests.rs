mod socket_tests {
    use smart_home::device::Socket;

    #[test]
    fn test_socket_info() {
        let socket = Socket {
            name: "Socket1".to_string(),
            description: "Socket1 description".to_string(),
            is_on: false,
            power_consumption: 20.0,
        };

        let expected_info = "Device info - Socket name: Socket1, description: Socket1 description, power consumption: 20, state: false \n";
        assert_eq!(socket.info(), expected_info);
    }
}

mod thermo_tests {
    use smart_home::device::Thermo;

    #[test]
    fn test_thermo_info() {
        let thermo = Thermo {
            name: "Thermo1".to_string(),
            temperature: 22.5,
        };

        let expected_info = "Device info - Thermo name: Thermo1, temperature: 22.5 \n";
        assert_eq!(thermo.info(), expected_info);
    }
}
