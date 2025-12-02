mod task;
mod utils;

use std::io;
use task::{Task, TaskList};

fn main() {
    println!("🦀 Welcome to Abiloye's Rust Task Manager 🦀");

    let mut task_list = TaskList::new();

    loop {
        println!("\n--- MENU ---");
        println!("1. Add a Task");
        println!("2. View Tasks");
        println!("3. Mark Task as Done");
        println!("4. Exit");
        println!("Choose an option: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("Enter task description:");
                let description = utils::get_input();
                task_list.add_task(description);
            }
            "2" => task_list.display_tasks(),
            "3" => {
                println!("Enter task ID to mark as done:");
                let id_input = utils::get_input();
                if let Ok(id) = id_input.trim().parse::<u32>() {
                    task_list.mark_done(id);
                } else {
                    println!("❌ Invalid ID. Please enter a number.");
                }
            }
            "4" => {
                println!("👋 Exiting... Goodbye!");
                break;
            }
            _ => println!("⚠️ Invalid choice. Please try again."),
        }
    }
}
