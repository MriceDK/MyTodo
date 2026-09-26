// lib.rs
pub mod todo;
pub mod storage;

pub use todo::{TodoItem, TodoList};
pub use storage::{data_file_path, load, save};