// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(async_fn_in_trait)]

mod command;
pub mod database;
mod error;
pub mod prelude;

use tauri::Manager;

pub struct AppState {
  pub database: sea_orm::DatabaseConnection,
}

pub type State<'a> = tauri::State<'a, AppState>;

#[tokio::main]
async fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .plugin(tauri_plugin_manatsu::init())
    .setup(|app| {
      let state = AppState {
        database: database::connect(&app.config()).unwrap(),
      };

      app.manage(state);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![command::version])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
