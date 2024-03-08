use std::fmt::{Display, Formatter};

use rusqlite::Connection;
use time::{format_description, OffsetDateTime};

#[derive(Debug)]
pub struct TodoItem {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    /// Format: "YYYY-MM-DD"
    pub created_at: OffsetDateTime,
}

impl TodoItem {
    pub fn new(description: String) -> Self {
        let created_timestamp = OffsetDateTime::now_local().unwrap();

        Self {
            id: 0,
            description,
            completed: false,
            created_at: created_timestamp,
        }
    }
}

impl Display for TodoItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let is_completed = if self.completed { "X" } else { " " };

        let format =
            format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
        let formatted_time = self.created_at.format(&format).unwrap();

        write!(
            f,
            "[{}] ID: {}, Description: {}, Created At: {}",
            is_completed, self.id, self.description, formatted_time
        )
    }
}

pub fn create_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS TodoItem (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                completed BOOLEAN NOT NULL,
                created_at DATE NOT NULL DEFAULT CURRENT_TIMESTAMP
                )",
        (),
    )?;

    Ok(())
}

pub fn add_todo_item(conn: &Connection, todo_item: &TodoItem) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO TodoItem (description, completed) VALUES (?1, ?2)",
        (&todo_item.description, 0),
    )?;

    Ok(())
}

pub fn get_todo(conn: &Connection, id: u32) -> rusqlite::Result<TodoItem> {
    conn.query_row("SELECT * FROM TodoItem WHERE id = ?1", [id], |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            description: row.get(1)?,
            completed: row.get(2)?,
            created_at: row.get(3)?,
        })
    })
}

pub fn get_all_todo_items(conn: &Connection) -> rusqlite::Result<Vec<TodoItem>> {
    let mut list_data = conn.prepare("SELECT * FROM TodoItem")?;
    let list_iter = list_data
        .query_map([], |row| {
            Ok(TodoItem {
                id: row.get(0)?,
                description: row.get(1)?,
                completed: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?
        .map(|task| task.unwrap());
    Ok(list_iter.collect())
}

pub fn check_todo(conn: &Connection, id: u32) -> rusqlite::Result<()> {
    conn.execute("UPDATE TodoItem SET completed = 1 WHERE id = ?1", [id])?;
    Ok(())
}

pub fn uncheck_todo(conn: &Connection, id: u32) -> rusqlite::Result<()> {
    conn.execute("UPDATE TodoItem SET completed = 0 WHERE id = ?1", [id])?;
    Ok(())
}

pub fn remove_todo_item(conn: &Connection, id: u32) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM TodoItem WHERE id = ?1", [id])?;
    Ok(())
}

pub fn remove_overdue_todos(conn: &Connection) -> rusqlite::Result<()> {
    let current_date = OffsetDateTime::now_utc();
    conn.execute("DELETE FROM TodoItem WHERE created_at < ?1", [current_date])?;
    Ok(())
}

pub fn remove_all_todos(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM TodoItem", [])?;
    Ok(())
}
