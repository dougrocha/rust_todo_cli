pub mod todo_item;

use std::io::Write;

use clap::{Parser, Subcommand};
use rusqlite::Connection;

use todo_item::{
    add_todo_item, check_todo, get_all_todo_items, get_todo, remove_all_todos,
    remove_overdue_todos, remove_todo_item, uncheck_todo, TodoItem,
};

#[derive(Debug, Parser)]
#[command(name = "todo app")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Parser)]
enum Commands {
    /// Add a to-do item
    #[command(arg_required_else_help = true)]
    Add {
        /// The to-do item to add
        task: String,
    },

    /// Complete a to-do item
    #[command(arg_required_else_help = true)]
    Check {
        /// The task to complete
        id: u32,
    },

    /// Uncheck a to-do item
    #[command(arg_required_else_help = true)]
    Uncheck {
        /// The task to uncomplete
        id: u32,
    },

    /// Remove a to-do item
    #[command(arg_required_else_help = true)]
    Remove {
        /// The task to remove
        id: u32,
    },

    /// Clear all to-do items
    #[command(subcommand, arg_required_else_help = true)]
    Clear(ClearCommands),

    /// List all tasks
    List,
}

#[derive(Subcommand, Debug)]
enum ClearCommands {
    /// Clear all to-do items
    All,

    /// Clear all completed to-do items before today
    Old,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("todo.db")?;
    todo_item::create_table(&conn)?;

    let args = Cli::parse();

    match &args.command {
        Commands::List => {
            list_todo_items(&conn)?;
        }
        Commands::Add { task } => {
            add_todo_item(&conn, &TodoItem::new(task.to_string()))?;
        }
        Commands::Check { id } => {
            let selected_todo = get_todo(&conn, *id)?;
            if selected_todo.completed {
                println!("Task already completed");
                return Ok(());
            }

            check_todo(&conn, *id)?;

            list_todo_items(&conn)?;
        }
        Commands::Uncheck { id } => {
            let selected_todo = get_todo(&conn, *id)?;
            if !selected_todo.completed {
                println!("Task already not completed");
                return Ok(());
            }

            uncheck_todo(&conn, *id)?;

            list_todo_items(&conn)?;
        }
        Commands::Remove { id } => {
            remove_todo_item(&conn, *id)?;

            println!("Removed task with id: {}", id);
        }
        Commands::Clear(cmds) => match cmds {
            ClearCommands::All => {
                remove_all_todos(&conn)?;

                println!("Cleared all todos");
            }
            ClearCommands::Old => {
                remove_overdue_todos(&conn)?;

                println!("Cleared all overdue todos");
            }
        },
    }

    Ok(())
}

fn list_todo_items(conn: &Connection) -> rusqlite::Result<()> {
    let todo_items = get_all_todo_items(conn)?;
    if todo_items.is_empty() {
        println!("No tasks found");
        return Ok(());
    }

    for item in todo_items {
        println!("{}", item);
    }

    Ok(())
}
