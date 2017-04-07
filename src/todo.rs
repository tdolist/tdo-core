//! Implementation of a single Todo item.

/// Data Structure for a simple todo.
///
/// A `Todo` item is the atomic unit within the `tdo` microcosm.
/// It represents a single todo or task, which is identified by an ID.
/// Information about its state (_done_ or _undone_) can be derived from
/// the data structure as well as the title of the todo item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    /// Unique identifier for every Todo.
    pub id: u32,
    /// Title of the todo.
    pub name: String,
    /// Status of the todo.
    pub done: bool,
}


impl Todo {
    /// Constructor. Creates a new Todo item.
    pub fn new(id: u32, name: &str) -> Todo {
        Todo {
            id: id,
            name: name.to_string(),
            done: false,
        }
    }

    /// Edit the title of a given Todo.
    pub fn edit(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }

    /// Set the status of a Todo item to _done_.
    pub fn set_done(&mut self) {
        self.done = true;
    }

    /// Mark a todo item as _undone_.
    pub fn set_undone(&mut self) {
        self.done = false;
    }
}
