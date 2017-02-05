use todo::Todo;
use std::fs::File;
use error::{StorageError, TodoError};

/// The representation of a todo list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoList {
    pub name: String,
    pub list: Vec<Todo>,
}


impl TodoList {
    /// Create a new list with the given name
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

    /// Add a new todo to the list
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


    /// Mark a todo with the given id as done
    ///
    /// This function returns a `ResultType` wich will contain a `TodoError::NotInList` if the list does not contain any todo with the given id.
    pub fn done_id(&mut self, id: u32) -> Result<(), TodoError> {
        match self.list.iter().position(|x| x.id == id) {
            Some(index) => Ok(self.list[index].set_done()),
            None => Err(TodoError::NotInList),
        }
    }

    /// Remove a todo with the given id
    ///
    /// This function returns a `ResultType` wich will contain the removed Todo itself of a `TodoError::NotInList` if the list does not contain any todo with the given id.
    pub fn remove_id(&mut self, id: u32) -> Result<Todo, TodoError> {
        match self.list.iter().position(|x| x.id == id) {
            Some(index) => Ok(self.list.remove(index)),
            None => Err(TodoError::NotInList),
        }
    }

    /// Search for all undone todos in the list
    ///
    /// Returns a Vector of all undone todos.
    pub fn list_undone(&self) -> Vec<Todo> {
        let mut undone: Vec<Todo> = vec![];
        for entry in self.to_owned().list.into_iter() {
            if !entry.done {
                undone.push(entry);
            }
        }
        undone
    }

    /// Remove all done todos of the list
    pub fn clean(&mut self) {
        for entry in self.to_owned().list.into_iter() {
            if entry.done {
                let _ = self.remove_id(entry.id);
            }
        }
    }
}

/// Instanciates the default TodoList of a new Tdo
impl Default for TodoList {
    fn default() -> TodoList {
        TodoList {
            name: "default".to_string(),
            list: Vec::new(),
        }
    }
}

/// The representation of all TodoLists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tdo {
    pub lists: Vec<TodoList>,
}

impl Tdo {
    /// Create a new Tdo with a default `TodoList`
    ///
    /// # Example
    ///
    /// ```
    /// # use tdo_core::list::*;
    /// let tdo = Tdo::new();
    /// ```
    pub fn new() -> Tdo {
        Tdo { lists: vec![TodoList::default()] }
    }

    /// Load a saved Tdo from JSON
    ///
    /// This function returns a `ResultType` wich will contain the deserialized JSON as a `Tdo` or a `serde_json::Error`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tdo_core::list::*;
    /// let mut tdo = Tdo::load("foo.json");
    /// ```
    pub fn load(path: &str) -> Result<Tdo, super::serde_json::Error> {
        let file = File::open(path).unwrap();
        super::serde_json::from_reader(&file)
    }

    /// Dump the Tdo to JSON
    ///
    /// This function returns a `ResultType` with an `StorageError::SaveFailure` if there could not be created a JSON file.
    ///
    /// # Example
    ///
    /// ```
    /// # use tdo_core::list::*;
    /// # let mut tdo = Tdo::new();
    /// let res = tdo.save("foo.json");
    /// assert_eq!(res.unwrap(), ());
    /// ```
    pub fn save(&self, path: &str) -> Result<(), StorageError> {
        match File::create(path) {
            Ok(mut f) => {
                let _ = super::serde_json::to_writer_pretty(&mut f, self);
                Ok(())
            }
            Err(_) => Err(StorageError::SaveFailure),
        }
    }

    /// Add a list
    pub fn add_list(&mut self, list: TodoList) {
        self.lists.push(list);
    }

    /// Add a todo to the given list
    ///
    /// This function returns a `ResultType` with an `TodoError::NoSuchList` if there is no matching list found.
    pub fn add_todo(&mut self, list_name: &str, todo: Todo) -> Result<(), TodoError> {
        match self.get_list_index(&list_name) {
            Ok(index) => {
                self.lists[index].add(todo);
                Ok(())
            }
            Err(x) => Err(x),
        }
    }

    /// Cycle through all lists and mark a todo with the given id as done
    pub fn done_id(&mut self, id: u32) {
        for list in 0..self.lists.len() {
            let _ = self.lists[list].done_id(id);
        }
    }

    /// Cycle through all lists and remove a todo with the given id
    pub fn remove_id(&mut self, id: u32) {
        for mut list in self.to_owned().lists.into_iter() {
            let _ = list.remove_id(id);
        }
    }

    /// Remove all done todos from all lists
    pub fn clean_lists(&mut self) {
        for list in 0..self.lists.len() {
            self.lists[list].clean();
        }
    }

    fn get_list_index(&self, name: &str) -> Result<usize, TodoError> {
        match self.lists.iter().position(|x| x.name == name.to_string()) {
            Some(index) => Ok(index),
            None => Err(TodoError::NoSuchList),
        }
    }
}
