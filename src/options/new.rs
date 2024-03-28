use rusqlite::{Connection, Result};
use todo::todo_util;

use crate::option::Option;

pub struct New { pub conn: Connection }
/*
impl New {
    pub fn new(connection: Connection) -> Self {
        New { conn: connection }
    }
}*/

impl Option for New {
    fn exec(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() < 1 {
            return Err("Missing name of todo".to_string().into());
        }

        let todo_name = &args[0];
        let _new_todo_id = todo_util::create_todo(&self.conn, todo_name);
        Ok(())
    }
}