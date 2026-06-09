use std::fs;
use std::io;
use std::path::PathBuf;

pub fn pack(files: Vec<PathBuf>, _password: Option<String>) -> Result<(), io::Error> {
    for file_path in &files {
        if !file_path.exists() {
            eprintln!("Error: File not found: {:#?}", file_path);
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }

        if !file_path.is_file() {
            eprintln!("Error: Path is a directory, not a file: {:#?}", file_path);
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Is a directory",
            ));
        }

        match fs::metadata(file_path) {
            Ok(meta) => {
                println!("File size is: {} bytes", meta.len());
            }
            Err(_e) => {
                eprintln!("Could not read metadata for: {:#?}", file_path);
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Could not read metadata",
                ));
            }
        }
    }

    Ok(())
}
