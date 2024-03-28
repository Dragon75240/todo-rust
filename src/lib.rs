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


pub mod todo_util {
    use rusqlite::Connection;

    pub fn create_todo(conn: &Connection, name: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let mut stmt = conn.prepare("INSERT INTO todos (name) VALUES (?1)")?;
        let _rows_affected = stmt.execute(&[name])?;

        let last_insert_row_id = conn.last_insert_rowid();

        Ok(last_insert_row_id as i32)
    }
}
