#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: String) {
        let id = (self.tasks.len() + 1) as u32;
        let task = Task {
            id,
            description,
            completed: false,
        };
        self.tasks.push(task);
        println!("✅ Task added successfully!");
    }

    pub fn display_tasks(&self) {
        if self.tasks.is_empty() {
            println!("📭 No tasks found. Try adding one!");
            return;
        }

        println!("\nYour Tasks:");
        for task in &self.tasks {
            let status = if task.completed { "✅ Done" } else { "⏳ Pending" };
            println!("{}: {} [{}]", task.id, task.description, status);
        }
    }

    pub fn mark_done(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("🎉 Task '{}' marked as done!", task.description);
                return;
            }
        }
        println!("❌ Task not found!");
    }
}
