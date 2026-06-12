use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug)]
struct FileEntry {
    name: String,
    size: u64,
}

#[derive(Debug)]
struct ArchiveHeader {
    file_count: u32,
    files: Vec<FileEntry>,
}

pub fn unpack(archive: PathBuf, _password: Option<String>) -> Result<(), io::Error> {
    check_archive_file(&archive)?;

    let header = read_header(&archive)?;

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
        file_count,
        files: file_list,
    })
}
