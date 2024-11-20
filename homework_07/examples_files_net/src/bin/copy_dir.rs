use std::path::Path;
use std::{fs, io};

fn main() {
    let copied = copy_dir_all("./target", "./target2").unwrap();
    println!("Copied {} files", copied);
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<usize> {
    // Create dst directory if it doesn't exist
    fs::create_dir_all(&dst)?;

    let mut count = 0;
    let dir_iter = fs::read_dir(src.as_ref())?;
    for entry in dir_iter {
        match copy_entry(entry, &dst) {
            Ok(c) => count += c,
            Err(e) => eprintln!("Failed to copy entry: {}", e),
        }
    }

    Ok(count)
}

fn copy_entry(entry: io::Result<fs::DirEntry>, dst: impl AsRef<Path>) -> io::Result<usize> {
    let entry = entry?;
    let entry_type = entry.file_type()?;

    let copied = if entry_type.is_dir() {
        copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?
    } else {
        fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        1
    };

    Ok(copied)
}
