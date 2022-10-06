use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum TaskType {
    TODO,
    HABIT,
}
