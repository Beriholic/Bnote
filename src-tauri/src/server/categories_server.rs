use crate::db::{funcs::categories, models};

use super::server::new_connection;

#[tauri::command]
pub async fn create_categories(name: String) -> bool {
    let db = new_connection().await;
    let res = categories::create_categories(&db, &name).await;
    match res {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        }
    }
}
#[tauri::command]
pub async fn delete_categories_by_id(id: i32) -> bool {
    let db = new_connection().await;
    let res = categories::delete_categories_by_id(&db, id).await;
    match res {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        }
    }
}
#[tauri::command]
pub async fn confirm_delete_categories_by_id(id: i32) -> bool {
    let db = new_connection().await;
    let res = categories::confirm_delete_categories_by_id(&db, id).await;
    match res {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        }
    }
}
#[tauri::command]
pub async fn update_categories_by_id(id: i32, name: String) -> bool {
    let db = new_connection().await;
    let res = categories::update_categories_by_id(&db, id, &name).await;
    match res {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        }
    }
}
#[tauri::command]
pub async fn get_categories_by_id(id: i32) -> Option<models::Categories> {
    let db = new_connection().await;
    let res = categories::get_categories_by_id(&db, id).await;
    match res {
        Ok(categories) => categories,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            None
        }
    }
}
#[tauri::command]
pub async fn get_categories_list() -> Option<Vec<models::Categories>> {
    let db = new_connection().await;
    let res = categories::get_categories_list(&db).await;
    match res {
        Ok(categories) => categories,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            None
        }
    }
}
