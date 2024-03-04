// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;

use sea_orm::prelude::DatabaseConnection;
use sea_orm::Database;

const DATABASE_URL: &str = "sqlite::memory:";

pub struct AppState {
  pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
  let db = Database::connect(DATABASE_URL).await.unwrap();

  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .plugin(tauri_plugin_manatsu::init())
    .manage(AppState { db })
    .invoke_handler(tauri::generate_handler![command::version])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
