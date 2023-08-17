#[derive(Debug, Clone)]
pub struct Todo {
    pub id: usize,
    pub task: String,
    pub done: bool,
}

impl Todo {
    pub fn new(id: usize, task: String) -> Self {
        Self {
            id,
            task,
            done: false,
        }
    }

    pub fn mark_done(&mut self) {
        self.done = true;
    }
}
