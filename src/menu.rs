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
            let choice = match Self::read_input_as_usize() {
                Ok(num) => num,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
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

    fn read_input_as_usize() -> Result<usize, &'static str> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<usize>() {
            Ok(num) => Ok(num),
            Err(_) => Err("Please enter a number"),
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

        match self.todo_repository.add_task(task) {
            Ok(_) => println!("Task added successfully"),
            Err(err) => println!("Error: {}", err),
        }
    }

    fn list_tasks(&self) {
        for task in self.todo_repository.list_tasks() {
            println!("ID: {}, Task: {}, Done: {}", task.id, task.task, task.done);
        }
    }

    fn mark_as_done(&mut self) {
        println!("Enter the ID of the task to mark as done:");
        io::stdout().flush().unwrap();

        let id = match Self::read_input_as_usize() {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        match self.todo_repository.mark_as_done(id) {
            Ok(_) => println!("Task marked as done successfully"),
            Err(err) => println!("Error: {}", err),
        }
    }

    fn remove_task(&mut self) {
        print!("Enter the ID of the task to remove:");
        io::stdout().flush().unwrap();

        let id = match Self::read_input_as_usize() {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        match self.todo_repository.remove_task(id) {
            Ok(_) => println!("Task removed successfully"),
            Err(err) => println!("Error: {}", err),
        }
    }
}
