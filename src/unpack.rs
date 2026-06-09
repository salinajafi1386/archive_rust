use std::fs;
use std::io;
use std::path::PathBuf;

pub fn unpack(archive: PathBuf, _password: Option<String>) -> Result<(), io::Error> {
    if !archive.exists() {
        eprintln!("Error: File not found: {:#?}", &archive);
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    if !archive.is_file() {
        eprintln!("Error: Archive path is a directory: {:#?}", archive);
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Is a directory",
        ));
    }

    match fs::metadata(&archive) {
        Ok(meta) => {
            println!("File size is: {} bytes", meta.len());
            Ok(())
        }
        Err(_e) => {
            eprintln!("Could not read metadata for: {:#?}", &archive);
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }
    }
}
