use crate::db::{func, models};

#[tauri::command]
pub async fn get_note_by_id(id: i32) -> Option<models::Note> {
    return match func::get_note_by_id(id).await {
        Ok(note) => note,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            None
        }
    };
}
#[tauri::command]
pub async fn get_note_list() -> Option<Vec<models::Note>> {
    return match func::get_note_list().await {
        Ok(notes) => notes,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            None
        }
    };
}

#[tauri::command]
pub async fn create_note(title: String, content: String) -> bool {
    match func::create_note(&title, &content).await {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        }
    }
}
#[tauri::command]
pub async fn delete_note_by_id(id: i32) -> bool {
    match func::delete_note_by_id(id).await {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        }
    }
}
#[tauri::command]
pub async fn confirm_delete_note_by_id(id: i32) -> bool {
    match func::confirm_delete_note_by_id(id).await {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        }
    }
}
#[tauri::command]
pub async fn update_note_by_id(id: i32, title: String, content: String) -> bool {
    match func::update_note_by_id(id, &title, &content).await {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        }
    }
}
#[tauri::command]
pub async fn get_total_number() -> i32 {
    return match func::get_total_number().await {
        Ok(number) => number,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            0
        }
    };
}
