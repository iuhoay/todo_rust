use super::entity::Todo;

pub struct TodoRepository {
    pub tasks: Vec<Todo>,
    pub next_id: usize,
}

impl TodoRepository {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, task: String) -> Result<(), String> {
        let task = task.trim().to_string();
        if task.is_empty() {
            return Err("Task cannot be empty".into());
        }
        self.tasks.push(Todo::new(self.next_id, task));
        self.next_id += 1;
        Ok(())
    }

    pub fn list_tasks(&self) -> Vec<Todo> {
        self.tasks.clone()
    }

    pub fn mark_as_done(&mut self, id: usize) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.mark_done();
            Ok(())
        } else {
            Err(format!("Task with id {} not found", id))
        }
    }

    pub fn remove_task(&mut self, id: usize) -> Result<(), String> {
        if let Some(position) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(position);
            Ok(())
        } else {
            Err(format!("Task with id {} not found", id))
        }
    }
}
