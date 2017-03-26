//! General implementation of todo lists and the base structure.

use std::fs::File;

use todo::Todo;
use error::*;


/// Simple todo list structure.
///
/// Todos can be grouped together in so called todo lists (as in the real world).
/// Therefore, the `TodoList` struct can be used. It's a simple data structure that holds a number of `Todo` items and offers all basic functions for managing them.
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
    /// This function returns a `ResultType`, which will contain a `TodoError::NotInList` if the list does not contain any todo with the given ID.
    pub fn done_id(&mut self, id: u32) -> Result<(), TodoError> {
        match self.list.iter().position(|x| x.id == id) {
            Some(index) => Ok(self.list[index].set_done()),
            None => Err(TodoError::NotInList),
        }
    }

    /// Remove a todo with the given ID from the list.
    ///
    /// This function returns a `ResultType`, which will contain the removed Todo itself or a `TodoError::NotInList` if the list does not contain any todo with the given id.
    pub fn remove_id(&mut self, id: u32) -> Result<Todo, TodoError> {
        match self.list.iter().position(|x| x.id == id) {
            Some(index) => Ok(self.list.remove(index)),
            None => Err(TodoError::NotInList),
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


/// Basic container structure for a set of todo lists.
///
/// This data structure acts as a conatiner for all todo lists and its associated todos.
/// The whole `tdo` microcosm settles around this structure which is also used for (de-)serialization.
///
/// When instanciated, it comes with an empty _default_ list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tdo {
    /// A vector of all todo lists.
    pub lists: Vec<TodoList>,
    /// The tdo version the last dump was saved with.
    version: String,
}

impl Tdo {
    /// Create a new `Tdo` container.
    /// Each new container is instanciated with a _default_ `TodoList`.
    ///
    /// # Example
    ///
    /// ```
    /// # use tdo_core::list::*;
    /// let tdo = Tdo::new();
    /// ```
    pub fn new() -> Tdo {
        Tdo {
            lists: vec![TodoList::default()],
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Load a saved `Tdo` container from a JSON file.
    ///
    /// This function returns a `ResultType` which will yield the deserialized JSON or a `serde_json::Error`.
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

    /// Dump the `Tdo` container to a JSON file.
    ///
    /// This function returns a `ResultType` yielding a `StorageError::SaveFailure` if the JSON file could not be opened/saved.
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
        // TODO: At this point we could be much more precise about the error if we would include
        // the error from the file system as SaveFailure(ArbitraryErrorFromFS)
        //  -- Feliix42 (2017-03-14; 17:04)
        match File::create(path) {
            Ok(mut f) => {
                let _ = super::serde_json::to_writer_pretty(&mut f, self);
                Ok(())
            }
            Err(_) => Err(StorageError::SaveFailure),
        }
    }

    /// Add a todo list to the container.
    pub fn add_list(&mut self, list: TodoList) -> TdoResult<()> {
        match self.get_list_index(&list.name) {
            Ok(_) => Err(TodoError::NameAlreadyExists.into()),
            Err(_) => {
                self.lists.push(list);
                Ok(())
            }
        }
    }

    /// Removes a list from the container.
    pub fn remove_list(&mut self, list_name: &str) -> TdoResult<()>{
        if list_name == "default" {
            Err(TodoError::CanNotRemoveDefault.into())
        } else {
            match self.get_list_index(list_name) {
                Ok(index) => {
                    self.lists.remove(index);
                    Ok(())
                },
                Err(_) => Err(TodoError::NoSuchList.into()),
            }
        }
    }

    /// Add a todo to the todo list, identified by its name.
    ///
    /// This function returns a `ResultType` with a `TodoError::NoSuchList` if there is no matching list found.
    pub fn add_todo(&mut self, list_name: Option<&str>, todo: Todo) -> TdoResult<()> {
        match self.get_list_index(&list_name.unwrap_or("default")) {
            Ok(index) => {
                self.lists[index].add(todo);
                Ok(())
            }
            Err(x) => Err(x),
        }
    }

    /// Cycle through all todo lists and mark a todo with the given IDas done.
    /// This function has no return value and thus won't indicate whether there was a matching todo found.
    pub fn done_id(&mut self, id: u32) {
        for list in 0..self.lists.len() {
            let _ = self.lists[list].done_id(id);
        }
    }

    /// Cycle through all todo lists and remove a todo with the given id.
    /// This function has no return value and thus won't indicate whether there was a matching todo found.
    pub fn remove_id(&mut self, id: u32) {
        for mut list in self.to_owned().lists.into_iter() {
            let _ = list.remove_id(id);
        }
    }

    /// Remove all todos that have been marked as _done_ from all todo lists.
    pub fn clean_lists(&mut self) {
        for list in 0..self.lists.len() {
            self.lists[list].clean();
        }
    }


    fn get_list_index(&self, name: &str) -> TdoResult<usize> {
        match self.lists.iter().position(|x| x.name.to_lowercase() == name.to_string().to_lowercase()) {
            Some(index) => Ok(index),
            None => Err(TodoError::NoSuchList.into()),
        }
    }
}
