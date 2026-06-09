use std::path::PathBuf;

pub fn pack(files: Vec<PathBuf>, password: Option<String>) {
    println!("Packing mode initiated...");
    println!("Files: {:#?}", files);
    println!("Password: {:#?}", password);
}
