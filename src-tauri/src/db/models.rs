use serde::{Deserialize, Serialize};

use crate::entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: String,
}
impl From<entity::note::Model> for Note {
    fn from(note: entity::note::Model) -> Self {
        Note {
            id: note.id,
            title: note.title,
            content: note.content,
            created_at: note.created_at.unwrap_or_default().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Categories {
    pub id: i32,
    pub name: String,
    pub created_at: String,
    pub is_delete: i32,
}

impl From<entity::category::Model> for Categories {
    fn from(category: entity::category::Model) -> Self {
        Categories {
            id: category.id,
            name: category.name,
            created_at: category.created_at.unwrap_or_default().to_string(),
            is_delete: category.is_delete.unwrap_or_default(),
        }
    }
}
