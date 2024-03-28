use rusqlite::{Connection, Result};

use crate::option::Option;

pub struct New {
    conn: Connection,
}

pub fn create_todo(conn: &Connection, name: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("INSERT INTO todos (name) VALUES (?1)")?;
    let _rows_affected = stmt.execute(&[name])?;

    let last_insert_row_id = conn.last_insert_rowid();

    Ok(last_insert_row_id as i32)
}

impl Option for New {
    fn exec(&self, args: &[String]) -> Result<(), String> {
        if args.len() < 1 {
            return Err("Missing name of todo".to_string());
        }

        let todo_name = &args[0];
        let _new_todo_id = create_todo(&self.conn, todo_name);

        Ok(())
    }
}