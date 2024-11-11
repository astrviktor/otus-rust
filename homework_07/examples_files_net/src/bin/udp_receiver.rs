fn main() {
    let socket = std::net::UdpSocket::bind("127.0.0.1:12345").unwrap();

    let mut buf = [0; 1024];
    for _ in 0..100u32 {
        let (got_bytes, from_addr) = socket.recv_from(&mut buf).unwrap();
        assert_eq!(got_bytes, 4);
        let got_number = u32::from_be_bytes(buf[0..4].try_into().unwrap());
        println!("Received {got_bytes} from {from_addr}: {got_number}");
    }
}