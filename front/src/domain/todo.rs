use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Todo {
    pub id: String,
    pub title: String,
}
