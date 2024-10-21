mod house_tests {
    use smart_home::device::{Socket, Thermo, Device};
    use smart_home::house::House;
    use smart_home::room::Room;
    use std::collections::HashMap;

    #[test]
    fn test_house_create_report() {
        let thermo = Thermo {
            name: "Thermo".to_string(),
            temperature: 22.5,
        };

        let mut kitchen = Room {
            name: "Kitchen".to_string(),
            devices: HashMap::new(),
        };
        kitchen
            .devices
            .insert(thermo.get_name(), Device::ThermoDevice(thermo));

        let mut house = House {
            name: "House".to_string(),
            rooms: HashMap::new(),
        };
        house.rooms.insert(kitchen.get_name(), kitchen);

        let expected_report = "Report: \nHouse name: House\nRoom name: Kitchen\nDevices: \nDevice name: Thermo\nDevice info - Thermo name: Thermo, temperature: 22.5 \n";
        assert_eq!(house.create_report(), expected_report);
    }

    #[test]
    fn test_house_get_rooms_list() {
        let kitchen = Room {
            name: "Kitchen".to_string(),
            devices: HashMap::new(),
        };

        let bedroom = Room {
            name: "Bedroom".to_string(),
            devices: HashMap::new(),
        };

        let mut house = House {
            name: "House".to_string(),
            rooms: HashMap::new(),
        };
        house.rooms.insert(kitchen.get_name(), kitchen);
        house.rooms.insert(bedroom.get_name(), bedroom);

        assert_eq!(house.get_rooms_list().len(), 2);
        assert!(house.get_rooms_list().contains("Kitchen"));
        assert!(house.get_rooms_list().contains("Bedroom"));
    }
}
