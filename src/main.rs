/*
    For a todo list we need this
    1. a database for the todos
    2. managing the todos through console
*/

struct TodoItem {
    id: i32,
    name: String,
    status: bool,
}

pub mod option;

use dirs;
use rusqlite::{Connection, Result};
use std::{collections::HashMap, env};
use std::rc::Rc;
use std::cell::RefCell;

use crate::option::Option;

mod options {
    pub mod new;
    pub mod remove;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Database Stuff //
    let db_path = dirs::document_dir().expect("Path").join("todos.sqlite");
    let conn = Rc::new(RefCell::new(Connection::open(db_path)?));

    conn.borrow_mut().execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            status INTEGER NOT NULL
        )",
        [],
    )?;

    let todos = get_todos(&conn.borrow_mut())?;

    let mut options: HashMap<String, Box<dyn Option>> = HashMap::new();
    options.insert("new".to_string(), Box::new(options::new::New {conn: Rc::clone(&conn)}) as Box<dyn option::Option>);

    options.insert("remove".to_string(), Box::new(options::remove::Remove {conn: Rc::clone(&conn)}) as Box<dyn option::Option>);
    
    //HashMap<String, Box<dyn Option>>

    if args.len() > 1 {
        let option_name = args[1].clone();
        let args = &args[2..];

        match options.get(&option_name) {
            Some(option) => {
                match option.exec(args) {
                    Ok(_) => (),
                    Err(e) => println!("Error: {}", e),
                }
            },
            None => println!("Unknown command"),
        }
    } else {
        for todo in todos { 
            println!("{}. {}", todo.id, todo.name);
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
            status: row.get(2)?
        })
    })?;

    let mut todos = Vec::new();
    for todo in todo_iter {
        todos.push(todo?);
    }

    Ok(todos)
}
