use crate::db::{
    funcs::{note, total::add_total_number},
    models,
};

use super::server::new_connection;

#[tauri::command]
pub async fn get_note_by_id(id: i32) -> Option<models::Note> {
    let db = new_connection().await;
    return match note::get_note_by_id(&db, id).await {
        Ok(note) => note,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            None
        }
    };
}
#[tauri::command]
pub async fn get_note_list() -> Option<Vec<models::Note>> {
    let db = new_connection().await;
    return match note::get_note_list(&db).await {
        Ok(notes) => notes,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            None
        }
    };
}

#[tauri::command]
pub async fn create_note(title: String, content: String, word_cnt: i32) -> bool {
    let db = new_connection().await;
    match note::create_note(&db, &title, &content).await {
        Ok(_) => {
            return add_total_number(&db, word_cnt).await.is_ok();
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return false;
        }
    }
}
#[tauri::command]
pub async fn delete_note_by_id(id: i32) -> bool {
    let db = new_connection().await;
    match note::delete_note_by_id(&db, id).await {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        }
    }
}
#[tauri::command]
pub async fn confirm_delete_note_by_id(id: i32) -> bool {
    let db = new_connection().await;
    match note::confirm_delete_note_by_id(&db, id).await {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        }
    }
}
#[tauri::command]
pub async fn update_note_by_id(id: i32, title: String, content: String) -> bool {
    let db = new_connection().await;
    match note::update_note_by_id(&db, id, &title, &content).await {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        }
    }
}
#[tauri::command]
pub async fn ping() -> String {
    format!("Pong")
}
