mod menu;
mod todo;

use std::io;

use menu::{add_task, display_menu, list_tasks, mark_as_done, remove_task};
use todo::Todo;

fn main() {
    let mut tasks: Vec<Todo> = Vec::new();
    let mut next_id: usize = 1;

    loop {
        // Display menu to the user
        display_menu();

        // Read input from the user
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => add_task(&mut tasks, &mut next_id), // Add task
            2 => list_tasks(&tasks),                 // List tasks (id, task, done
            3 => mark_as_done(&mut tasks),           // Mark task as done
            4 => remove_task(&mut tasks),            // Remove task
            5 => break,
            _ => println!("Invalid choice. Please enter a number between 1 and 5"),
        }
    }
}
