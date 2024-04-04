// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(async_fn_in_trait)]

mod command;
pub mod database;
mod error;
pub mod prelude;

use sea_orm::DatabaseConnection;
use tauri::Manager;

pub struct AppState {
  pub db: DatabaseConnection,
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .plugin(tauri_plugin_manatsu::init())
    .setup(|app| {
      let handle = app.handle();
      app.manage(AppState {
        db: database::connect(handle).unwrap(),
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![command::hello_manatsu])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
