use todo::Todo;

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
        Tdo {
            lists: Vec::new(),
        }
    }
}
