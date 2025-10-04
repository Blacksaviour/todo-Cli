use clap::{Parser,Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process;


const TASK_FILE: &str = "tasks.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task{
    id: u64,
    description: String,
    done: bool,
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Simple CLI To-Do App (Rust + Clap + Serde)")]
struct Cli{
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands{
    Add {description: String},
    List,
    Done{id: u64},
    Delete{id: u64},
    Edit { id: u64, description: String },
}
fn load_tasks() ->Vec<Task> {
    let path = PathBuf::from(TASK_FILE);
    if !path.exists(){
        return Vec::new();
    }
    let mut file = File::open(path).expect("failed to open task file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("failed to read tasks");
    serde_json::from_str(&data).unwrap_or_default()
}

fn save_tasks(tasks: &[Task]){
    let data = serde_json::to_string_pretty(tasks).expect("failed to serialize task");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(TASK_FILE)
        .expect("failed to open tasks file for writing");
    file.write_all(data.as_bytes())
        .expect("failed to write task file");

}

fn main(){
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match cli.command {
        Commands::Add {description} => {
            let next_id = tasks.last().map_or(1, |t| t.id + 1);
            let task = Task {
                id: next_id,
                description,
                done: false,
            };
            tasks.push(task);
            save_tasks(&tasks);
            println!("Task {} added.", next_id);
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No tasks found.");
            } else {
                for t in &tasks{
                    println!("{}: [{}] {}",
                             t.id,
                             if t.done {"x"} else {" "},
                             t.description);
                }
            }
        }
        Commands::Done {id} => {
           if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
                    t.done = true;
                    save_tasks(&tasks);
                    println!("Task {} marked as done.", id);
                }
                else {
                    eprintln!("Task {} not found.", id);
                    process::exit(1);
                }
            }
        Commands::Delete {id} => {
            let len_before = tasks.len();
            tasks.retain(|t| t.id != id);
            if tasks.len() < len_before {
                save_tasks(&tasks);
                println!("Task {} deleted.", id);
            }else {
                eprintln!("Task {} not found.", id);
                process::exit(1);
            }
        }
        Commands::Edit { id, description } => {
            if let Some(t) = tasks.iter_mut().find(|t| t.id == id) {
                t.description = description;
                save_tasks(&tasks);
                println!("Task {} updated.", id);
            } else {
                eprintln!("Task {} not found.", id);
                process::exit(1);
            }
        }
        }

        }

