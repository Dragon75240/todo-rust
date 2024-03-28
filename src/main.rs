/*
    For a todo list we need this
    1. a database for the todos
    2. managing the todos through console
*/

struct TodoItem {
    id: i32,
    name: String,
}

pub mod option;

use dirs;
use rusqlite::{Connection, Result};
use std::{collections::HashMap, env};

use crate::option::Option;

mod options {
    pub mod new;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let db_path = dirs::document_dir().expect("Path").join("todos.sqlite");
    let conn = Connection::open(db_path)?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )",
        [],
    )?;
    
    let todos = get_todos(&conn)?;

    let mut options: HashMap<String, Box<dyn Option>> = HashMap::new();

    if args.len() > 1 {
        let command_name = args[1].clone();
        let args = &args[2..];
        
    } else {
        for todo in todos {
            println!("ID: {}, Name: {}", todo.id, todo.name);
        }
    }

    Ok(())
}

fn get_todos(conn: &Connection) -> Result<Vec<TodoItem>> {
    let mut stmt = conn.prepare("SELECT id, name FROM todos")?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    let mut todos = Vec::new();
    for todo in todo_iter {
        todos.push(todo?);
    }

    Ok(todos)
}
