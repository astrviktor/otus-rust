use std::fs::File;
use std::io::{self, BufWriter};
use std::net::TcpListener;

use synt_examples::{transfer, BUF_SIZE};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:12345")?;

    if let Ok((stream, peer)) = listener.accept() {
        println!("Receiving file from {peer}");

        let file = File::create("./big_file_received.data")?;
        let file = BufWriter::with_capacity(BUF_SIZE * 4, file);
        let total_received = transfer(stream, file)?;
        println!("Received {total_received} bytes in total");
    }

    Ok(())
}
