use std::path::PathBuf;

pub fn unpack(archive: PathBuf, password: Option<String>) {
    println!("Unpacking mode initiated...");
    println!("Archive: {:#?}", archive);
    println!("Password: {:#?}", password);
}
