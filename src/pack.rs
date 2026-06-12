use std::fs;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::{Read, Write};
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

pub fn pack(
    files: Vec<PathBuf>,
    _password: Option<String>,
    output_name: PathBuf,
) -> Result<(), io::Error> {
    check_files(&files)?;

    let output_name = validate_output_name(&output_name)?;

    let header = create_archive_header(&files)?;

    println!("{:#?}", header);

    let mut writer = create_arch(&output_name)?;

    write_header(&mut writer, &header)?;

    write_files_data(&files, &mut writer)?;

    Ok(())
}

fn check_files(files: &[PathBuf]) -> Result<(), io::Error> {
    for file_path in files {
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
    }

    Ok(())
}

fn create_archive_header(files: &[PathBuf]) -> Result<ArchiveHeader, io::Error> {
    let mut file_list = Vec::new();

    for file_path in files {
        let meta = match fs::metadata(file_path) {
            Ok(m) => m,
            Err(e) => return Err(e),
        };

        let size = meta.len();
        let file_name = match file_path.file_name() {
            Some(name) => name.to_string_lossy().into_owned(),
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Could not determine file name",
                ));
            }
        };

        let info = FileEntry {
            name: file_name,
            size,
        };

        file_list.push(info);
    }

    let header = ArchiveHeader {
        file_count: file_list.len() as u32,
        files: file_list,
    };

    Ok(header)
}

fn validate_output_name(output_name: &PathBuf) -> Result<PathBuf, io::Error> {
    match output_name.extension() {
        Some(ext) => {
            if ext.to_str() != Some("arch") {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Output file must use the .arch extension.",
                ));
            }
            Ok(output_name.to_path_buf())
        }
        None => {
            let mut fixed_name = output_name.to_path_buf();
            fixed_name.set_extension("arch");
            Ok(fixed_name)
        }
    }
}

fn create_arch(output_name: &PathBuf) -> Result<BufWriter<File>, io::Error> {
    let file = File::create(output_name)?;
    let writer = BufWriter::new(file);
    Ok(writer)
}

fn write_header(writer: &mut BufWriter<File>, header: &ArchiveHeader) -> Result<(), io::Error> {
    writer.write_all(&header.file_count.to_le_bytes())?;

    for file in &header.files {
        writer.write_all(&(file.name.len() as u32).to_le_bytes())?;

        writer.write_all(file.name.as_bytes())?;

        writer.write_all(&file.size.to_le_bytes())?;
    }
    writer.flush()?;

    Ok(())
}

fn write_files_data(files: &[PathBuf], writer: &mut BufWriter<File>) -> Result<(), io::Error> {
    for file in files {
        let mut data = Vec::new();

        let mut opened_file = File::open(file)?;

        opened_file.read_to_end(&mut data)?;

        writer.write_all(&data)?;
    }
    writer.flush()?;

    Ok(())
}
