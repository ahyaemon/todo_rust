use serde::Serialize;

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct Todo {
    id: String,
    title: String,
    description: String,
}

impl Todo {

    pub fn new(
        id: String,
        title: String,
        description: String,
    ) -> Self {
        Todo { id, title, description }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
