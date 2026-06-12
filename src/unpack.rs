use crate::crypto::xor_stream;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::PathBuf;

#[derive(Debug)]
struct FileEntry {
    name: String,
    size: u64,
}

#[derive(Debug)]
struct ArchiveHeader {
    encrypted: bool,
    file_count: u32,
    files: Vec<FileEntry>,
}
pub fn unpack(archive: PathBuf, password: Option<String>) -> Result<(), io::Error> {
    check_archive_file(&archive)?;

    let header = read_header(&archive)?;

    check_password_requirement(&header, &password)?;

    if header.encrypted && password.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Archive is encrypted. Please provide a password.",
        ));
    }

    read_files(&archive, &header, password)?;
    println!("{:#?}", header);

    Ok(())
}

fn check_archive_file(archive: &PathBuf) -> Result<(), io::Error> {
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

    Ok(())
}

fn read_header(archive: &PathBuf) -> Result<ArchiveHeader, io::Error> {
    let mut file_list = Vec::new();

    let mut file = File::open(archive)?;

    let mut magic = [0u8; 4];
    file.read_exact(&mut magic)?;

    if &magic != b"ARCH" {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid archive format",
        ));
    }

    let mut encrypted_buffer = [0u8; 1];
    file.read_exact(&mut encrypted_buffer)?;

    let encrypted = encrypted_buffer[0] != 0;

    let mut buffer = [0u8; 4];

    file.read_exact(&mut buffer)?;

    let file_count = u32::from_le_bytes(buffer);

    for _ in 0..file_count {
        let mut name_len_buffer = [0u8; 4];
        file.read_exact(&mut name_len_buffer)?;

        let name_len = u32::from_le_bytes(name_len_buffer);

        let mut name_buffer = vec![0u8; name_len as usize];
        file.read_exact(&mut name_buffer)?;

        let name = String::from_utf8(name_buffer).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 in file name")
        })?;

        let mut file_size_buffer = [0u8; 8];
        file.read_exact(&mut file_size_buffer)?;

        let file_size = u64::from_le_bytes(file_size_buffer);

        file_list.push(FileEntry {
            name,
            size: file_size,
        });
    }

    Ok(ArchiveHeader {
        encrypted,
        file_count,
        files: file_list,
    })
}
fn read_files(
    archive: &PathBuf,
    header: &ArchiveHeader,
    password: Option<String>,
) -> Result<(), io::Error> {
    fs::create_dir_all("output")?;

    let mut file = File::open(archive)?;
    let mut reader = BufReader::new(&mut file);

    let mut encrypted_buffer = [0u8; 1];
    reader.read_exact(&mut encrypted_buffer)?;

    let mut count_buffer = [0u8; 4];
    reader.read_exact(&mut count_buffer)?;

    let file_count = u32::from_le_bytes(count_buffer);

    for _ in 0..file_count {
        let mut name_len_buffer = [0u8; 4];
        reader.read_exact(&mut name_len_buffer)?;
        let name_len = u32::from_le_bytes(name_len_buffer);

        let mut name_buffer = vec![0u8; name_len as usize];
        reader.read_exact(&mut name_buffer)?;
        let mut size_buffer = [0u8; 8];
        reader.read_exact(&mut size_buffer)?;
    }

    for entry in &header.files {
        let output_path = PathBuf::from("output").join(&entry.name);

        let mut output_file = File::create(output_path)?;
        let mut limited_reader = reader.by_ref().take(entry.size);
        if let Some(pass) = &password {
            xor_stream(limited_reader, output_file, pass)?;
        } else {
            std::io::copy(&mut limited_reader, &mut output_file)?;
        }
    }

    Ok(())
}

fn check_password_requirement(
    header: &ArchiveHeader,
    password: &Option<String>,
) -> Result<(), io::Error> {
    if header.encrypted && password.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "This archive is encrypted. Please provide a password.",
        ));
    }

    Ok(())
}
