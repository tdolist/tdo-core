
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub name: String,
    pub done: bool,
}


impl Todo {
    pub fn new(id: u32, name: &str) -> Todo {
        Todo {
            id: id,
            name: name.to_string(),
            done: false,
        }
    }
    pub fn edit(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }
    pub fn set_done(&mut self) {
        self.done = true;
    }

}
