//! Implementation for one Todo.

/// The Todo Struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    /// Unique identifier for every Todo.
    pub id: u32,
    /// Content of the Todo.
    pub name: String,
    /// Status of the Todo.
    pub done: bool,
}


impl Todo {
    /// Creates a new Todo.
    pub fn new(id: u32, name: &str) -> Todo {
        Todo {
            id: id,
            name: name.to_string(),
            done: false,
        }
    }

    /// Edit a given Todo.
    pub fn edit(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }

    /// Change the Todo value to done.
    pub fn set_done(&mut self) {
        self.done = true;
    }

}
