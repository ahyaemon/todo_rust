use crate::application::todo_repository::TodoRepository;
use crate::domain::todo::Todo;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct TodoRepositoryImpl {
    hash_map: Mutex<HashMap<String, Todo>>,
}

impl TodoRepositoryImpl {
    pub fn new(hash_map: Mutex<HashMap<String, Todo>>) -> Self {
        let uuid1 = "501d09e6-c484-47e3-941a-7496c61d224b".to_string();
        let todo1 = Todo::new(
            uuid1.clone(),
            "title1".to_string(),
            "This is description. This is description. This is description.".to_string(),
        );
        hash_map.lock().unwrap().insert(uuid1, todo1);
        let uuid2 = "3ed82377-3072-4b79-8bdb-d4c3158fd755".to_string();
        let todo2 = Todo::new(
            uuid2.clone(),
            "title2".to_string(),
            "This is description. This is description. This is description.".to_string(),
        );
        hash_map.lock().unwrap().insert(uuid2, todo2);
        TodoRepositoryImpl { hash_map }
    }
}

impl TodoRepository for TodoRepositoryImpl {
    fn get(&self, id: &str) -> Todo {
        self.hash_map.lock().unwrap().get(id).unwrap().clone()
    }

    fn list(&self) -> Vec<Todo> {
        let mut vec: Vec<Todo> = self
            .hash_map
            .lock()
            .unwrap()
            .clone()
            .into_values()
            .collect();
        vec.sort_by(|a, b| a.id().cmp(b.id()));
        vec
    }

    fn add(&self, todo: Todo) -> Todo {
        let key = String::from(todo.id());
        let value = todo.clone();
        self.hash_map.lock().unwrap().insert(key, value);
        todo
    }

    fn delete(&self, id: &str) {
        self.hash_map.lock().unwrap().remove(id);
    }
}

#[cfg(test)]
mod tests {
    use crate::adapter::todo_repository_impl::TodoRepositoryImpl;
    use crate::application::todo_repository::TodoRepository;
    use crate::domain::todo::Todo;
    use std::collections::HashMap;
    use std::sync::Mutex;

    fn get_repository() -> TodoRepositoryImpl {
        let hash_map = Mutex::new(HashMap::new());
        TodoRepositoryImpl::new(hash_map)
    }

    fn create_todo(post_fix: &str) -> Todo {
        Todo::new(
            format!("id{}", post_fix),
            format!("title{}", post_fix),
            format!("description{}", post_fix),
        )
    }

    #[test]
    fn add() {
        let repository = get_repository();

        let todo = create_todo("1");
        let actual = repository.add(todo);
        let expected = create_todo("1");
        assert_eq!(actual, expected);
    }

    #[test]
    fn get() {
        let repository = get_repository();

        let todo1 = create_todo("1");
        repository.add(todo1.clone());
        let actual1 = repository.get("id1");
        assert_eq!(actual1, todo1);

        let todo2 = create_todo("2");
        repository.add(todo2.clone());
        let actual2 = repository.get("id2");
        assert_eq!(actual2, todo2);
    }

    #[test]
    fn list() {
        let repository = get_repository();

        let uuid1 = "501d09e6-c484-47e3-941a-7496c61d224b".to_string();
        let todo_pre1 = Todo::new(
            uuid1.clone(),
            "title1".to_string(),
            "This is description. This is description. This is description.".to_string(),
        );
        let uuid2 = "3ed82377-3072-4b79-8bdb-d4c3158fd755".to_string();
        let todo_pre2 = Todo::new(
            uuid2.clone(),
            "title2".to_string(),
            "This is description. This is description. This is description.".to_string(),
        );

        let todo1 = create_todo("1");
        repository.add(todo1.clone());
        let todo2 = create_todo("2");
        repository.add(todo2.clone());

        let actual = repository.list();
        assert_eq!(actual, vec![todo_pre2, todo_pre1, todo1, todo2])
    }

    #[test]
    fn delete() {
        let repository = get_repository();

        let todo1 = create_todo("1");
        repository.add(todo1.clone());
        let todo2 = create_todo("2");
        repository.add(todo2.clone());
        // NOTE 最初から要素を二つ入れている
        assert_eq!(repository.list().len(), 4);

        repository.delete("id1");
        assert_eq!(repository.list().len(), 3);

        repository.delete("id2");
        assert_eq!(repository.list().len(), 2);
    }
}
