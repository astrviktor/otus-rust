use std::fs::File;
use std::io::{self, BufReader};
use std::net::TcpStream;

use synt_examples::{transfer, BUF_SIZE};

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:12345").unwrap();

    let path = "./big_file.data";
    let file = BufReader::with_capacity(BUF_SIZE * 4, File::open(path)?);

    let total_sent = transfer(file, stream)?;
    println!("Sent {total_sent} bytes in total");

    Ok(())
}
