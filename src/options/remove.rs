use rusqlite::{Connection, Result};
use std::rc::Rc;
use std::cell::RefCell;

use crate::option::Option;

fn remove_todo(conn: &Connection, name: &str) -> Result<usize, Box<dyn std::error::Error>> {
    // Prepare the SQL statement to delete a to-do item by its name
    let mut stmt = conn.prepare("DELETE FROM todos WHERE name = ?1")?;

    // Execute the statement with the name as a parameter
    let rows_affected = stmt.execute(&[name])?;

    Ok(rows_affected)
}

pub struct Remove {
    pub conn: Rc<RefCell<Connection>>,
}

impl Option for Remove {
    fn exec(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() < 1 {
            return Err("Missing name of todo".to_string().into());
        }

        let todo_name = &args[0];

        let _ = remove_todo(&self.conn.borrow(), &todo_name);
        Ok(())
    }
}
