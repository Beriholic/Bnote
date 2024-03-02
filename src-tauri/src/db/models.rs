use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: String,
}
