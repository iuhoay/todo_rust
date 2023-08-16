use crate::todo::Todo;
use std::io::{self, Write};

pub fn display_menu() {
    println!("Todo List");
    println!("1. Add task");
    println!("2. List tasks");
    println!("3. Mark task as done");
    println!("4. Remove task");
    println!("5. Exit");
    println!("Enter your choice:");
    io::stdout().flush().unwrap();
}

pub fn add_task(tasks: &mut Vec<Todo>, next_id: &mut usize) {
    print!("Enter task: ");
    io::stdout().flush().unwrap();

    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();
    tasks.push(Todo::new(*next_id, task.trim().to_string()));
    *next_id += 1;
}

pub fn list_tasks(tasks: &Vec<Todo>) {
    for task in tasks {
        println!("ID: {}, Task: {}, Done: {}", task.id, task.task, task.done);
    }
}

pub fn mark_as_done(tasks: &mut Vec<Todo>) {
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

    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.mark_done();
    } else {
        println!("Task with ID {} not found", id);
    }
}

pub fn remove_task(tasks: &mut Vec<Todo>) {
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

    tasks.retain(|task| task.id != id);
}
