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

    pub fn add_task(&mut self, task: String) {
        let task = task.trim().to_string();
        self.tasks.push(Todo::new(self.next_id, task));
        self.next_id += 1;
    }

    pub fn list_tasks(&self) -> Vec<Todo> {
        self.tasks.clone()
    }

    pub fn mark_as_done(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.mark_done();
        } else {
            println!("Task with id {} not found", id);
        }
    }

    pub fn remove_task(&mut self, id: usize) {
        self.tasks.retain(|task| task.id != id);
    }
}
