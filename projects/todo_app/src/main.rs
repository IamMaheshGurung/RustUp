use rusqlite::{params, Connection, Result};
use clap::{Parser, Subcommand};



#[derive(Parser)]
#[command(name = "Todo")]
#[command(about = "A MultiUser todo app", version = "1.0")]



struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    Register {
        username: String,
        password: String,
    },

    //for login
    Login {
        username: String,
        password: String,
    },

    //for adding task
    Add {
         description: String,
        },
    //for listing tasks
    List,
    //for marking as done
    Done { id: i32 },

    //logout command
    Logout,
}


#[derive(Debug)]
struct Task {
    id: i32,
    description: String,
    done: bool,
}


fn init_db(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            description TEXT NOT NULL,
            done BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    ).expect("Failed to create tasks table");
}

fn add_task(conn: &Connection, description: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO tasks (description, done) VALUES (?1, 0)",
        params![description],
    )?;
    Ok(())
}

fn list_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, description, done FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            description: row.get(1)?,
            done: row.get(2)?,
        })
    })?;

    let tasks: Result<Vec<Task>> = task_iter.collect();
    tasks
}

fn mark_done(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("UPDATE tasks SET done = 1 WHERE id = ?1", params![id])?;
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let conn = Connection::open("todo.db")?;
    
    init_db(&conn);

    match cli.command {
        Command::Register{ username, password} => auth::register( &username, &password)?,
        Command::Login { username, password } => {
            if auth::login(&conn, &username, &password)? {
                println!("Login successful for user: {}", username);
            } else {
                println!("Login failed for user: {}", username);
            }
        }
        Commands::Add { description } => {
            add_task(&conn, &description)?;
            println!("Task added: {}", description);
        }
        Commands::List => {
            let tasks = list_tasks(&conn)?;
            for task in tasks {
                println!("{}: {} [{}]", task.id, task.description, if task.done { "Done" } else { "Not Done" });
            }
        }
        Commands::Done { id } => {
            mark_done(&conn, id)?;
            println!("Task {} marked as done", id);
        }
    }

    Ok(())
}