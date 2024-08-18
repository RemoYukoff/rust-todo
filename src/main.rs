use std::io::{self, Write};

struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            completed: false,
        }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    fn list_tasks(&mut self) {
        for (index, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "✓" } else { " " };
            println!("{}: [{}] {}", index + 1, status, task.description);
        }
    }

    fn complete_task(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.complete();
        }
    }
}
fn main() {
    let mut todo_list = TodoList::new();
    loop {
        println!("Options:");
        println!("1. Add a task");
        println!("2. List tasks");
        println!("3. Complete a task");
        println!("4. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        match choice.trim() {
            "1" => {
                print!("Enter the task description: ");
                io::stdout().flush().unwrap();

                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read line");

                todo_list.add_task(description.trim().to_string());
            }
            "2" => {
                todo_list.list_tasks();
            }
            "3" => {
                print!("Enter the task number to complete: ");
                io::stdout().flush().unwrap();

                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read line");

                if let Ok(index) = index.trim().parse::<usize>() {
                    todo_list.complete_task(index - 1);
                } else {
                    println!("Please enter a valid number.");
                }
            }
            "4" => {
                break;
            }
            _ => {
                println!("Invalid choice, please try again.")
            }
        }
    }
}
