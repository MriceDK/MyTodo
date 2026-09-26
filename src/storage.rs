use crate::todo::TodoList;
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn data_file_path() -> PathBuf {
    // To store the data in the local share folder (Linux) / Appdata (Windows)
    // let mut path = dirs::data_dir().expect("no data dir found");
    // path.push("my-todo-app");
    // fs::create_dir_all(&path).ok();
    // path.push("todos.json");
    // path
    PathBuf::from("todos.json")
}

pub fn load(path: &PathBuf, default_name: &str) -> TodoList {
    fs::read_to_string(path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_else(|| TodoList::new(default_name.to_string()))
}

pub fn save(list: &TodoList, path: &PathBuf) -> io::Result<()> {
    let data = serde_json::to_string_pretty(list)?;
    fs::write(path, data)
}