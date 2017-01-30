use todo::Todo;
use std::fs::File;
use storage::StorageError;

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tdo {
    pub lists: Vec<TodoList>,
}

impl Tdo {
    pub fn new() -> Tdo {
        Tdo { lists: Vec::new() }
    }

    pub fn load(path: &str) -> Tdo {
        let file = File::open(path).unwrap();
        super::serde_json::from_reader(&file).unwrap()
    }

    pub fn save(&self, path: &str) -> Result<(), StorageError> {
        match File::create(path) {
            Ok(mut f) => {
                let _ = super::serde_json::to_writer_pretty(&mut f, self);
                Ok(())
            },
            Err(_) => Err(StorageError::FileCorrupted)
        }


    }
}
