use crate::db::funcs::total;

use super::server::new_connection;

#[tauri::command]
pub async fn get_total_number() -> i32 {
    let db = new_connection().await;
    return match total::get_total_number(&db).await {
        Ok(number) => number,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            0
        }
    };
}
