#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use server::categories_server::*;
use server::note_server::*;
use server::total_server::*;

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
            update_note_by_id,
        ])
        .invoke_handler(tauri::generate_handler![get_total_number])
        .invoke_handler(tauri::generate_handler![
            create_categories,
            delete_categories_by_id,
            confirm_delete_categories_by_id,
            update_categories_by_id,
            get_categories_by_id,
            get_categories_list,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
