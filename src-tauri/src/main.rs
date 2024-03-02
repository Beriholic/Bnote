#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::server::server::*;
mod db;
mod entity;
mod server;
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_note_by_id,
            get_note_list,
            create_note,
            delete_note_by_id,
            confirm_delete_note_by_id,
            update_note_by_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
