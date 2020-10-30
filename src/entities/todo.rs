#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u8,
    pub content: String,
    pub completed: bool
}

#[remote]
pub fn get_todo() -> Todo {
    Todo { id: 10, content: String::from("Hello craig!"), completed: false }
}

#[remote]
pub fn add_todo(mut new_todo: Todo) {
    println!("Got a new todo! {:?}", new_todo);
}