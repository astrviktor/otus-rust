use std::io::{self, Read, Write};

pub const MB: usize = 1024 * 1024;
pub const BUF_SIZE: usize = 4 * MB;

pub fn transfer(mut from: impl Read, mut to: impl Write) -> io::Result<usize> {
    let mut buffer = vec![0u8; BUF_SIZE];

    let mut total_sent = 0;

    while let Ok(red) = from.read(&mut buffer) {
        if red == 0 {
            println!("Reading finished");
            break;
        }

        to.write_all(&buffer[..red])?;
        total_sent += red;
        println!("Trasfered {total_sent} bytes");
    }

    Ok(total_sent)
}

