/*
    For a todo list we need this
    1. a database for the todos
    2. managing the todos through console
*/

use rusqlite::{Connection, Result};

struct TodoItem {
    name: String
}

fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}
