use serde::Serialize;

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct Todo {
    id: String,
    title: String,
}

impl Todo {

    pub fn new(id: &str, title: &str) -> Self {
        Todo { id: String::from(id), title: String::from(title) }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
