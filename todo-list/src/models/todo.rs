pub struct Todo {
    name: String,
    is_completed: bool,
}

impl Todo {
    pub fn new(name: String, is_completed: bool) -> Todo {
        Todo {
            name,
            is_completed
        }
    }
}
