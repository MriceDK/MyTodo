use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    pub task: String,
    pub is_done: bool,
}

impl TodoItem {
    fn new(task: String) -> TodoItem {
        TodoItem { task, is_done: false }
    }

    pub fn mark_as_done(&mut self) {
        self.is_done = true;
    }

    pub fn mark_as_unfinished(&mut self) {
        self.is_done = false;
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = if self.is_done { "x" } else { " " };
        write!(f, "[{}] {}", status, self.task)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TodoList {
    pub name: String,
    pub todo_items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new(name: String) -> TodoList {
        TodoList {
            name,
            todo_items: Vec::new(),
        }
    }

    pub fn add(&mut self, task: String) {
        self.todo_items.push(TodoItem::new(task));
    }

    pub fn remove(&mut self, id: usize) {
        if id <= self.todo_items.len() {
            self.todo_items.remove(id);
        }
    }

    pub fn find_mut(&mut self, id: usize) -> Option<&mut TodoItem> {
        self.todo_items.get_mut(id)
    }

    pub fn mark_as_done(&mut self, id: usize) {
        if let Some(item) = self.find_mut(id) {
            item.mark_as_done();
        }
    }

    pub fn mark_as_unfinished(&mut self, id: usize) {
        if let Some(item) = self.find_mut(id) {
            item.mark_as_unfinished();
        }
    }
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}:", self.name)?;
        for (id, item) in self.todo_items.iter().enumerate() {
            writeln!(f, "  {}. {}", id + 1, item)?;
        }
        Ok(())
    }
}