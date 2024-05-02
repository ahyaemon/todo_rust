use crate::domain::todo::Todo;

pub trait TodoRepository {

    fn get(&self, id: &str) -> Todo;

    fn list(&self) -> Vec<Todo>;

    fn add(&self, todo: Todo) -> Todo;

    fn delete(&self, id: &str);
}
