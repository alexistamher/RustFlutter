use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}