//! General implementation of todo lists.
use todo::Todo;
use error::*;

/// Simple todo list structure.
///
/// Todos can be grouped together in so called todo lists (as in the real world).
/// Therefore, the `TodoList` struct can be used. It's a simple data structure that holds a
/// number of `Todo` items and offers all basic functions for managing them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoList {
    /// Name of the todo list.
    pub name: String,
    /// The actual vector of `Todo` items.
    pub list: Vec<Todo>,
}

impl TodoList {
    /// Create a new list with the given name.
    ///
    /// # Example
    ///
    /// ```
    /// # use tdo_core::list::*;
    /// let mut list = TodoList::new("important");
    /// ```
    pub fn new(name: &str) -> TodoList {
        TodoList {
            name: name.to_string(),
            list: Vec::new(),
        }
    }

    /// Add a new todo to the list.
    ///
    /// # Example
    ///
    /// ```
    /// # use tdo_core::list::*;
    /// # use tdo_core::todo::*;
    /// # let mut list = TodoList::new("important");
    /// list.add(Todo::new(0,"A first important todo"));
    /// ```

    pub fn add(&mut self, new_todo: Todo) {
        self.list.push(new_todo);
    }


    /// Mark a todo from the list with the given ID as done.
    ///
    /// This function returns a `ResultType`, which will contain a `TodoError::NotInList`
    /// if the list does not contain any todo with the given ID.
    pub fn done_id(&mut self, id: u32) -> TdoResult<()> {
        match self.list.iter().position(|x| x.id == id) {
            Some(index) => Ok(self.list[index].set_done()),
            None => Err(TodoError::NotInList.into()),
        }
    }

    /// Remove a todo with the given ID from the list.
    ///
    /// This function returns a `ResultType`, which will contain the removed Todo itself or a
    /// `TodoError::NotInList` if the list does not contain any todo with the given id.
    pub fn remove_id(&mut self, id: u32) -> TdoResult<Todo> {
        match self.list.iter().position(|x| x.id == id) {
            Some(index) => Ok(self.list.remove(index)),
            None => Err(TodoError::NotInList.into()),
        }
    }

    /// Search for all undone todos in the list.
    ///
    /// Returns a vector of all undone todos.
    pub fn list_undone(&self) -> Vec<Todo> {
        let mut undone: Vec<Todo> = vec![];
        for entry in self.to_owned().list.into_iter() {
            if !entry.done {
                undone.push(entry);
            }
        }
        undone
    }

    /// Remove all done todos from the list.
    pub fn clean(&mut self) {
        for entry in self.to_owned().list.into_iter() {
            if entry.done {
                let _ = self.remove_id(entry.id);
            }
        }
    }
}

/// Instanciates a _default_ `TodoList`.
/// This function is invoked when a `Tdo` container structure is instanciated.
impl Default for TodoList {
    fn default() -> TodoList {
        TodoList {
            name: "default".to_string(),
            list: Vec::new(),
        }
    }
}
