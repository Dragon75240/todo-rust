use std::{cell::RefCell, rc::Rc};

use rusqlite::{Connection, Result};

use crate::option::Option;

fn create_todo(conn: &Connection, name: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("INSERT INTO todos (name) VALUES (?1)")?;
    let _rows_affected = stmt.execute(&[name])?;

    let last_insert_row_id = conn.last_insert_rowid();

    Ok(last_insert_row_id as i32)
}

pub struct New {
    pub conn: Rc<RefCell<Connection>>,
}

impl Option for New {
    fn exec(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() < 1 {
            return Err("Missing name of todo".to_string().into());
        }

        let todo_name: String = args.join(" ");

        let _ = create_todo(&self.conn.borrow(), &todo_name);
        Ok(())
    }
}