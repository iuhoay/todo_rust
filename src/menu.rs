use crate::todo::repository::TodoRepository;
use std::io::{self, Write};

pub struct Menu {
    todo_repository: TodoRepository,
}

impl Menu {
    // The new function is a constructor that returns a new instance of the Menu struct.
    pub fn new() -> Self {
        Self {
            todo_repository: TodoRepository::new(),
        }
    }

    pub fn render(&mut self) {
        loop {
            // Display menu to the user
            self.display_menu();

            // Read input from the user
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice: u8 = match choice.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match choice {
                1 => self.add_task(),     // Add task
                2 => self.list_tasks(),   // List tasks (id, task, done
                3 => self.mark_as_done(), // Mark task as done
                4 => self.remove_task(),  // Remove task
                5 => break,
                _ => println!("Invalid choice. Please enter a number between 1 and 5"),
            }
        }
    }

    fn display_menu(&self) {
        println!("Todo List");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Mark task as done");
        println!("4. Remove task");
        println!("5. Exit");
        println!("Enter your choice:");
        io::stdout().flush().unwrap();
    }

    fn add_task(&mut self) {
        print!("Enter task: ");
        io::stdout().flush().unwrap();

        let mut task = String::new();
        io::stdin().read_line(&mut task).unwrap();

        self.todo_repository.add_task(task);
    }

    fn list_tasks(&self) {
        for task in self.todo_repository.list_tasks() {
            println!("ID: {}, Task: {}, Done: {}", task.id, task.task, task.done);
        }
    }

    fn mark_as_done(&mut self) {
        println!("Enter the ID of the task to mark as done:");
        io::stdout().flush().unwrap();

        let mut id = String::new();
        io::stdin().read_line(&mut id).unwrap();
        let id: usize = match id.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                return;
            }
        };

        self.todo_repository.mark_as_done(id);
    }

    fn remove_task(&mut self) {
        print!("Enter the ID of the task to remove:");
        io::stdout().flush().unwrap();

        let mut id = String::new();
        io::stdin().read_line(&mut id).unwrap();
        let id: usize = match id.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                return;
            }
        };

        self.todo_repository.remove_task(id);
    }
}
