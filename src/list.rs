use todo::Todo;
use std::fs::File;
use error::{StorageError, TodoError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoList {
    pub name: String,
    pub list: Vec<Todo>,
}


impl TodoList {
    pub fn new(name: &str) -> TodoList {
        TodoList {
            name: name.to_string(),
            list: Vec::new(),
        }
    }

    pub fn list_undone(&self) -> Vec<Todo> {
        let mut undone: Vec<Todo> = vec![];
        for entry in self.to_owned().list.into_iter() {
            if !entry.done{
                undone.push(entry);
            }
        }
        undone
    }

    pub fn add(&mut self, new_todo: Todo) {
        self.list.push(new_todo);
    }

    pub fn remove_name(&mut self, name: &str) -> Result<Todo, TodoError> {
        match self.list.iter().position(|x| x.name == name.to_string()){
            Some(index) => Ok(self.list.remove(index)),
            None => Err(TodoError::NotInList)
        }
    }

    pub fn remove_id(&mut self, id: u32) -> Result<Todo, TodoError> {
        match self.list.iter().position(|x| x.id == id){
            Some(index) => Ok(self.list.remove(index)),
            None => Err(TodoError::NotInList)
        }
    }

    pub fn clean(&mut self) {
        for entry in self.to_owned().list.into_iter() {
            if entry.done {
                let _ = self.remove_id(entry.id);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tdo {
    pub lists: Vec<TodoList>,
}

impl Tdo {
    pub fn new() -> Tdo {
        Tdo { lists: Vec::new() }
    }

    pub fn load(path: &str) -> Result<Tdo, super::serde_json::Error> {
        let file = File::open(path).unwrap();
        super::serde_json::from_reader(&file)
    }

    pub fn save(&self, path: &str) -> Result<(), StorageError> {
        match File::create(path) {
            Ok(mut f) => {
                let _ = super::serde_json::to_writer_pretty(&mut f, self);
                Ok(())
            },
            Err(_) => Err(StorageError::SaveFailure)
        }


    }
}
