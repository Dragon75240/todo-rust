/*use std::{fs, io::Error, io::prelude::Read};
use dirs;

pub fn open_json(file_name: &str) -> Result<String, Error> {
    let mut file = fs::File::open(dirs::document_dir().expect("Path")
    .join("/".to_string() + file_name))?;
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => println!("Successfully opened JSON."),
        Err(e) => println!("Failed to open JSON: {}", e),
    }

    Ok(contents)
}*/