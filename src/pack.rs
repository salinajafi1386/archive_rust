use std::fs;
use std::fs::File;
use std::io;
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

    create_arch(&output_name)?;

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

fn create_arch(output_name: &PathBuf) -> Result<(), io::Error> {
    let _file = File::create(output_name)?;
    Ok(())
}
