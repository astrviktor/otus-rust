fn main() {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();

    for i in 0..10u32 {
        socket.send_to(&i.to_be_bytes(), "127.0.0.1:12345").unwrap();
        println!("Sent {}", i);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}